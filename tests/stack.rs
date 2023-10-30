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

#[test]
fn test_03() {
    let bytecode = vec![
        Push1(0x1),
        Push1(0x2),
        Sub,
        Return
    ];

    check(bytecode,Ok(RuntimeOutput::Value(0xff)))
}

#[test]
fn test_04() {
    let bytecode = vec![
        Push1(0x1),
        Push1(0x2),
        Eq,
        Return
    ];

    check(bytecode,Ok(RuntimeOutput::Value(0x0)))
}

fn check(code: Vec<Bytecode>, output: Result<RuntimeOutput,MinimalMachineError>) {
    let svm = StackMachine::<u8>::new(code);
    let init = VecState::<u8>::init();
    let o = svm.execute(init);
    assert_eq!(o,output);
}

// ===================================================================
// Machine definition
// ===================================================================

/// A non-trivial, yet minimalistic machine implementation.  This can
/// describe non-trivial computation and is primarily based around a
/// stack machine.  
pub struct StackMachine<T:MachineWord> {
    dummy: PhantomData<T>,
    code: Vec<Bytecode>
}

impl<T:MachineWord> StackMachine<T> {
    pub fn new(code: Vec<Bytecode>) -> Self {
        Self{code,dummy: PhantomData}
    }
}

// ===================================================================
// Bytecodes
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
    Return
}

#[derive(Clone,Copy,Debug,PartialEq)]
pub enum RuntimeOutput {
    Error,
    Value(u8)
}

// ===================================================================
// Semantics
// ===================================================================

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
                Bytecode::Eq => {
                    let r = state.pop()?;
                    let l = state.pop()?;
                    state.push(l.equal(r))?;
                    state.goto(state.pc()+1);                    
                }
                Bytecode::Neq => {
                    let r = state.pop()?;
                    let l = state.pop()?;
                    state.push(l.equal(r).not())?;
                    state.goto(state.pc()+1);                    
                }
                Bytecode::Lt => {
                    let r = state.pop()?;
                    let l = state.pop()?;
                    state.push(l.less_than(r))?;
                    state.goto(state.pc()+1);                    
                }
                Bytecode::LtEq => {
                    let r = state.pop()?;
                    let l = state.pop()?;
                    state.push(l.equal(r).or(l.less_than(r)))?;
                    state.goto(state.pc()+1);                    
                }
                Bytecode::Gt => {
                    let r = state.pop()?;
                    let l = state.pop()?;
                    state.push(r.less_than(l))?;
                    state.goto(state.pc()+1);                    
                }
                Bytecode::GtEq => {
                    let r = state.pop()?;
                    let l = state.pop()?;
                    state.push(l.equal(r).or(r.less_than(l)))?;
                    state.goto(state.pc()+1);                    
                }
                Bytecode::Add => {
                    let r = state.pop()?;
                    let l = state.pop()?;
                    state.push(l.wrapping_add(r))?;
                    state.goto(state.pc()+1);                    
                }
                Bytecode::Sub => {
                    let r = state.pop()?;
                    let l = state.pop()?;
                    state.push(l.wrapping_sub(r))?;
                    state.goto(state.pc()+1);                    
                }
                Bytecode::Mul => {
                    let r = state.pop()?;
                    let l = state.pop()?;
                    state.push(l.wrapping_mul(r))?;
                    state.goto(state.pc()+1);                    
                }
                Bytecode::Div => {
                    let r = state.pop()?;
                    let l = state.pop()?;
                    state.push(l / r)?;
                    state.goto(state.pc()+1);                    
                }
                Bytecode::Rem => {
                    let r = state.pop()?;
                    let l = state.pop()?;
                    state.push(l % r)?;
                    state.goto(state.pc()+1);                    
                }
                Bytecode::Return => {
                    let v = state.pop()?;
                    return Ok(RuntimeOutput::Value(v));
                }
            }            
        }
    }
}
