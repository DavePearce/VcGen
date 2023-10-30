use std::marker::PhantomData;
use vcg::{Machine,MachineState,MachineWord,MinimalMachineError,VecState};

use Bytecode::*;

#[test]
fn test_01() {
    let bytecode = vec![
        Push1(0x1),
        Push1(0x2),
        Add,
        Return
    ];

    check(bytecode,Ok(RuntimeOutput::Value(0x3)))
}

#[test]
fn test_02() {
    let bytecode = vec![
        Add
    ];

    check(bytecode,Err(MinimalMachineError::StackUnderflow))
}

fn check(code: Vec<Bytecode>, output: Result<RuntimeOutput,MinimalMachineError>) {
    let svm = StackMachine::<u8>::new(code);
    let init = VecState::<u8>::init();
    let o = svm.execute(init);
    assert_eq!(o,output);
}

// ===================================================================
// Bytecode instruction set
// ===================================================================

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
    Return,
    Fail
}

#[derive(Clone,Copy,Debug,PartialEq)]
pub enum RuntimeOutput {
    Error,
    Value(u8)
}

// ===================================================================
// Stack Machine
// ===================================================================

pub struct StackMachine<T:MachineWord> {
    dummy: PhantomData<T>,
    code: Vec<Bytecode>
}

impl<T:MachineWord> StackMachine<T> {
    pub fn new(code: Vec<Bytecode>) -> Self {
        Self{code,dummy: PhantomData}
    }
}

impl Machine for StackMachine<u8> {
    type Error = MinimalMachineError;
    type State = VecState<u8,Self::Error>;
    type Instruction = Bytecode;
    type Outcome = RuntimeOutput;

    fn get(&self,pc: usize) -> Result<&Self::Instruction,Self::Error> {
        if pc < self.code.len() {
            Ok(&self.code[pc])
        } else {
            Err(Self::Error::InvalidPC)
        }
    }

    fn execute(&self, mut state: Self::State) -> Result<Self::Outcome,Self::Error> {
        // Execute until nothing left.
        loop {
            //
            match self.get(state.pc())? {
                // Literals
                Bytecode::Push1(c) => {
                    state.push(*c)?;
                    state.goto(state.pc()+1);        
                }
                Bytecode::Add => {
                    let r = state.pop()?;
                    let l = state.pop()?;
                    state.push(l+r)?;
                    state.goto(state.pc()+1);                    
                }
                Bytecode::Return => {
                    let v = state.pop()?;
                    return Ok(RuntimeOutput::Value(v));
                }
                _ => {                    
                    todo!()
                }
            }            
        }
    }
}
