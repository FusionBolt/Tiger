use nom::bytes::complete::tag;
use nom::branch::alt;
use nom::character::complete::{alpha1, alphanumeric1};
use nom::combinator::recognize;
use nom::IResult;
use nom::multi::many0;
use nom::sequence::pair;
use nom_locate::{LocatedSpan, position};
use crate::ir::expr::{Span, LSpan};

// todo:not a keyword
pub fn identifier(i: LSpan) -> IResult<LSpan, &str> {
    let (s, id) = recognize(
        pair(
            alt((alpha1, tag("_"))),
            many0(alt((alphanumeric1, tag("_"))))
        )
    )(i)?;
    Ok((s, id.fragment()))
    // let (s, id) = recognize(
    //     pair(
    //         alt((alpha1, tag("_"))),
    //         many0(alt((alphanumeric1, tag("_"))))
    //     )
    // )(i)?;
    // let (s, pos) = position(s)?;
    // Ok((s, Id{ id: "", pos: Span::from_located_span(s) }))
}

// todo: test error id
#[cfg(test)]
mod tests {
    use crate::parser::common::{identifier, Span};
    use crate::ir::expr::LSpan;

    fn assert_true_id(i: &str) {
        match identifier(LSpan::new(i)) {
            Ok((l, s)) => {
                assert_eq!(s, i)
            } Err(_) => {
                assert!(false)
            }
        }
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