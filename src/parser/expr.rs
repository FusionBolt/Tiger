use nom::bytes::complete::{tag, take_while};
use nom::branch::alt;
use nom::character::complete::{multispace0, one_of};
use nom::{IResult, Parser};
use nom::combinator::opt;
use nom::multi::{separated_list0, many1, many0, separated_list1};
use nom::sequence::{delimited, preceded, separated_pair, tuple};
use nom_locate::position;
use crate::ir::expr::{get_position, LSpan, Span, TExpr, TVar, TFor};
use crate::parser::common::{identifier, delimited_space0, preceded_space0, parse_separated_list0};
use nom::combinator::{recognize, map, map_res};
use nom::character::{is_digit, is_alphanumeric};
use crate::parser::ty::parse_type_id;
use crate::ir::expr::TExpr::Var;
use crate::ir::expr::TVar::SimpleVar;

pub fn parse_expr(i: LSpan) -> IResult<LSpan, TExpr> {
    Ok((i, TExpr::Nil))
}

// fn parse_exp(i: LSpan) -> IResult<LSpan, TExpr> {
//     alt((parse_lvalue, parse_nil))(i)
// }

// lvalue -> id
//        -> lvalue . id
//        -> lvalue [ exp ]
fn parse_lvalue(i: LSpan) -> IResult<LSpan, TExpr> {
    // alt((parse_identifier))(i)
    Ok((i, TExpr::Nil))
}

fn parse_identifier(i: LSpan) -> IResult<LSpan, TExpr> {
    let (i, id) = preceded_space0(identifier)(i)?;
    let (i, pos) = get_position(i)?;
    Ok((i, TExpr::Var(TVar::SimpleVar(id.to_string(), pos))))
}

// fn parse_no_value_expr(i: LSpan) -> IResult<LSpan, TExpr> {
//
// }

fn parse_nil(i: LSpan) -> IResult<LSpan, TExpr> {
    let (i, _) = preceded_space0(tag("nil"))(i)?;
    Ok((i, TExpr::Nil))
}

// todo: get separated_list position
fn parse_sequence(i: LSpan) -> IResult<LSpan, TExpr> {
    let (i, exprs) = separated_list1(tag(";"), parse_expr)(i)?;
    let (i, pos) = get_position(i)?;
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
    Ok((i, TExpr::Int(num.to_string().parse::<i64>().unwrap())))
}

fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(input, 10)
}

fn is_character(c: char) -> bool {
    return is_alphanumeric(c as u8);
}

fn parse_string(i: LSpan) -> IResult<LSpan, TExpr> {
    let parse_str = recognize(many0(one_of("asdfghjkk")));
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

// arithmetic operation, compare, bool
fn parse_binary_expr(i: LSpan) -> IResult<LSpan, TExpr> {
    let parse_binary_op = one_of("x-*/");
    let (i, (lhs, op, rhs)) = tuple((
        preceded_space0(parse_expr),
        preceded_space0(parse_binary_op),
        preceded_space0(parse_expr)))(i)?;
    Ok((i, TExpr::Nil))
}

// todo: ( ) in expr
fn parse_unary_expr(i: LSpan) -> IResult<LSpan, TExpr> {
    let parse_unary_op = one_of("-~&|");
    let (i, (op, item)) = tuple((
        preceded_space0(parse_unary_op),
        preceded_space0(parse_expr)))(i)?;
    Ok((i, TExpr::Nil))
}

// typeid multispace0 { id = exp, id = exp}
fn parse_create_record_var(i: LSpan) -> IResult<LSpan, TExpr> {
    let parse_record_field = tuple((
        parse_type_id, preceded(delimited_space0(tag("=")), parse_expr)
    ));
    let (i, (field_id, expr)) = tuple((
        identifier,
        delimited(tag("{"),
                  parse_separated_list0(tag(","), parse_record_field),
                  tag("}"))
    ))(i)?;
    let (i, pos) = get_position(i)?;
    // todo:what should return?
    Ok((i, TExpr::Var(TVar::FieldVar(
        Box::new(TVar::SimpleVar(field_id.to_string(), pos.clone())),
        field_id.to_string(),
        pos))))
}

fn parse_create_array(i: LSpan) -> IResult<LSpan, TExpr> {
    let (i, (type_id, n, v)) = tuple((parse_type_id,
           opt(delimited_space0(parse_expr)),
           preceded(delimited_space0(tag("of")), parse_expr)))(i)?;
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
    let (i, (lvalue, expr)) = tuple((
        parse_lvalue,
        preceded(delimited_space0(tag(":=")), parse_expr)
    ))(i)?;
    let (i, pos) = get_position(i)?;
    // todo: lvalue not return true value
    Ok((i, TExpr::Assign {
        var: TVar::SimpleVar("UnImplError".to_string(), pos.clone()),
        expr: Box::new(expr),
        pos
    }))
}

// todo:what should return, TExpr or if??
fn parse_if(i: LSpan) -> IResult<LSpan, TExpr> {
    let (i, (cond, if_expr, else_expr)) = tuple((
        preceded(tag("if"), delimited_space0(parse_expr)),
        preceded(tag("then"), delimited_space0(parse_expr)),
        opt(preceded(tag("else"), delimited_space0(parse_expr))),
    ))(i)?;
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

fn parse_while(i: LSpan) -> IResult<LSpan, TExpr> {
    let (i, (cond, body)) = tuple((
        preceded(tag("while"), delimited_space0(parse_expr)),
        preceded(tag("do"), delimited_space0(parse_expr))
    ))(i)?;
    let (i, pos) = get_position(i)?;
    Ok((i, TExpr::While {
        cond: Box::new(cond),
        body: Box::new(body),
        pos
    }))
}

fn parse_for(i: LSpan) -> IResult<LSpan, TExpr> {
    let parse_for_update_stmt = tuple((identifier, preceded(delimited_space0(tag(":=")), parse_expr)));
    let (i, ((var, low_expr), high_expr, body)) = tuple((
        preceded(tag("for"), delimited_space0(parse_for_update_stmt)),
        preceded(tag("to"), delimited_space0(parse_expr)),
        preceded(tag("do"), delimited_space0(parse_expr)),
    ))(i)?;
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
    tag("break")?
}

// todo:refactor, after tag should space1
// todo:replace tag with cut
fn parse_let(i: LSpan) -> IResult<LSpan, TExpr> {
    let (i, ((var, low_expr), high_expr, body)) = tuple((
        preceded(tag("let"), delimited_space0(parse_for_update_stmt)),
        preceded(tag("in"), delimited_space0(parse_expr)),
        tag("end")
    ))(i)?;
}
// lvalue . id
// fn parse_record_field_access(i: LSpan) -> IResult<LSpan, TExpr> {
//     let (i, (lv, id)) = tuple((parse_lvalue, preceded(tuple((multispace0, tag("."), multispace0)),
//                                   identifier)))(i)?;
//     Ok((i, ))
// }

// fn parse_array_index(i: LSpan) -> IResult<LSpan, TExpr> {
//     let (i (lv, id)) = tuple((parse_lvalue, delimited(
//         tuple((multispace0, tag("["), multispace0)),
//         identifier,
//         tuple((multispace0, tag("]"), multispace0)))))(i)?;
//
// }

#[cfg(test)]
mod tests {
    use crate::ir::expr::{LSpan, TExpr};
    use crate::parser::expr::{parse_nil, parse_number, parse_call};

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

    #[test]
    fn test_parse_call() {
        // todo:not test
        parse_call(LSpan::new("a()"));
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
    fn test_num() {
        assert_num("233", 233);
    }
}