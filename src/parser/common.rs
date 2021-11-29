use nom::bytes::complete::{tag, is_not};
use nom::branch::alt;
use nom::character::complete::{alpha1, alphanumeric1, multispace0};
use nom::combinator::{recognize, not, cond, verify};
use nom::{AsChar, error, InputIter, InputLength, IResult, Parser};
use nom::error::ParseError;
use nom::multi::{many0, separated_list0};
use nom::sequence::{delimited, pair, preceded, tuple};
use nom_locate::{LocatedSpan, position};
use crate::ir::expr::{Span, LSpan};

// todo:not a keyword
fn not_keyword(i: &LSpan) -> bool {
    // alt((is_not("var"), is_not("function"), is_not("array"), is_not("in"), is_not("let")))(i)
    let keywords = vec!["var", "function", "array", "let", "in", "end", "nil"];
    !keywords.contains(i.fragment())
}

pub fn identifier(i: LSpan) -> IResult<LSpan, &str> {
    let (s, id) = verify(recognize(
        pair(
            alt((alpha1, tag("_"))),
            many0(alt((alphanumeric1, tag("_")))),
        )
    ), not_keyword)(i)?;
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

pub fn preceded_space0<'a, F: 'a, O, E: ParseError<LSpan<'a>>>(inner: F)
                                                               -> impl FnMut(LSpan<'a>) -> IResult<LSpan<'a>, O, E>
    where
        F: FnMut(LSpan<'a>) -> IResult<LSpan<'a>, O, E>,
{
    preceded(
        multispace0,
        inner,
    )
}

// pub fn around_space0_tag<'a, F: 'a, O, E: ParseError<LSpan<'a>>>(inner: F)
// pub fn around_space0_tag<'a, F: 'a, O, E: ParseError<LSpan<'a>>>(inner: F)
//                                                                 -> impl FnMut(LSpan<'a>) -> IResult<LSpan<'a>, O, E>
//     where
//         F: FnMut(LSpan<'a>) -> IResult<LSpan<'a>, O, E>
// {
//     tuple((multispace0, inner, multispace0))
// }

pub fn delimited_space0<'a, F: 'a, O, E: ParseError<LSpan<'a>>>(inner: F)
                                                                -> impl FnMut(LSpan<'a>) -> IResult<LSpan<'a>, O, E>
    where
        F: FnMut(LSpan<'a>) -> IResult<LSpan<'a>, O, E>
{
    delimited(
        multispace0,
        inner,
        multispace0,
    )
}


pub fn parse_separated_list0<'a, G: 'a, F: 'a, O1, O2, E: ParseError<LSpan<'a>>>(sep: G, inner: F)
                                                                                 -> impl FnMut(LSpan<'a>) -> IResult<LSpan<'a>, Vec<O2>, E>
    where
        G: FnMut(LSpan<'a>) -> IResult<LSpan<'a>, O1, E>,
        F: FnMut(LSpan<'a>) -> IResult<LSpan<'a>, O2, E>,
{
    separated_list0(
        sep,
        delimited_space0(inner),
    )
}

// // todo:finish this and import a macro for test

// todo: test error id
#[cfg(test)]
mod tests {
    use crate::parser::common::{identifier, Span};
    use crate::ir::expr::LSpan;

    fn assert_true_id(i: &str) {
        match identifier(LSpan::new(i)) {
            Ok((l, s)) => {
                assert_eq!(s, i)
            }
            Err(_) => {
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