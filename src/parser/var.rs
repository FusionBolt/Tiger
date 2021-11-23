use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::character::streaming::multispace1;
use nom::IResult;
use nom::sequence::{delimited, preceded, tuple};
use crate::ir::expr::{LSpan, TType, TDec, Span, TFunDec, TVarDec, get_position};
use crate::ir::expr::TDec::{VarDec, FunDec};
use crate::parser::common::{identifier, preceded_space0};
use crate::parser::expr::parse_expr;
use crate::parser::ty::{parse_type_fields, parse_type_id};
use nom_locate::position;
use crate::ir::expr::TType::RecordType;
use nom::combinator::opt;

pub fn parse_var(i: LSpan) -> IResult<LSpan, TDec> {
    let (i, ((name, ty), expr)) = tuple((
        // id
        preceded(tuple((tag("var"), multispace1)),
                 // name_id:type_id
                 tuple((identifier,
                        opt(preceded(tag(":"), parse_type_id))))),
        preceded(tuple((multispace0, tag(":="), multispace0)), parse_expr)))(i)?;
    println!("var dec:{:?}", i.fragment());
    let (i, pos) = get_position(i)?;
    // todo:process default
    let ty = ty.unwrap_or("auto").to_string();
    Ok((i, TDec::VarDec(TVarDec{
        name: name.to_string(),
        ty,
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

    fn assert_var(i: &str, nm: &str, ty: &str) {
        match parse_var(LSpan::new(i)) {
            Ok((l, VarDec(TVarDec{ name, ty, init, escape, pos }))) => {
                assert_eq!(name, nm.to_string());
                assert_eq!(ty, ty.to_string());
                assert!(true)
            }
            _ => {
                assert!(false)
            }
        }
    }

    #[test]
    fn test_parse_var() {
        assert_var("var a := 1", "a", "auto");
        assert_var("var a:int := 1", "a", "int");
    }
}