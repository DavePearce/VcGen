/// An abstract "machine" which can be used to (abstractly) execute
/// instructions.  The model is that of a stack machine whose elements
/// are bitstrings of arbitrary length.  The size of the stack is
/// always known (i.e. concrete), but the elements on the stack may be
/// abstract (in some sense).  Furthermore, the machine has fixed
/// instruction sequence which defines its "program".
pub trait Machine {
    /// A specific state of the abstract machine.
    type State : MachineState;

    /// Represents the outcome from executing the next instruction in
    /// a given state.  Observe that this could be one (or more)
    /// states which are to be executed next.  It could also indicate
    /// that the given instruction was terminating (in some sense).
    type Outcome;

    /// Represents an arbitrary instruction which can be executed by
    /// this machine.
    type Instruction;

    /// Get the instruction at a given position in the instruction
    /// stream.  Observe that this is _unbounded_.  That is, we assume
    /// an instruction exists at every possible `pc` offset.
    fn get(&self, pc: usize) -> &Self::Instruction;
    
    /// Execute a given state producing zero (or more) states.  No
    /// states are produced if the given instruction is terminating
    /// (in some sense).  One state is produced if the given
    /// instruction follows on to the next.  Multiple states can be
    /// produced when a given instruction forks (or _branches_) the
    /// control-flow.
    fn execute(&self, _: Self::State) -> Self::Outcome;
}

/// Represents a single state of the stack machine.
pub trait MachineState {
    /// Defines what constitutes a word in the machine (i.e. an
    /// element of the stack).
    type Word : MachineWord;

    /// Get the size of the stack.  This is _always_ known at compile
    /// time.
    fn size(&self) -> usize;

    /// Peek `nth` item from stack (where `n==0` is top element).
    fn peek(&self, n: usize) -> &Self::Word;

    /// Push an item onto the stack.
    fn push(&mut self, item: Self::Word);

    /// Pop an item from the stack.
    fn pop(&mut self) -> Self::Word;

    /// Set `nth` item from stack (where `n==0` is top element),
    /// whilst returning the item previously at that position.
    fn set(&mut self, n: usize, item: Self::Word) -> Self::Word;

    /// Swap top item on stack with nth item on stack (where `n>0`,
    /// and `n==0` would be the top element).  Whilst this can be
    /// implemented using `peek()` and `set()`, this offers more
    /// efficiency.
    fn swap(&mut self, n: usize);

    /// Set position within instruction sequence.
    fn goto(&mut self, pc: usize);
}

/// Represents the fundamental unit of computation within a stack
/// machine.  This is intentially left abstract, so that it could be
/// reused across both _concrete_ and _abstract_ semantics.
pub trait MachineWord {
    // Comparators
    fn less_than(self,rhs:Self)->Self;
    fn equal(self,rhs:Self)->Self;
    // Arithmetic
    fn add(self,rhs:Self)->Self;
    fn mul(self,rhs:Self)->Self;
    fn div(self,rhs:Self)->Self;
    fn rem(self,rhs:Self)->Self;
    fn neg(self)->Self;
    // Bitwise
    fn and(self,rhs:Self)->Self;
    fn or(self,rhs:Self)->Self;
    fn xor(self,rhs:Self)->Self;
    fn not(self)->Self;
}
