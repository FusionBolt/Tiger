use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::character::streaming::multispace1;
use nom::IResult;
use nom::sequence::{delimited, preceded, tuple};
use crate::ir::expr::{LSpan, TType, TDec};
use crate::ir::expr::TDec::VarDec;
use crate::parser::common::identifier;
use crate::parser::expr::parse_expr;


pub fn parse_var(i: LSpan) -> IResult<LSpan, TDec> {
    let (i, (id, (type_id), expr)) = tuple((
        // id
        preceded(tuple((tag("var"), multispace1)), identifier),
        // [type_id]
        preceded(multispace0,
                 alt((
                     tuple((tag(":="))),
                     delimited(tuple((tag(":"), multispace1)),
                                      identifier,
                                      tuple((multispace1, tag(":="))))
                 ))),
        // expr
        preceded(multispace0, parse_expr)))(i)?;
        // preceded(tuple((multispace0, tag(":="), multispace0)), parse_expr)))(i)?;
    Ok((i, TDec::VarDec()))
}

pub fn parse_fun(i: LSpan) -> IResult<LSpan, TDec> {
    Ok((i, TDec::VarDec()))
}

#[cfg(test)]
mod tests {
    use crate::ir::expr::LSpan;
    use crate::parser::var::parse_var;

    fn assert_var(i: &str) {
        match parse_var(LSpan::new(i)) {
            Ok((l, _)) => {
                assert!(true)
            }
            Err(_) => {
                assert!(false)
            }
        }
    }
    #[test]
    fn test_parse_var() {
        assert_var("var a := 1");
        assert_var("var a:int := 1");
    }
}