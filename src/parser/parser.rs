use crate::ir::expr;
use nom::{IResult, bytes::complete::{tag, take_while_m_n}, combinator::map_res, character::complete::char, sequence::tuple, Or};
use nom::sequence::{delimited, preceded, pair};
use nom::bytes::complete::is_not;
use nom::branch::alt;
use nom::combinator::{opt, recognize};
use nom::character::complete::{alpha0, alpha1, alphanumeric1, anychar, multispace0, space0};
use crate::ir::expr::{TDec, LSpan};

// todo:nested
fn parse_comment(i: LSpan) -> IResult<LSpan, &str> {
    let (i, comment) = preceded(multispace0, delimited(tag("/*"), is_not("*/"), tag("*/")))(i)?;
    Ok((i, comment))
}


fn parse_source(i: LSpan) {
    
}

#[cfg(test)]
mod tests {
    use crate::parser::parser::parse_comment;
    use crate::ir::expr::LSpan;

    #[test]
    fn test_comment() {
        assert_eq!(parse_comment(LSpan::new("/*this*/")), Ok(("", "this")));
        assert_eq!(parse_comment(LSpan::new("/*this is comment*/")), Ok(("", "this is comment")));
        assert_eq!(parse_comment(LSpan::new("/*this is \r escape comment*/")), Ok(("", "this is \r escape comment")));
    }
}