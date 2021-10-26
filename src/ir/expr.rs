type TSymbol = String;
type TPos = i64;

pub enum TVar {
    SimpleVar(TSymbol),
    FieldVar(Box<TVar>, TSymbol),
    SubscriptVar(Box<TVar>, Box<TExpr>),
}

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

struct TField {
    name: TSymbol,
    expr: Box<TExpr>,
}

struct TRecord {
    r_type: TSymbol,
    fields: Vec<TField>,
}

struct TFor {
    var: TSymbol,
    low: Box<TExpr>,
    high: Box<TExpr>,
    body: Box<TExpr>,
    escape: bool,
}

struct TFunDec {
    pos: TPos,
    name: TSymbol,
    params: Vec<TField>
}

struct TNameType {
    name: TSymbol,
    ty: TType
}

struct TVarDec {
    var: TSymbol,
    ty: TSymbol,
    init: Box<TExpr>,
    escape: bool
}

// todo: many pos
pub enum TDec {
    FunDec(Vec<TFunDec>),
    VarDec(),
    TypeDec(Vec<TNameType>),
}

pub enum TType {
    NameType,
    RecordType,
    ArrayType,
}

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