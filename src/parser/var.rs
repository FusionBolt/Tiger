use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::character::streaming::multispace1;
use nom::IResult;
use nom::sequence::{delimited, preceded, tuple};
use crate::ir::expr::{LSpan, TType, TDec, Span, TFunDec, TVarDec, get_position};
use crate::ir::expr::TDec::{VarDec, FunDec};
use crate::parser::common::identifier;
use crate::parser::expr::parse_expr;
use crate::parser::ty::parse_type_fields;
use nom_locate::position;
use crate::ir::expr::TType::RecordType;


pub fn parse_var(i: LSpan) -> IResult<LSpan, TDec> {
    let (i, (name, expr)) = tuple((
        // id
        preceded(tuple((tag("var"), multispace1)), identifier),
        preceded(tuple((multispace0, tag(":="), multispace0)), parse_expr)))(i)?;
        // // [type_id]
        // preceded(multispace0,
        //          alt((
        //              tuple((tag(":="))),
        //              delimited(tuple((tag(":"), multispace1)),
        //                               identifier,
        //                               tuple((multispace1, tag(":="))))
        //          ))),
        // // expr
        // preceded(multispace0, parse_expr)))(i)?;
    let (i, pos) = get_position(i)?;
    Ok((i, TDec::VarDec(TVarDec{
        name: name.to_string(),
        ty: "auto".to_string(),
        init: Box::from(expr),
        escape: false,
        pos
    })))
}



#[cfg(test)]
mod tests {
    use crate::ir::expr::{LSpan, TVarDec};
    use crate::parser::var::{parse_var};
    use crate::ir::expr::TDec::VarDec;

    fn assert_var(i: &str, nm: &str) {
        match parse_var(LSpan::new(i)) {
            Ok((l, VarDec(TVarDec{ name, ty, init, escape, pos }))) => {
                assert_eq!(name, nm.to_string());
                assert!(true)
            }
            _ => {
                assert!(false)
            }
        }
    }

    #[test]
    fn test_parse_var() {
        assert_var("var a := 1", "a");
        // assert_var("var a:int := 1");
    }
}