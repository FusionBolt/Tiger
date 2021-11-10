use nom::bytes::complete::tag;
use nom::character::complete::{multispace0, multispace1};
use nom::IResult;
use nom::sequence::{delimited, preceded, tuple};
use nom_locate::position;
use crate::ir::expr::{get_position, LSpan, Span, TDec, TFunDec};
use crate::parser::common::identifier;
use crate::parser::expr::parse_expr;
use crate::parser::ty::parse_type_fields;

// todo:maybe should return fun dec??
pub fn parse_fun(i: LSpan) -> IResult<LSpan, TDec> {
    let (i, (name, fields, body)) = tuple((
        // id
        preceded(tuple((multispace0, tag("function"), multispace1)), identifier),
        // param
        delimited(tag("("), parse_type_fields, tag(")")),
        // expr
        preceded(tuple((multispace0, tag("="), multispace0)), parse_expr)))(i)?;
    // ty fields
    let (i, pos) = get_position(i)?;
    Ok((i, TDec::FunDec(TFunDec { name: name.to_string(), params: fields, body: Box::from(body), pos })))
}

#[cfg(test)]
mod tests {
    use crate::ir::expr::LSpan;
    use crate::parser::fun::parse_fun;

    fn assert_fun(i: &str) {
        match parse_fun(LSpan::new(i)) {
            Ok((l, _)) => {
                assert!(true)
            }
            Err(_) => {
                assert!(false)
            }
        }
    }

    #[test]
    fn test_parse_fun() {
        assert_fun("function treeLeaves(t : tree) = 1");
    }
}