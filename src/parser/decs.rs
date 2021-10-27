use nom::bytes::complete::tag;
use nom::branch::alt;
use nom::character::complete::space0;
use nom::IResult;
use nom::sequence::{delimited, tuple};
use crate::ir::expr::{TDec, TNameType, TType};
use crate::parser::common::identifier;

fn parse_type(i: &str) -> IResult<&str, TType> {
    let ((i, _)) = tag("int")(i)?;
    Ok((i, TType::NameType))
}

// tydec -> type type-id = ty
fn parse_type_dec(i: &str) -> IResult<&str, TDec> {
    let (i, (_, _, type_id, _, _, _, type_info, _)) = delimited(space0, tuple((tag("type"), space0, identifier, space0, tag("="), space0, parse_type, space0)), space0)(i)?;
    Ok((i, TDec::TypeDec(vec![TNameType{name: type_id.to_string(), ty: type_info}])))
}

fn parse_var_dec(i: &str) -> IResult<&str, TDec> {
    let (i, (_, _, _)) = delimited(space0, tuple((tag("var"), space0, identifier)), space0)(i)?;
    Ok((i, TDec::VarDec()))
}

fn parse_fun_dec(i: &str) -> IResult<&str, TDec> {
    //let (i, (_, _, _)) = delimited(space0, tuple((tag("function"), space0, identifier)), space0)(i)?;
    Ok((i, TDec::VarDec()))
}

fn parse_decs(i: &str) -> IResult<&str, TDec> {
    alt((parse_type_dec, parse_var_dec, parse_fun_dec))(i)
}

#[cfg(test)]
mod tests {
    use crate::ir::expr::{TDec, TNameType, TType};
    use crate::parser::decs::parse_type_dec;

    #[test]
    fn test_type_dec() {
        assert_eq!(parse_type_dec("type a = int"), Ok(("", TDec::TypeDec(vec![TNameType{
            name: "a".to_string(),
            ty: TType::NameType
        }]))));
    }
}