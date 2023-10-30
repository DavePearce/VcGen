use std::marker::PhantomData;
use crate::{Machine,MachineError,MachineState,MachineWord,MinimalMachineError};

/// A minimal implementation of `MachineState` which represents data
/// using a single, flat, vector.  As such, this is primarily useful
/// for runtime execution rather than abstract execution.
pub struct VecState<T,E=MinimalMachineError> {
    dummy: PhantomData<E>,
    pc: usize,
    stack: Vec<T>
}

impl<T,E> VecState<T,E> {
    pub fn init() -> Self {
        Self{pc:0, stack: Vec::new(), dummy: PhantomData}
    }
}

impl<T:MachineWord,E:MachineError> MachineState for VecState<T,E> {
    type Word = T;
    type Error = E;
    
    fn size(&self) -> usize { self.stack.len() }

    fn pc(&self) -> usize { self.pc }
    
    fn peek(&self, n: usize) -> Result<&Self::Word,Self::Error> {
        if self.stack.len() > n {
            let m = self.stack.len() - (n+1);
            Ok(&self.stack[m])
        } else {
            Err(E::stack_underflow())
        }
    }

    fn push(&mut self, item: Self::Word) -> Result<(),Self::Error> {
        Ok(self.stack.push(item))
    }

    fn pop(&mut self) -> Result<Self::Word,Self::Error> {
        if self.stack.len() > 0 {
            Ok(self.stack.pop().unwrap())
        } else {
            Err(E::stack_underflow())            
        }
    }

    fn set(&mut self, n: usize, item: Self::Word) -> Result<Self::Word,Self::Error> {
        todo!()
    }

    fn swap(&mut self, n: usize) -> Result<(),Self::Error> {
        todo!()
    }

    /// Set position within instruction sequence.
    fn goto(&mut self, pc: usize) {
        self.pc = pc;
    }    
}
