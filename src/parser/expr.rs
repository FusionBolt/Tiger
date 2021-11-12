use nom::bytes::complete::{tag, take_while};
use nom::branch::alt;
use nom::character::complete::{multispace0, one_of};
use nom::{IResult, Parser};
use nom::combinator::opt;
use nom::multi::{separated_list0, many1, many0, separated_list1};
use nom::sequence::{delimited, preceded, separated_pair, tuple};
use nom_locate::position;
use crate::ir::expr::{get_position, LSpan, Span, TExpr, TVar};
use crate::parser::common::identifier;
use nom::combinator::{recognize, map, map_res};
use nom::character::{is_digit, is_alphanumeric};

pub fn parse_expr(i: LSpan) -> IResult<LSpan, TExpr> {
    Ok((i, TExpr::Nil))
}

// fn parse_exp(i: LSpan) -> IResult<LSpan, TExpr> {
//     alt((parse_lvalue, parse_nil))(i)
// }

// lvalue -> id
//        -> lvalue . id
//        -> lvalue [ exp ]
// fn parse_lvalue(i: LSpan) -> IResult<LSpan, TExpr> {
//     alt((parse_identifier))(i)
// }

fn parse_identifier(i: LSpan) -> IResult<LSpan, TExpr> {
    let (i, id) = preceded(multispace0, identifier)(i)?;
    let (i, pos) = get_position(i)?;
    Ok((i, TExpr::Var(TVar::SimpleVar(id.to_string(), pos))))
}

// fn parse_no_value_expr(i: LSpan) -> IResult<LSpan, TExpr> {
//
// }

fn parse_nil(i: LSpan) -> IResult<LSpan, TExpr> {
    let (i, _) = preceded(multispace0, tag("nil"))(i)?;
    Ok((i, TExpr::Nil))
}

// todo: get separated_list position
fn parse_sequence(i: LSpan) -> IResult<LSpan, TExpr> {
    let (i, exprs) = separated_list1(tag(";"), parse_expr)(i)?;
    let (i, pos) = get_position(i)?;
    Ok((i, TExpr::Seq(exprs.into_iter().map(|expr| (Box::from(expr), pos.clone())).collect())))
}

fn parse_no_value(i: LSpan) -> IResult<LSpan, TExpr> {
    let (i, _) = tuple((preceded(multispace0, tag("(")),
    preceded(multispace0, tag(")"))))(i)?;
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
    let parse_expr_list = separated_list0(
        tag(","),
        delimited(
            multispace0,
            parse_expr,
            multispace0)
    );
    let (i, (id, expr_list)) = tuple(
        (delimited(multispace0, identifier, multispace0),
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
        preceded(multispace0, parse_expr),
        preceded(multispace0, parse_binary_op),
        preceded(multispace0, parse_expr)))(i)?;
    Ok((i, TExpr::Nil))
}

// todo: ( ) in expr
fn parse_unary_expr(i: LSpan) -> IResult<LSpan, TExpr> {
    let parse_unary_op = one_of("-~&|");
    let (i, (op, item)) = tuple((
        preceded(multispace0, parse_unary_op),
        preceded(multispace0, parse_expr)))(i)?;
    Ok((i, TExpr::Nil))
}

// typeid multispace0 { id = exp, id = exp}
// fn parse_record_create(i: LSpan) -> IResult<LSpan, TExpr> {
//     tuple((identifier,))
// }
// fn parse_call(i: LSpan) -> IResult<LSpan, TExpr> {
//
// }

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