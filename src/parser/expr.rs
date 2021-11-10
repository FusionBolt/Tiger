use nom::bytes::complete::tag;
use nom::branch::alt;
use nom::character::complete::multispace0;
use nom::{IResult, Parser};
use nom::multi::separated_list1;
use nom::sequence::{delimited, preceded, tuple};
use nom_locate::position;
use crate::ir::expr::{get_position, LSpan, Span, TExpr, TVar};
use crate::parser::common::identifier;

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

}

fn parse_string(i: LSpan) -> IResult<LSpan, TExpr> {

}

fn parse_call(i: LSpan) -> IResult<LSpan, TExpr> {

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
    use crate::ir::expr::LSpan;
    use crate::parser::expr::parse_nil;

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
    fn test_nil() {
        assert_nil("nil");
        assert_nil(" nil");
        // todo: assert NIL is false
    }
}