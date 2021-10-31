use nom::bytes::complete::tag;
use nom::branch::alt;
use nom::character::complete::multispace0;
use nom::IResult;
use nom::sequence::{delimited, preceded, tuple};
use crate::ir::expr::{TDec, TNameType, TType};
use crate::parser::common::identifier;
use nom::multi::{many0, many_m_n, separated_list0};
use crate::parser::ty::parse_type;
use nom::error::context;


// tydec -> type type-id = ty
fn parse_type_dec(i: &str) -> IResult<&str, TDec> {
    let (i, (_, _, type_id, _, _, _, type_info, _)) =
        delimited(multispace0, tuple((tag("type"), multispace0, identifier, multispace0, tag("="), multispace0, parse_type, multispace0)), multispace0)(i)?;
    Ok((i, TDec::TypeDec(vec![TNameType { name: type_id.to_string(), ty: type_info }])))
}

fn parse_var_dec(i: &str) -> IResult<&str, TDec> {
    let (i, id) = delimited(multispace0, preceded(tuple((tag("var"), multispace0)), identifier), multispace0)(i)?;
    Ok((i, TDec::VarDec()))
}

fn parse_fun_dec(i: &str) -> IResult<&str, TDec> {
    //let (i, (_, _, _)) = delimited(multispace0, tuple((tag("function"), multispace0, identifier)), multispace0)(i)?;
    Ok((i, TDec::VarDec()))
}

fn parse_decs(i: &str) -> IResult<&str, TDec> {
    context("parse_decs" ,alt((parse_type_dec, parse_var_dec, parse_fun_dec)))(i)
}

#[cfg(test)]
mod tests {
    use crate::ir::expr::{TDec, TNameType, TType};
    use crate::parser::decs::parse_type_dec;

    #[test]
    fn test_parse_type_dec() {
        assert_eq!(parse_type_dec("type a = int"), Ok(("", TDec::TypeDec(vec![TNameType {
            name: "a".to_string(),
            ty: TType::NameType("int".to_string()),
        }]))));
    }
}