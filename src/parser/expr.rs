use nom::bytes::complete::{tag, take_while, is_not, take_till};
use nom::branch::alt;
use nom::character::complete::{multispace0, one_of, satisfy, char, alphanumeric1};
use nom::{IResult, Parser, Needed, Err};
use nom::combinator::{opt, cut, not};
use nom::multi::{separated_list0, many1, many0, separated_list1, many_till};
use nom::sequence::{delimited, preceded, separated_pair, tuple, terminated};
use nom_locate::position;
use crate::ir::expr::{get_position, LSpan, Span, TExpr, TVar, TFor, BinaryOpCode, UnaryOpCode, TSourceBlock};
use crate::parser::common::{identifier, delimited_space0, preceded_space0, parse_separated_list0};
use nom::combinator::{recognize, map, map_res};
use nom::character::{is_digit, is_alphanumeric};
use crate::parser::ty::parse_type_id;
use crate::ir::expr::TExpr::Var;
use crate::ir::expr::TVar::SimpleVar;
use nom::error::{context, ErrorKind};
use crate::parser::decs::parse_decs;
use crate::parser::fun::parse_fun;

pub fn parse_block_expr(i: LSpan) -> IResult<LSpan, TSourceBlock> {
    let (i, expr) = parse_expr(i)?;
    println!("block expr end {:?}", i);
    Ok((i, TSourceBlock::Expr(Box::new(expr))))
}

// todo:reduce this code copy
fn parse_expr_begin_with_id(i: LSpan) -> IResult<LSpan, TExpr> {
    let (new_i, id) = delimited_space0(parse_expr_identifier)(i)?;
    let cur_char = match new_i.chars().nth(0){
        Some(c) => c,
        None => return Ok((new_i, id))
    };
    // todo: process }, ) not matched
    if cur_char == '{' {
        // id{ id }
        return parse_create_record_var(i);
    } else if cur_char == '[' {
        // return parse_create_array(i)
        let (array_right, _) = terminated(tuple((take_till(|x| x == ']'), tag("]"))), multispace0)(i)?;
        // todo:return ] is not matched
        // .map(|(ls, _)| Ok((ls, TExpr::Nil)))
        //     .unwrap_or(Err(nom::Err::Error(nom::error::Error::new("] is not matched", nom::error::ErrorKind::Tag))))
        //     .expect("");
        // .or_else(|e|
        //     Err(nom::Err::Error(nom::error::Error::new("] is not matched", nom::error::ErrorKind::Tag))))?;
        let res = tag::<&str, LSpan, nom::error::Error<LSpan>>("of")(array_right);
        match res {
            Ok(_) => return parse_create_array(i),
            _ => return parse_array_index(i)
        }
    } else if cur_char == '(' {
        // id(expr...)
        return parse_call(i)
    } else if cur_char == '.' {
        return parse_record_field_access(i)
    }
    Ok((new_i, id))
}

fn parse_term(i: LSpan) -> IResult<LSpan, TExpr> {
    // record / array maybe start at term
    // alt((parse_literal_expr, parse_call, parse_record_field_access, parse_array_index, parse_expr_identifier))(i)
    alt((parse_literal_expr, parse_expr_begin_with_id))(i)
}

fn parse_op(i: LSpan) -> IResult<LSpan, BinaryOpCode> {
    let (i, c) = alt((char('+'), char('-'), char('*'), char('/')))(i)?;
    Ok((i, match c {
        '+' => BinaryOpCode::Plus,
        '-' => BinaryOpCode::Minus,
        '*' => BinaryOpCode::Times,
        '/' => BinaryOpCode::Divide,
        _ => todo!("un support op code")
    }))
}

fn parse_normal_expr(i: LSpan) -> IResult<LSpan, TExpr> {
    // todo:unary?
    let (i, (term, expr)) = preceded_space0(
        tuple((parse_term,
               many0(
                   tuple((preceded_space0(parse_op), preceded_space0(parse_term)))))))(i)?;
    let (i, pos) = get_position(i)?;
    Ok((i, expr.into_iter().fold(term, |sum, (op, rhs)|{
        TExpr::BinaryOp {
            op_type: op,
            left: Box::new(sum),
            right: Box::new(rhs),
            pos: pos.clone()
        }
    })))
}

pub fn parse_expr(i: LSpan) -> IResult<LSpan, TExpr> {
    // let, if, while, for, break
    delimited_space0(alt((parse_let, parse_if, parse_while, parse_for, parse_break, parse_normal_expr)))(i)
}

fn parse_literal_expr(i: LSpan) -> IResult<LSpan, TExpr> {
    alt((parse_nil, parse_number, parse_string))(i)
}
// fn parse_exp(i: LSpan) -> IResult<LSpan, TExpr> {
//     alt((parse_lvalue, parse_nil))(i)
// }

// lvalue -> id
//        -> lvalue . id
//        -> lvalue [ exp ]
fn parse_lvalue(i: LSpan) -> IResult<LSpan, TExpr> {
    let (new_i, id) = delimited_space0(parse_expr_identifier)(i)?;
    let cur_char = match new_i.chars().nth(0){
        Some(c) => c,
        None => return Ok((new_i, id))
    };
    if cur_char == '[' {
        parse_array_index(i)
    } else if cur_char == '.' {
        parse_record_field_access(i)
    } else {
        Ok((new_i, id))
    }
}

// todo: a.b.c.d.e
// lvalue . id
fn parse_record_field_access(i: LSpan) -> IResult<LSpan, TExpr> {
    let (i, (lv, id)) = tuple((parse_expr_identifier, preceded(tuple((multispace0, tag("."), multispace0)),
                                  alt((parse_call, parse_expr_identifier)))))(i)?;
    match id {
        TExpr::Call{ fun, mut args, pos } => {
            args.insert(0, Box::from(lv));
            Ok((i, TExpr::Call { fun, args, pos }))
        }
        TExpr::Var(v) => {
            Ok((i, TExpr::Var(v)))
        }
        _ => {
            // todo:myself error process
            Err(nom::Err::Error(nom::error::Error::new(i, nom::error::ErrorKind::Tag)))
        }
    }
}

// todo: a[1][2][3]
fn parse_array_index(i: LSpan) -> IResult<LSpan, TExpr> {
    let (i, (lv, id)) = tuple((parse_expr_identifier, delimited(
        tuple((multispace0, tag("["), multispace0)),
        parse_expr,
        tuple((multispace0, tag("]"), multispace0)))))(i)?;
    // todo:return value call array index
    Ok((i, TExpr::Nil))
}

fn parse_expr_identifier(i: LSpan) -> IResult<LSpan, TExpr> {
    let (i, id) = preceded_space0(identifier)(i)?;
    let (i, pos) = get_position(i)?;
    Ok((i, TExpr::Var(TVar::SimpleVar(id.to_string(), pos))))
}

fn parse_nil(i: LSpan) -> IResult<LSpan, TExpr> {
    let (i, _) = preceded_space0(tag("nil"))(i)?;
    Ok((i, TExpr::Nil))
}

// todo: get separated_list position
fn parse_expr_sequence(i: LSpan) -> IResult<LSpan, TExpr> {
    println!("parse_expr_sequence");
    let (i, exprs) = separated_list1(tag(";"), parse_expr)(i)?;
    let (i, pos) = get_position(i)?;
    println!("parse_expr_sequence ok:{:?}", i);
    Ok((i, TExpr::Seq(exprs.into_iter().map(|expr| (Box::from(expr), pos.clone())).collect())))
}

fn parse_no_value(i: LSpan) -> IResult<LSpan, TExpr> {
    let (i, _) = tuple((preceded_space0(tag("(")),
    preceded_space0(tag(")"))))(i)?;
    Ok((i, TExpr::Nil))
}

// todo: neg num
fn parse_number(i: LSpan) -> IResult<LSpan, TExpr> {
    let (i, num) = recognize(many1(one_of("1234567890")))(i)?;
    let (i, pos) = get_position(i)?;
    println!("parse number: {:?}, fragment: {:?}", num, i);
    Ok((i, TExpr::Int(num.to_string().parse::<i64>().unwrap())))
}

fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(input, 10)
}

fn is_character(c: char) -> bool {
    return is_alphanumeric(c as u8);
}

fn parse_string(i: LSpan) -> IResult<LSpan, TExpr> {
    // todo:refactor, many character
    let parse_str = recognize(many0(one_of(" qwertyuiopasdfghjklzxcvbnm")));
    let (i, str) = delimited(tag("\""), parse_str,tag("\""))(i)?;
    let (i, pos) = get_position(i)?;
    Ok((i, TExpr::String(str.to_string(), pos)))
}

fn parse_call(i: LSpan) -> IResult<LSpan, TExpr> {
    let parse_expr_list = parse_separated_list0(
        tag(","),
        parse_expr
    );
    let (i, (id, expr_list)) = tuple((
        delimited_space0(identifier),
        delimited(tag("("), opt(parse_expr_list),tag(")")))
    )(i)?;
    let (i, pos) = get_position(i)?;
    let expr_list = match expr_list {
        Some(l) => {
            l
        }
        None => {
            vec![]
        }
    };

    let args = expr_list.into_iter().map(|expr| Box::new(expr)).collect();
    Ok((i, TExpr::Call { fun: id.to_string(), args, pos}))
}

fn parse_op_item(i: LSpan) -> IResult<LSpan, TExpr> {
    // todo: binary and unary
    alt((parse_literal_expr, parse_call, parse_lvalue))(i)
}


// arithmetic operation, compare, bool
fn parse_binary_expr(i: LSpan) -> IResult<LSpan, TExpr> {
    let parse_binary_op = one_of("+-*/");
    let (i, (lhs, op, rhs)) = tuple((
        preceded_space0(parse_op_item),
        preceded_space0(parse_binary_op),
        preceded_space0(parse_op_item)))(i)?;
    let (i, pos) = get_position(i)?;
    // todo:error!!
    Ok((i, TExpr::BinaryOp {
        op_type: BinaryOpCode::Plus,
        left: Box::new(lhs),
        right: Box::new(rhs),
        pos
    }))
}

// todo: ( ) in expr
fn parse_unary_expr(i: LSpan) -> IResult<LSpan, TExpr> {
    let parse_unary_op = one_of("-&|");
    let (i, (op, item)) = tuple((
        preceded_space0(parse_unary_op),
        preceded_space0(parse_op_item)))(i)?;
    let (i, pos) = get_position(i)?;
    Ok((i, TExpr::UnaryOp {
        op_type: UnaryOpCode::Neg,
        value: Box::new(item),
        pos
    }))
}

// typeid multispace0 { id = exp, id = exp}
fn parse_create_record_var(i: LSpan) -> IResult<LSpan, TExpr> {
    let parse_record_field = tuple((
        // id space* = space* expr
        // todo:replace with around_space0_tag
        // todo:why next is failed
        parse_type_id, preceded(tuple((multispace0, tag("="), multispace0)), parse_expr)
        // parse_type_id, preceded(delimited_space0(tag("=")), parse_expr)
    ));
    let (i, (field_id, expr)) = tuple((
        identifier,
        delimited_space0(delimited(tag("{"),
                  parse_separated_list0(tag(","), parse_record_field),
                  tag("}")))
    ))(i)?;
    let (i, pos) = get_position(i)?;
    Ok((i, TExpr::Var(TVar::SimpleVar(
        field_id.to_string(),
        pos))))
}

fn parse_create_array(i: LSpan) -> IResult<LSpan, TExpr> {
    let (i, (type_id, n, v)) = preceded_space0(tuple((parse_type_id,
           opt(delimited_space0(delimited(tag("["), parse_expr, tag("]")))),
           preceded(delimited_space0(tag("of")), parse_expr))))(i)?;
    let (i, pos) = get_position(i)?;
    let size = match n {
        Some(n) => n,
        None => TExpr::Int(0)
    };
    Ok((i, TExpr::Array {
        item_type:type_id.to_string(),
        size: Box::new(size),
        init: Box::new(v),
        pos
    }))
}

fn parse_assign(i: LSpan) -> IResult<LSpan, TExpr> {
    let (i, (lvalue, expr)) = preceded_space0(tuple((
        parse_lvalue,
        preceded(delimited_space0(tag(":=")), parse_expr)
    )))(i)?;
    // todo:failed should return error
    let lvar = match lvalue {
        Var(var) => {
            var
        }
        _ => {
            return Err(Err::Incomplete(Needed::new(4)));
        }
    };
    let (i, pos) = get_position(i)?;
    // todo: lvalue not return true value
    Ok((i, TExpr::Assign {
        var: lvar,
        expr: Box::new(expr),
        pos
    }))
}

// todo:what should return, TExpr or if??
fn parse_if(i: LSpan) -> IResult<LSpan, TExpr> {
    let (i, (cond, if_expr, else_expr)) = preceded_space0(tuple((
        preceded(tag("if"), delimited_space0(parse_expr)),
        preceded(tag("then"), delimited_space0(parse_expr)),
        opt(preceded(tag("else"), delimited_space0(parse_expr))),
    )))(i)?;
    let (i, pos) = get_position(i)?;
    // todo: about optional, has more good solution?
    let else_expr = match else_expr {
        Some(else_expr) => else_expr,
        None => TExpr::Nil
    };
    Ok((i, TExpr::If {
        cond: Box::new(cond),
        if_expr: Box::new(if_expr),
        else_expr: Box::new(else_expr),
        pos
    }))
}

// todo:pre space0 tuple
fn parse_while(i: LSpan) -> IResult<LSpan, TExpr> {
    let (i, (cond, body)) = preceded_space0(tuple((
        preceded(tag("while"), delimited_space0(parse_expr)),
        preceded(tag("do"), delimited_space0(parse_expr))
    )))(i)?;
    let (i, pos) = get_position(i)?;
    Ok((i, TExpr::While {
        cond: Box::new(cond),
        body: Box::new(body),
        pos
    }))
}

fn parse_for(i: LSpan) -> IResult<LSpan, TExpr> {
    let parse_for_update_stmt = tuple((identifier, preceded(delimited_space0(tag(":=")), parse_expr)));
    let (i, ((var, low_expr), high_expr, body)) = preceded_space0(tuple((
        preceded(tag("for"), delimited_space0(parse_for_update_stmt)),
        preceded(tag("to"), delimited_space0(parse_expr)),
        preceded(tag("do"), delimited_space0(parse_expr)),
    )))(i)?;
    let (i, pos) = get_position(i)?;
    Ok((i, TExpr::For(TFor{
        var: var.to_string(),
        low: Box::new(low_expr),
        high: Box::new(high_expr),
        body: Box::new(body),
        escape: false,
        pos
    })))
}

fn parse_break(i: LSpan) -> IResult<LSpan, TExpr> {
    let (i, _) = tag("break")(i)?;
    Ok((i, TExpr::Nil))
}

// todo:refactor, after tag should space1
// todo:replace tag with cut
fn parse_let(i: LSpan) -> IResult<LSpan, TExpr> {
    let (i, (decs, body)) = preceded_space0(terminated(tuple((
        preceded(tag("let"), parse_decs),
        preceded_space0(preceded(tag("in"), parse_expr_sequence)))),
                                                           preceded_space0(tag("end"))))(i)?;
    println!("end let :{:?}", i);
    let (i, pos) = get_position(i)?;
    Ok((i, TExpr::Let {
        decs,
        body: Box::new(body),
        pos
    }))
}

// todo:add a bool parser(true or false)
#[cfg(test)]
mod tests {
    use crate::ir::expr::{LSpan, make_simple_var_expr, BinaryOpCode, TExpr, TVar, get_simple_var_name, UnaryOpCode, get_int, TFor};
    use crate::parser::expr::{parse_nil, parse_number, parse_call, parse_string, parse_binary_expr, parse_unary_expr, parse_create_record_var, parse_create_array, parse_assign, parse_if, parse_while, parse_for, parse_let, parse_expr, parse_record_field_access};
    use crate::ir::expr::TExpr::UnaryOp;

    fn assert_nil(i: &str) {
        match parse_nil(LSpan::new(i)) {
            Ok(_) => {
                assert!(true)
            }
            _ => {
                assert!(false)
            }
        }
    }

    #[test]
    fn test_parse_nil() {
        assert_nil("nil");
        assert_nil(" nil");
        // todo: assert NIL is false
    }

    fn assert_call(i: &str, id: &str) {
        match parse_call(LSpan::new(i)) {
            Ok((_, TExpr::Call { fun, args, pos })) => {
                assert_eq!(fun, id.to_string())
            }
            _ => {
                assert!(false)
            }
        }
    }

    #[test]
    fn test_parse_call() {
        assert_call("foo()", "foo");
    }

    fn assert_num(i: &str, num: i64) {
        match parse_number(LSpan::new(i)) {
            Ok((_, TExpr::Int(n))) => {
                assert_eq!(n, num)
            }
            _ => {
                assert!(false)
            }
        }
    }

    #[test]
    fn test_parse_num() {
        assert_num("233", 233);
    }

    fn assert_string(i: &str, string: &str) {
        match parse_string(LSpan::new(i)) {
            Ok((_, TExpr::String(s, pos))) => {
                assert_eq!(s, string)
            }
            def => {
                assert!(false, "should:{} in fact:{:?}", string, def)
            }
        }
    }

    #[test]
    fn test_parse_string() {
        assert_string("\"this is some string\"", "this is some string");
    }

    fn assert_binary_id(i: &str, op: BinaryOpCode, left_v: &str, right_v: &str) {
        match parse_binary_expr(LSpan::new(i)) {
            Ok((_, TExpr::BinaryOp { op_type, left, right, pos })) => {
                assert_eq!(op_type, op);
                assert_eq!(get_simple_var_name(left.as_ref()), left_v);
                assert_eq!(get_simple_var_name(right.as_ref()), right_v);
            }
            res => {
                println!("{:?} {:?}", i, res);
                assert!(false)
            }
        }
    }

    #[test]
    fn test_parse_binary() {
        assert_binary_id("a + b", BinaryOpCode::Plus, "a", "b");
    }

    fn assert_unary(i: &str, op: UnaryOpCode, id: &str) {
        match parse_unary_expr(LSpan::new(i)) {
            Ok((_, TExpr::UnaryOp { op_type, value, pos })) => {
                assert_eq!(op_type, op);
                assert_eq!(get_simple_var_name(value.as_ref()), id);
            }
            res => {
                println!("{:?} {:?}", i, res);
                assert!(false)
            }
        }
    }

    #[test]
    fn test_parse_unary() {
        assert_unary("- foo", UnaryOpCode::Neg, "foo");
    }

    fn assert_create_record_var(i: &str, name: &str) {
        match parse_create_record_var(LSpan::new(i)) {
            Ok((_, TExpr::Var(TVar::SimpleVar(id, pos))))=> {
                assert_eq!(id, name)
            }
            res => {
                println!("{:?} {:?}", i, res);
                assert!(false)
            }
        }
    }

    #[test]
    fn test_parse_create_record_var() {
        assert_create_record_var("foo{ a=5, b =m}", "foo");
    }

    fn assert_create_array(i: &str, arr_type: &str, arr_size: i64, arr_init: i64) {
        match parse_create_array(LSpan::new(i)) {
            Ok((_, TExpr::Array { item_type, size, init, pos })) => {
                assert_eq!(item_type, arr_type);
                assert_eq!(get_int(size.as_ref()), arr_size);
                assert_eq!(get_int(init.as_ref()), arr_init);
            }
            res => {
                println!("{:?} {:?}", i, res);
                assert!(false)
            }
        }
    }

    #[test]
    fn test_parse_create_array() {
        assert_create_array(" int [5] of 1", "int", 5, 1);
    }

    fn assert_assign_string(i: &str, l_var: &str, exp: &str) {
        match parse_assign(LSpan::new(i)) {
            Ok((_, TExpr::Assign { var: TVar::SimpleVar(var, _), expr: box TExpr::String(str, _), pos })) => {
                assert_eq!(var, l_var);
                assert_eq!(str, exp)
                // assert_eq!(get_simple_var_name(var.name), l_var);
                // assert_eq!(, arr_init);
            }
            res => {
                println!("{:?} {:?}", i, res);
                assert!(false)
            }
        }
    }

    #[test]
    fn test_parse_assign() {
        assert_assign_string(" foo := \"str\"", "foo", "str");
    }

    fn assert_if(i: &str, cond_v: &str, if_expr_v: &str, else_expr_v: &str) {
        match parse_if(LSpan::new(i)) {
            Ok((_, TExpr::If { cond, if_expr, else_expr, pos })) => {
                assert_eq!(get_simple_var_name(cond.as_ref()), cond_v);
                assert_eq!(get_simple_var_name(if_expr.as_ref()), if_expr_v);
                assert_eq!(get_simple_var_name(else_expr.as_ref()), else_expr_v);
            }
            res => {
                println!("{:?} {:?}", i, res);
                assert!(false)
            }
        }
    }

    #[test]
    fn test_parse_if() {
        assert_if(" if a then b else c", "a", "b", "c");
    }

    fn assert_while(i: &str, cond_v: &str) {
        match parse_while(LSpan::new(i)) {
            Ok((_, TExpr::While { cond, body, pos })) => {
                assert_eq!(get_simple_var_name(cond.as_ref()), cond_v);
            }
            res => {
                println!("{:?} {:?}", i, res);
                assert!(false)
            }
        }
    }

    #[test]
    fn test_parse_while() {
        // todo:test body
        assert_while("while a do a + b", "a");
    }

    fn assert_for(i: &str, id: &str, low_v: i64, high_v: i64, expr: &str) {
        match parse_for(LSpan::new(i)) {
            Ok((_, TExpr::For(TFor { var, low, high, body, escape, pos }))) => {
                assert_eq!(var, id);
                assert_eq!(get_int(low.as_ref()), low_v);
                assert_eq!(get_int(high.as_ref()), high_v);
                assert_eq!(get_simple_var_name(body.as_ref()), expr);
            }
            res => {
                println!("{:?} {:?}", i, res);
                assert!(false)
            }
        }
    }

    #[test]
    fn test_parse_for() {
        assert_for(" for foo := 1 to 8 do a", "foo", 1, 8, "a");
    }

    fn assert_let(i: &str, id: &str, low_v: i64, high_v: i64, expr: &str) {
        match parse_let(LSpan::new(i)) {
            Ok((_, TExpr::For(TFor { var, low, high, body, escape, pos }))) => {
                assert_eq!(var, id);
                assert_eq!(get_int(low.as_ref()), low_v);
                assert_eq!(get_int(high.as_ref()), high_v);
                assert_eq!(get_simple_var_name(body.as_ref()), expr);
            }
            res => {
                println!("{:?} {:?}", i, res);
                assert!(false)
            }
        }
    }

    #[test]
    fn test_parse_let() {
        assert_for(" for foo := 1 to 8 do a", "foo", 1, 8, "a");
    }

    mod ambiguity {
        use nom::IResult;
        use crate::parser::expr::{parse_expr, parse_record_field_access, parse_array_index};
        use crate::ir::expr::{LSpan, TExpr};

        fn assert(res: IResult<LSpan, TExpr>) {
            match res {
                Ok(v) => println!("{:?}", v),
                res => {
                    println!("{:?}", res);
                    assert!(false);
                }
            }
        }
        #[test]
        fn test_parse_expr_ambiguity() {
            // not stack overflow is pass
            match parse_expr(LSpan::new("a + b")) {
                Ok((i, TExpr::BinaryOp { .. })) => {

                }
                res => {
                    println!("{:?}", res);
                    assert!(false)
                }
            };
            assert(parse_expr(LSpan::new("123")));
            assert(parse_expr(LSpan::new("\"str\"")));
            assert(parse_expr(LSpan::new("a()")));
            assert(parse_expr(LSpan::new("id")));
        }

        // todo:array and member access should begin with expr, but now begin with id
        #[test]
        fn test_parse_array_ambiguity() {
            assert(parse_expr(LSpan::new("int [10] of 0")));
            assert(parse_expr(LSpan::new("a[10]")));
        }

        #[test]
        fn test_member() {
            assert(parse_expr(LSpan::new("a.b")));
            assert(parse_expr(LSpan::new("a.b()")));
            // let res = parse_record_field_access(LSpan::new("a.b.c.d"));
            // println!("{:?}", res);
        }

        #[test]
        fn test_array_index() {
            assert(parse_expr(LSpan::new("a[1][2][3]")));
            assert!(false)
        }

        #[test]
        fn test_nest_array_index_chain() {
            assert(parse_expr(LSpan::new("a[b[2]]")));
        }
    }
}