#[derive(Clone,Debug,PartialEq)]
pub enum Bytecode {
    // Literals
    Push1(u8),
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
    // Control-Flow
    Fail
}
