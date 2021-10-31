use nom::bytes::complete::tag;
use nom::branch::alt;
use nom::character::complete::{multispace0, multispace1};
use nom::IResult;
use nom::sequence::{delimited, preceded, tuple, separated_pair};
use crate::ir::expr::{TDec, TNameType, TType};
use crate::parser::common::identifier;
use nom::multi::{many0, many_m_n, separated_list0};
use nom::error::context;

fn parse_name_type(i: &str) -> IResult<&str, TType> {
    let (i, id) = identifier(i)?;
    Ok((i, TType::NameType(id.to_string())))
}

// type_fields -> null
// type_fields -> id : type_id{, id : type_id}
// todo:test if first is ,
fn parse_type_fields(i: &str) -> IResult<&str, TType> {
    let (i, type_fields) = separated_list0(
        tag(","),
        delimited(
            multispace0,
            tuple((identifier, multispace0, tag(":"), multispace0, identifier)),
            multispace0)
    )(i)?;
    let type_fields = type_fields.into_iter().map(|(new_type_id, _, _, _, type_id)| {
        (new_type_id.to_string(), type_id.to_string())
    }).collect();
    Ok((i, TType::RecordType(type_fields)))
}

fn parse_array_type(i: &str) -> IResult<&str, TType> {
    // todo: add cut
    // todo: test multispace1
    let (i, id) = preceded(tuple((tag("array"), multispace1, tag("of"), multispace1)), identifier)(i)?;
    Ok((i, TType::ArrayType(id.to_string())))
}

fn parse_record_type(i: &str) -> IResult<&str, TType> {
    delimited(tag("{"), parse_type_fields, tag("}"))(i)
}

// todo: how to process space, write or find a parserc for this
// todo:test context
pub fn parse_type(i: &str) -> IResult<&str, TType> {
    context("parse_type",
            alt((parse_name_type, parse_record_type, parse_array_type)))(i)
}

#[cfg(test)]
mod tests {
    use crate::ir::expr::{TDec, TNameType, TType};
    use crate::ir::expr::TType::RecordType;
    use crate::parser::ty::{parse_name_type, parse_record_type, parse_array_type};

    #[test]
    fn test_parse_name_type() {
        assert_eq!(parse_name_type("int"), Ok(("", TType::NameType("int".to_string()))));
        assert_eq!(parse_name_type("string"), Ok(("", TType::NameType("string".to_string()))));
    }

    fn assert_record_type(i: &str, vec: &[(&str, &str)]) {
        assert_eq!(parse_record_type(i), Ok(("", TType::RecordType(
            vec.into_iter().map(|(s1, s2)| (s1.to_string(), s2.to_string())).collect()))))
    }

    #[test]
    fn test_parse_record_type() {
        assert_record_type("{a:int,b:string}", &vec![("a", "int"), ("b", "string")]);
        assert_record_type("{a : int, b : string}", &vec![("a", "int"), ("b", "string")]);
        // todo:add false test
        // assert_record_type("{,a : int, b : string}", &vec![("a", "int"), ("b", "string")]);
    }

    #[test]
    fn test_parse_array_type() {
        assert_eq!(parse_array_type("array of int"), Ok(("", TType::ArrayType("int".to_string()))));
        assert_eq!(parse_array_type("arrayof int"), Ok(("", TType::ArrayType("int".to_string()))));
    }
}