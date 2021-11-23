use nom::error::ParseError;
use nom::{InputIter, InputTake, IResult};
use nom_locate::{LocatedSpan, position};

pub type TSymbol = String;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Span {
    offset: usize,
    line: u32
}

impl Span {
    pub fn from_located_span(span: LSpan) -> Span {
        Span { offset:span.location_offset(), line:span.location_line() }
    }
}

pub type LSpan<'a> = LocatedSpan<&'a str>;

pub fn get_position<'a, E>(i: LSpan<'a>) -> IResult<LSpan<'a>, Span, E>
    where
        E: ParseError<LSpan<'a>> {
    let (i, pos) = position(i)?;
    Ok((i, Span::from_located_span(pos)))
}

#[derive(Debug, PartialEq)]
pub enum TVar {
    SimpleVar(TSymbol, Span),
    FieldVar(Box<TVar>, TSymbol, Span),
    SubscriptVar(Box<TVar>, Box<TExpr>, Span),
}

#[derive(Debug, PartialEq)]
pub enum UnaryOpCode {
    Neg,
    And,
    Or,
}

#[derive(Debug, PartialEq)]
pub enum BinaryOpCode {
    Plus,
    Minus,
    Times,
    Divide,
    Eq,
    Neq,
    Lt,
    Le,
    Gt,
    Ge,
}

#[derive(Debug, PartialEq)]
pub struct TFor {
    pub var: TSymbol,
    pub low: Box<TExpr>,
    pub high: Box<TExpr>,
    pub body: Box<TExpr>,
    pub escape: bool,
    pub pos: Span
}

#[derive(Debug, PartialEq)]
pub struct TField {
    pub name: TSymbol,
    pub ty: TSymbol,
    pub pos: Span
}

#[derive(Debug, PartialEq)]
pub struct TNameType {
    pub name: TSymbol,
    pub ty: TType
}

#[derive(Debug, PartialEq)]
pub struct TVarDec {
    pub name: TSymbol,
    pub ty: TSymbol,
    pub init: Box<TExpr>,
    pub escape: bool,
    pub pos: Span
}

#[derive(Debug, PartialEq)]
pub struct TFunDec {
    pub name: TSymbol,
    pub params: Vec<TField>,
    pub body: Box<TExpr>,
    pub pos: Span
}

// todo: many pos
#[derive(Debug, PartialEq)]
pub enum TDec {
    VarDec(TVarDec),
    FunDec(TFunDec),
    TypeDec(Vec<TNameType>),
}

#[derive(Debug, PartialEq)]
pub enum TType {
    NameType(TSymbol),
    // name, ty
    RecordType(Vec<TField>),
    ArrayType(TSymbol),
}

#[derive(Debug, PartialEq)]
pub enum TExpr {
    Var(TVar),
    Int(i64),
    String(String, Span),
    Call { fun: TSymbol, args: Vec<Box<TExpr>>, pos: Span },
    BinaryOp { op_type: BinaryOpCode, left: Box<TExpr>, right: Box<TExpr>, pos: Span },
    UnaryOp { op_type: UnaryOpCode, value: Box<TExpr>, pos: Span },
    Record { r_type: TSymbol, fields: Vec<TField>, pos: Span },
    Seq(Vec<(Box<TExpr>, Span)>),
    Assign { var: TVar, expr: Box<TExpr>, pos: Span },
    If { cond: Box<TExpr>, if_expr: Box<TExpr>, else_expr: Box<TExpr>, pos: Span },
    While { cond: Box<TExpr>, body: Box<TExpr>, pos: Span },
    For(TFor),
    Let { decs: Vec<TDec>, body: Box<TExpr>, pos: Span },
    Array {item_type: TSymbol, size: Box<TExpr>, init: Box<TExpr>, pos: Span },
    Nil,
}

#[derive(Debug)]
pub struct TModule {
    pub decs: Vec<TDec>,
    pub exprs: Vec<Box<TExpr>>
}

pub enum TSourceBlock {
    Dec(TDec),
    Expr(Box<TExpr>),
    Comment(String)
}

pub fn make_simple_var_expr(i: &str) -> Box<TExpr> {
    Box::from(TExpr::Var(TVar::SimpleVar(i.to_string(), Span::default())))
}

pub fn get_simple_var_name(v: &TExpr) -> TSymbol {
    match v {
        TExpr::Var(v) =>{
            match v {
                TVar::SimpleVar(name, pos) => {
                    name.clone()
                }
                _ => {
                    "error:should be simple var".to_string()
                }
            }
        }
        _ => {
            "error:should be var".to_string()
        }
    }
}

pub fn get_int(v: &TExpr) -> i64 {
    match v {
        TExpr::Int(i) =>{
            i.clone()
        }
        _ => {
            assert!(false, "error:should be int");
            0
        }
    }
}