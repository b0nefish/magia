use std::io::Result;

#[macro_use]
extern crate lazy_static;

use gag;

use gag::{BufferRedirect, Hold};
use std::io::{Read, Write, Error, ErrorKind};
use std::sync::Mutex;
use std::env;
use std::ffi::CString;
use std::fs;

lazy_static! {
    static ref STDERR_MUTEX: Mutex<()> = Mutex::new(());
}

extern {
    fn vasm_compile_bin(output: *const i8, input: *const i8) -> i32;
}

///
/// Compile m68k assembly and output a binary blob of the code
///
/// This code will currently generate a temporary file from the input and load the
/// generated file from the vasm code. It would be much nicer to keep this all in memory but
/// this will gives us a starting point at least
///
pub fn compile_bin(code: &str, name: &str) -> Result<Vec<u8>> {
    let _l = STDERR_MUTEX.lock().unwrap();
    let mut buf = BufferRedirect::stderr().unwrap();

    let mut vasm_input_filename = env::temp_dir();
    let mut vasm_output_filename = env::temp_dir();

    vasm_input_filename.push("vasm_temp.s");
    vasm_output_filename.push("vasm_output.bin");

    //let t: i32 = vasm_input_filename;

    //let vasm_input_filename = temp_dir.push("vasm_temp.s");
    //let vasm_output_filename = temp_dir.push("vasm_output.bin");

    //let t: i32 = vasm_input_filename;

    fs::write(vasm_input_filename.as_path(), code)?;

    let ffi_input_filename = CString::new(vasm_input_filename.as_os_str().to_str().unwrap()).unwrap();
    let ffi_output_filename = CString::new(vasm_output_filename.as_os_str().to_str().unwrap()).unwrap();

    // Call the vasm C code and check that error code is valid, load the binary file and return
    // that if not return error with the contents of the stderror output
    if unsafe { vasm_compile_bin(ffi_output_filename.as_ptr(), ffi_input_filename.as_ptr()) } == 0 {
        let mut output = String::new();
        buf.read_to_string(&mut output)?;
        Err(Error::new(ErrorKind::Other, output))
    } else {
        Ok(std::fs::read(vasm_output_filename.as_path())?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compile_basic_code() {
        let res = compile_bin(
            "nop
             nop",
             "nop_code");
        res.unwrap();
        //assert_eq!(true, res.is_ok());
    }

    #[test]
    fn compile_basic_code_fail() {
        let res = compile_bin("np\n", "nop_error");
        res.unwrap();
        //assert_eq!(true, res.is_err());
    }
}
