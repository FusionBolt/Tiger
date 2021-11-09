use nom_locate::LocatedSpan;

type TSymbol = String;
type TPos = i64;

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum TVar {
    SimpleVar(TSymbol, Span),
    FieldVar(Box<TVar>, TSymbol, Span),
    SubscriptVar(Box<TVar>, Box<TExpr>, Span),
}
#[derive(Debug, PartialEq)]
pub enum OpType {
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
pub struct TField {
    name: TSymbol,
    ty: Box<TExpr>,
}

#[derive(Debug, PartialEq)]
struct TFor {
    var: TSymbol,
    low: Box<TExpr>,
    high: Box<TExpr>,
    body: Box<TExpr>,
    escape: bool,
    pos: Span
}

#[derive(Debug, PartialEq)]
struct TFunDec {
    pos: TPos,
    name: TSymbol,
    params: Vec<TField>
}

#[derive(Debug, PartialEq)]
pub struct TNameType {
    pub name: TSymbol,
    pub ty: TType
}

#[derive(Debug, PartialEq)]
struct TVarDec {
    var: TSymbol,
    ty: TSymbol,
    init: Box<TExpr>,
    escape: bool
}

// todo: many pos
#[derive(Debug, PartialEq)]
pub enum TDec {
    VarDec(),
    FunDec(Vec<TFunDec>),
    TypeDec(Vec<TNameType>),
}

#[derive(Debug, PartialEq)]
pub enum TType {
    NameType(TSymbol),
    // name, ty
    RecordType(Vec<(TSymbol, TSymbol)>),
    ArrayType(TSymbol),
}

#[derive(Debug, PartialEq)]
pub enum TExpr {
    Var(TVar),
    Int(i64),
    String(String, Span),
    Call { fun: TSymbol, args: Vec<Box<TExpr>>, pos: Span },
    Op { op_type: OpType, left: Box<TExpr>, right: Box<TExpr>, pos: Span },
    Record { r_type: TSymbol, fields: Vec<TField>, pos: Span },
    Seq(Vec<(Box<TExpr>, Span)>),
    Assign { var: TVar, expr: Box<TExpr>, pos: Span },
    If { cond: Box<TExpr>, if_expr: Box<TExpr>, else_expr: Box<TExpr>, pos: Span },
    While{cond: Box<TExpr>, body: Box<TExpr>, pos: Span },
    For(TFor),
    Let { decs: Vec<TDec>, body: Box<TExpr>, pos: Span },
    Array {item_type: TSymbol, size: Box<TExpr>, init: Box<TExpr>, pos: Span },
    Nil,
}