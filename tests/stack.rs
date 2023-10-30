mod stackvm;

use vcg::{Machine,MachineWord};
use stackvm::{Bytecode,RuntimeOutput,StackMachine,StackMachineState};

use Bytecode::*;

#[test]
fn test_01() {
    let bytecode = vec![
        Push1(0x1),
        Push1(0x2),
        Add,
        Return
    ];

    check(bytecode,RuntimeOutput::Value(0x3))
}

fn check(code: Vec<Bytecode>, output: RuntimeOutput) {
    let svm = StackMachine::<u8>::new(code);
    let init = StackMachineState::<u8>::init();
    let o = svm.execute(init);
    assert_eq!(o,output);
}
