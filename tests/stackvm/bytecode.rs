use std::marker::PhantomData;
use vcg::{Machine,MachineState,MachineWord};

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

// ===================================================================
// Stack Machine State
// ===================================================================

pub struct StackMachineState<T> {
    pc: usize,
    stack: Vec<T>
}

impl<T> StackMachineState<T> {
    pub fn init() -> Self {
        Self{pc:0, stack: Vec::new()}
    }
}

impl<T:MachineWord> MachineState for StackMachineState<T> {
    type Word = T;

    fn size(&self) -> usize { self.stack.len() }

    fn peek(&self, n: usize) -> &Self::Word {
        let m = self.stack.len() - (n+1);
        &self.stack[m]
    }

    fn push(&mut self, item: Self::Word) {
        self.stack.push(item)
    }

    fn pop(&mut self) -> Self::Word {
        // TODO: error handling
        self.stack.pop().unwrap()
    }

    fn set(&mut self, n: usize, item: Self::Word) -> Self::Word {
        todo!()
    }

    fn swap(&mut self, n: usize) {
        todo!()
    }

    /// Set position within instruction sequence.
    fn goto(&mut self, pc: usize) {
        self.pc = pc;
    }    
}

impl Machine for StackMachine<u8> {
    type State = StackMachineState<u8>;
    type Instruction = Bytecode;
    type Outcome = RuntimeOutput;

    fn get(&self,pc: usize) -> &Self::Instruction {
        &self.code[pc]
    }

    fn execute(&self, mut state: Self::State) -> Self::Outcome {
        // Execute until nothing left.
        loop {
            //
            match self.get(state.pc) {
                // Literals
                Bytecode::Push1(c) => {
                    state.stack.push(*c);
                    state.pc += 1;                    
                }
                Bytecode::Add => {
                    let r = state.pop();
                    let l = state.pop();
                    state.stack.push(l+r);
                    state.pc += 1;
                }
                Bytecode::Return => {
                    let v = state.pop();
                    return RuntimeOutput::Value(v);
                }
                _ => {                    
                    todo!()
                }
            }            
        }
    }
}
