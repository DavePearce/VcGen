use vcg::{Bytecode,BoundedResult,Program};
use vcg::Bytecode::*;

// ===================================================================
// Logic (Literal)
// ===================================================================

#[test]
fn test_logic_01() {
    check(true,&[Assert,Bool(true)]);
}

#[test]
fn test_logic_02() {
    check(false,&[Assert,Bool(false)]);
}

#[test]
fn test_logic_03() {
    check(true,&[Assert,Not,Bool(false)]);
}

#[test]
fn test_logic_04() {
    check(false,&[Assert,And,Bool(false),Bool(false)]);
}

#[test]
fn test_logic_05() {
    check(false,&[Assert,And,Bool(true),Bool(false)]);
}

#[test]
fn test_logic_06() {
    check(false,&[Assert,And,Bool(false),Bool(true)]);
}

#[test]
fn test_logic_07() {
    check(true,&[Assert,And,Bool(true),Bool(true)]);
}

#[test]
fn test_logic_08() {
    check(false,&[Assert,Or,Bool(false),Bool(false)]);
}

#[test]
fn test_logic_09() {
    check(true,&[Assert,Or,Bool(true),Bool(false)]);
}

#[test]
fn test_logic_10() {
    check(true,&[Assert,Or,Bool(false),Bool(true)]);
}

#[test]
fn test_logic_11() {
    check(true,&[Assert,Or,Bool(true),Bool(true)]);
}

#[test]
fn test_logic_12() {
    check(true,&[Assert,Not,And,Bool(false),Bool(false)]);
}

#[test]
fn test_logic_13() {
    check(true,&[Assert,Not,Or,Bool(false),Bool(false)]);
}

// ===================================================================
// Logic (Variable)
// ===================================================================

#[test]
fn test_logic_14() {
    check(false,&[Assert,Var(0)]);
}

#[test]
fn test_logic_15() {
    check(false,&[Assert,And,Var(0),Not,Var(0)]);
}

#[test]
fn test_logic_16() {
    check(true,&[Assert,Or,Bool(true),Var(0)]);
}

#[test]
fn test_logic_17() {
    check(true,&[Assert,Implies,IsBool,Var(0),Or,Var(0),Bool(true)]);
}

#[test]
fn test_logic_18() {
    check(true,&[Assert,Implies,IsBool,Var(0),Or,Var(0),Not,Var(0)]);
}

// ===================================================================
// Comparator (Literal)
// ===================================================================

#[test]
fn test_comparator_01() {
    check(true,&[Assert,Eq,Int(0),Int(0)]);
}

#[test]
fn test_comparator_02() {
    check(true,&[Assert,Neq,Int(0),Int(1)]);
}

#[test]
fn test_comparator_03() {
    check(true,&[Assert,LtEq,Int(0),Int(0)]);
}

#[test]
fn test_comparator_04() {
    check(true,&[Assert,LtEq,Int(0),Int(1)]);
}

#[test]
fn test_comparator_05() {
    check(true,&[Assert,Lt,Int(0),Int(1)]);
}

#[test]
fn test_comparator_06() {
    check(true,&[Assert,GtEq,Int(0),Int(0)]);
}

#[test]
fn test_comparator_07() {
    check(true,&[Assert,GtEq,Int(1),Int(0)]);
}

#[test]
fn test_comparator_08() {
    check(true,&[Assert,Gt,Int(1),Int(0)]);
}

// Comparator (Variable)

#[test]
fn test_comparator_09() {
    check(true,&[Assert,Implies,IsUint,Var(0),Eq,Var(0),Var(0)]);
}

#[test]
fn test_comparator_10() {
    check(false,&[Assert,Implies,IsUint,Var(0),Eq,Int(1),Var(0)]);
}

#[test]
fn test_comparator_11() {
    check(false,&[Assert,Implies,IsUint,Var(0),Eq,Var(0),Int(1)]);
}

#[test]
fn test_comparator_12() {
    check(true,&[Assert,Implies,IsUint,Var(0),LtEq,Var(0),Var(0)]);
}

#[test]
fn test_comparator_13() {
    check(true,&[Assert,Implies,IsUint,Var(0),LtEq,Int(0),Var(0)]);
}

#[test]
fn test_comparator_14() {
    check(true,&[Assert,Implies,IsUint,Var(0),GtEq,Var(0),Var(0)]);
}

#[test]
fn test_comparator_15() {
    check(true,&[Assert,Implies,IsUint,Var(0),GtEq,Var(0),Int(0)]);
}

#[test]
fn test_comparator_16() {
    check(false,&[Assert,Implies,IsUint,Var(0),Lt,Var(0),Var(0)]);
}

#[test]
fn test_comparator_17() {
    check(true,&[Assert,Implies,IsUint,Var(0),Not,Lt,Var(0),Var(0)]);
}

#[test]
fn test_comparator_18() {
    check(false,&[Assert,Implies,IsUint,Var(0),Gt,Var(0),Var(0)]);
}

#[test]
fn test_comparator_19() {
    check(true,&[Assert,Implies,IsUint,Var(0),Not,Gt,Var(0),Var(0)]);
}

// ===================================================================
// Arithmetic (Literal)
// ===================================================================

#[test]
fn test_arithmetic_01() {
    check(true,&[Assert,Eq,Int(1),Add,Int(1),Int(0)]);
}

#[test]
fn test_arithmetic_02() {
    check(true,&[Assert,Eq,Int(1),Add,Int(0),Int(1)]);
}

#[test]
fn test_arithmetic_03() {
    check(true,&[Assert,Eq,Int(1),Sub,Int(2),Int(1)]);
}

#[test]
fn test_arithmetic_04() {
    check(true,&[Assert,Eq,Int(0),Mul,Int(1),Int(0)]);
}

#[test]
fn test_arithmetic_05() {
    check(true,&[Assert,Eq,Int(0),Mul,Int(0),Int(1)]);
}

#[test]
fn test_arithmetic_06() {
    check(true,&[Assert,Eq,Int(1),Mul,Int(1),Int(1)]);
}

#[test]
fn test_arithmetic_07() {
    check(true,&[Assert,Eq,Int(2),Mul,Int(2),Int(1)]);
}

#[test]
fn test_arithmetic_08() {
    check(true,&[Assert,Eq,Int(2),Mul,Int(1),Int(2)]);
}

#[test]
fn test_arithmetic_09() {
    check(true,&[Assert,Eq,Int(2),Div,Int(2),Int(1)]);
}

#[test]
fn test_arithmetic_10() {
    check(true,&[Assert,Eq,Int(1),Div,Int(2),Int(2)]);
}

#[test]
fn test_arithmetic_11() {
    check(true,&[Assert,Eq,Int(2),Div,Int(4),Int(2)]);
}

#[test]
fn test_arithmetic_12() {
    check(true,&[Assert,Eq,Int(0),Rem,Int(2),Int(1)]);
}

#[test]
fn test_arithmetic_13() {
    check(true,&[Assert,Eq,Int(1),Rem,Int(1),Int(2)]);
}

#[test]
fn test_arithmetic_14() {
    check(true,&[Assert,Eq,Int(1),Rem,Int(7),Int(3)]);
}

// ===================================================================
// Arithmetic (Variable)
// ===================================================================

#[test]
fn test_arithmetic_15() {
    check(true,&[Assert,Implies,IsUint,Var(0),Neq,Add,Int(1),Var(0),Var(0)]);
}

#[test]
fn test_arithmetic_16() {
    check(true,&[Assert,Implies,IsUint,Var(0),Eq,Add,Int(1),Var(0),Add,Var(0),Int(1)]);
}

#[test]
fn test_arithmetic_17() {
    check(true,&[Assert,Implies,IsUint,Var(0),Eq,Mul,Int(1),Var(0),Mul,Var(0),Int(1)]);
}

#[test]
fn test_arithmetic_18() {
    check(true,&[Assert,Implies,IsUint,Var(0),Eq,Mul,Int(2),Var(0),Mul,Var(0),Int(2)]);
}

#[test]
fn test_arithmetic_19() {
    check(true,&[Assert,Implies,IsUint,Var(0),Eq,Add,Var(0),Var(0),Mul,Var(0),Int(2)]);
}

#[test]
fn test_arithmetic_20() {
    check(true,&[Assert,Implies,IsUint,Var(0),Eq,Mul,Var(0),Int(2),Add,Var(0),Var(0)]);
}

#[test]
fn test_arithmetic_21() {
    check(true,&[Assert,Implies,IsUint,Var(0),Eq,Var(0),Div,Mul,Int(2),Var(0),Int(2)]);
}

// ===================================================================
// Helpers
// ===================================================================

fn check(expect: bool, codes: &[Bytecode]) {
    let program = Program::new(codes.to_vec());
    // Print out errors
    for r in program.check() {
	match r {
	    BoundedResult::Ok(_) => {
		assert!(expect,"verification should have failed");
	    }
	    BoundedResult::Err(_) => {
		assert!(!expect,"verification shouldn't have failed");
	    }
	    BoundedResult::OutOfResource => {
		assert!(!expect,"verification out-of-resource");
	    }
	}
    }
}
