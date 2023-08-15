use std::str::FromStr;
use std::fs;
use std::path::PathBuf;
use vcg::{BoundedResult,Program};

pub static REFTESTS_DIR: &str = "tests/files";

// Include the programmatically generated test file.
include!(concat!(env!("OUT_DIR"), "/srcfile_tests.rs"));

/// Run a specific test by loading the file out of the reference tests
/// repository and attempting to parse it.  All reference tests should
/// parse correctly.
fn check(test: &str) {
    // Construct filename
    let mut path = PathBuf::from(REFTESTS_DIR);
    path.push(test);
    let filename = path.as_path().to_str().unwrap();
    // Read the test file
    let input = fs::read_to_string(filename).unwrap();
    // Convert into a program
    let program = Program::from_str(&input).unwrap();
    // Print out errors
    for r in program.check() {
    	match r {
    	    BoundedResult::Ok(_) => {}
    	    BoundedResult::Err(_) => {
    		assert!(false,"verification shouldn't have failed");
    	    }
    	    BoundedResult::OutOfResource => {
    		assert!(false,"verification out-of-resource");
    	    }
    	}
    }
}
