#[derive(Clone,Debug,PartialEq)]
pub enum Bytecode {
    // Declarations
    Assert,
    Fun(usize),
    RecFun(usize),
    // Logical
    Not,
    And,
    Or,
    Implies,
    HasType,
    // Comparators
    Eq,
    Neq,
    Lt,
    LtEq,
    Gt,
    GtEq,
    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    // Other
    If,
    Invoke(usize,usize),
    // Literals
    Int(usize),
    Bool(bool),
    Var(usize),
    // Type Tests
    IsUint,
    IsBool
}
