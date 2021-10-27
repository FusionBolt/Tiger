use nom::bytes::complete::tag;
use nom::branch::alt;
use nom::character::complete::{alpha1, alphanumeric1};
use nom::combinator::recognize;
use nom::IResult;
use nom::multi::many0;
use nom::sequence::pair;

// todo:not a keyword
pub fn identifier(input: &str) -> IResult<&str, &str> {
    recognize(
        pair(
            alt((alpha1, tag("_"))),
            many0(alt((alphanumeric1, tag("_"))))
        )
    )(input)
}

// todo: test error id
#[cfg(test)]
mod tests {
    use crate::parser::common::identifier;

    fn assert_true_id(i: &str) {
        assert_eq!(identifier(i), Ok(("", i)))
    }

    #[test]
    fn test_identifier() {
        assert_true_id("name");
        assert_true_id("name123");
        assert_true_id("name_123");
        assert_true_id("name_123_");
        assert_true_id("_name_123_");
    }
}