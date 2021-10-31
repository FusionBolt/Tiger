use nom::bytes::complete::tag;
use nom::branch::alt;
use nom::character::complete::{alpha1, alphanumeric1};
use nom::combinator::recognize;
use nom::IResult;
use nom::multi::many0;
use nom::sequence::pair;
use nom_locate::{LocatedSpan, position};

// todo:not a keyword
pub fn identifier(input: &str) -> IResult<&str, &str> {
    recognize(
        pair(
            alt((alpha1, tag("_"))),
            many0(alt((alphanumeric1, tag("_"))))
        )
    )(input)
}

#[derive(Debug)]
pub struct Id<'a> {
    id: &'a str,
    pos: LocatedSpan<&'a str>
}

type Span<'a> = LocatedSpan<&'a str>;
pub fn identifier_(i: Span) -> IResult<Span, Id> {
    let (s, id) = recognize(
        pair(
            alt((alpha1, tag("_"))),
            many0(alt((alphanumeric1, tag("_"))))
        )
    )(i)?;
    let (s, pos) = position(s)?;
    Ok((s, Id{id: *s, pos }))
}

// todo: test error id
#[cfg(test)]
mod tests {
    use crate::parser::common::{identifier, identifier_, Span};

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

    #[test]
    fn test_id() {
        let output = identifier_(Span::new("str something"));
        println!("{:?}", output.unwrap());
        // assert_eq!(identifier_(Span::new("str")));
    }
}