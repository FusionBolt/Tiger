type TSymbol = String;
type TPos = i64;

#[derive(Debug, PartialEq)]
pub enum TVar {
    SimpleVar(TSymbol),
    FieldVar(Box<TVar>, TSymbol),
    SubscriptVar(Box<TVar>, Box<TExpr>),
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
struct TRecord {
    r_type: TSymbol,
    fields: Vec<TField>,
}

#[derive(Debug, PartialEq)]
struct TFor {
    var: TSymbol,
    low: Box<TExpr>,
    high: Box<TExpr>,
    body: Box<TExpr>,
    escape: bool,
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
    String(String),
    Call { fun: TSymbol, args: Vec<Box<TExpr>> },
    Op { op_type: OpType, left: Box<TExpr>, right: Box<TExpr> },
    Record(TRecord),
    Seq(Vec<Box<TExpr>>),
    Assign(TVar, Box<TExpr>),
    If(Box<TExpr>, Box<TExpr>, Box<TExpr>),
    While{cond: Box<TExpr>, body: Box<TExpr>},
    For(TFor),
    Let(Vec<TDec>, Box<TExpr>),
    Array{item_type: TSymbol, size: Box<TExpr>, init: Box<TExpr>},
    Nil,
}