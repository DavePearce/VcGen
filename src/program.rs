use std::str::FromStr;
use crate::{Bytecode};
use crate::parser::Parser;

pub enum BoundedResult<S,T> {
    // Valid result producing item.
    Ok(S),
    // Invalid result producing error.
    Err(T),
    // Out of resource
    OutOfResource
}

// =============================================================================
// Term bytecodes
// =============================================================================

pub struct Program {
    bytecodes: Vec<Bytecode>
}

impl<'a> Program {
    pub fn new(bytecodes: Vec<Bytecode>) -> Self {
	Self{bytecodes}
    }

    pub fn check(&self) -> Vec<BoundedResult<(),()>> {
        todo!()
    }
}

// =============================================================================
// Parser
// =============================================================================

impl FromStr for Program {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
	// Construct parser for the given input
	let parser = Parser::new(input);
	// Parser entire input file
	let bytecodes = parser.parse()?;
	// Construct result
	Ok(Program::new(bytecodes))
    }
}
