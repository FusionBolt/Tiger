type TSymbol = String;
type TPos = i64;

enum TVar {
    SimpleVar(TSymbol),
    FieldVar(Box<TVar>, TSymbol),
    SubscriptVar(Box<TVar>, Box<TExpr>)
}

enum OpType {
    Plus,
    Minus,
    Times,
    Divide,
    Eq,
    Neq,
    Lt,
    Le,
    Gt,
    Ge
}

struct TOp {
    op_type: OpType,
    left: Box<TExpr>,
    right: Box<TExpr>
}

struct TField {
    name: TSymbol,
    expr: Box<TExpr>
}

struct TRecord {
    r_type: TSymbol,
    fields: Vec<TField>,
}

enum TExpr {
    Var,
    Int(i64),
    String(String),
    Call(TSymbol, Vec<Box<TExpr>>),
    Op(TOp),
    Record(),
    Seq(Vec<Box<TExpr>>),
    Assign(),
    If,
    While,
    For,
    Let(),
    // type, size, init
    Array(TSymbol, Box<TExpr>, Box<TExpr>),
    Nil
}