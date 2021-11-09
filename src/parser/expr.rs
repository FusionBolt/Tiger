use nom::IResult;
use crate::ir::expr::{LSpan, TExpr};

pub fn parse_expr(i: LSpan) -> IResult<LSpan, TExpr> {
    Ok((i, TExpr::Nil))
}