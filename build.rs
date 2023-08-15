use std::fs;
use std::io::Write;
use std::path::Path;

pub static SRCTESTS_DIR: &str = "tests/files";
pub static SRC_EXT: &str = "vcg";

fn gentests(testdir: &str, ext: &str, target: &Path) {
    let mut f = fs::File::create(target).unwrap();
    // Open reference test directory
    let dir = fs::read_dir(testdir).unwrap();

    for e in dir {
        let p = e.as_ref().unwrap().path();
        let n = p.file_stem().unwrap().to_str().unwrap();
        //
        if p.extension().unwrap() == ext {
            writeln!(f).unwrap();
            writeln!(f,"#[test]").unwrap();
            writeln!(f,"fn test_{n}() {{ check(\"{n}.{ext}\"); }}").unwrap();
        }
    }
}

/// The purpose of this script is to generate a set of tests for each
/// of the language reference tests.
fn main() {
    // Create destination file
    let out_dir = std::env::var("OUT_DIR").unwrap();
    // Source tests
    let test_file = std::path::Path::new(&out_dir).join("srcfile_tests.rs");
    gentests(SRCTESTS_DIR,SRC_EXT,&test_file);
}
