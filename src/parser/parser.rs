use crate::ir::expr;
use nom::{IResult, bytes::complete::{tag, take_while_m_n}, combinator::map_res, character::complete::char, sequence::tuple, Or};
use nom::sequence::{delimited, preceded, pair};
use nom::bytes::complete::is_not;
use nom::branch::alt;
use nom::combinator::{opt, recognize};
use nom::character::complete::{alpha0, alpha1, alphanumeric1, anychar, multispace0, space0};
use crate::ir::expr::{TDec, LSpan, TModule, TSourceBlock, TExpr};
use crate::parser::decs::{parse_dec, parse_block_dec};
use crate::parser::expr::{parse_expr, parse_block_expr};
use nom::multi::many0;
use nom::error::context;

// todo:nested
fn parse_comment(i: LSpan) -> IResult<LSpan, TSourceBlock> {
    let (i, comment) = preceded(multispace0, delimited(tag("/*"), is_not("*/"), tag("*/")))(i)?;
    Ok((i, TSourceBlock::Comment(comment.fragment().to_string())))
}

fn parse_module(i: LSpan) -> IResult<LSpan, TModule> {
    let (i, blocks) = context("parse_module",
                              // todo:process when end, this is nom bug or my bug?
                              many0(alt((parse_comment, parse_block_dec, parse_block_expr))))(i)?;
    let mut decs: Vec<TDec> = vec![];
    let mut exprs: Vec<Box<TExpr>> = vec![];
    blocks.into_iter().for_each(|block|{
       match block {
           TSourceBlock::Dec(dec) => {
                decs.push(dec)
           }
           TSourceBlock::Expr(expr) => {
               exprs.push(expr)
           }
           TSourceBlock::Comment(_) => {

           }
       }
    });
    Ok((i, TModule { decs, exprs }))
}

pub fn parse_source(i: &str) -> Result<TModule, String> {
    parse_module(LSpan::new(i))
        .or_else(|e| Err(format!("parse_source error:{:#?}", e)))
        .and_then(|(_, module)| Ok(module))
}

#[cfg(test)]
mod tests {
    use crate::parser::parser::parse_comment;
    use crate::ir::expr::{LSpan, TSourceBlock};

    fn assert_comment(i: &str, o: &str) {
        match parse_comment(LSpan::new(i)) {
            Ok((l, TSourceBlock::Comment(res))) => {
                assert_eq!(res, o)
            }
            _ => {
                assert!(false)
            }
        }
    }

    #[test]
    fn test_comment() {
        assert_comment("/*this*/", "this");
        assert_comment("/*this*/", "this");
        assert_comment("/*this is comment*/", "this is comment");
        assert_comment("/*this is \r escape comment*/", "this is \r escape comment");
    }
}