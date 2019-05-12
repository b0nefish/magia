use std::io::{Error, ErrorKind, Result};
use std::process::Command;
use std::{fs, str};
use tempfile::NamedTempFile;

///
/// Compile m68k assembly and output a binary blob of the code
///
/// This code will currently generate a temporary file from the input and load the
/// generated file from the vasm code. It would be much nicer to keep this all in memory but
/// this will gives us a starting point at least
///
pub fn compile_bin(code: &str, name: &str) -> Result<Vec<u8>> {
    let output_name = NamedTempFile::new()?;
    let input_name = NamedTempFile::new()?;

    let output_path = output_name.into_temp_path();
    let input_path = input_name.into_temp_path();

    // TODO: Maybe we need to search backwards so we can be higher up in the directory structure
    // and still be able to call vasm. Remains to be seen but leaving it as is for now.
    let mut command = if cfg!(target_os = "linux") {
        Command::new("../bin/linux/vasm")
    } else if cfg!(target_os = "windows") {
        Command::new("..\\bin\\windows\\vasm.exe")
    } else {
        Command::new("../bin/mac/vasm")
    };

    // write the temp assembly code to a tempory file
    fs::write(&input_path, code)?;

    let vasm_output = command
        .arg(&input_path)
        .arg("-quiet")
        .arg("-Fbin")
        .arg("-o")
        .arg(&output_path)
        .output()?;

    // can close the input file at this point now
    input_path.close()?;

    // If vasm status was ok we read the generated file back, close the temp file and return
    if vasm_output.status.success() {
        let return_data = std::fs::read(&output_path)?;
        output_path.close()?;
        Ok(return_data)
    } else {
        output_path.close()?;
        // expect this to be correct as vasm only prints ascii
        let error_message = str::from_utf8(&vasm_output.stderr).unwrap();
        Err(Error::new(
            ErrorKind::Other,
            format!("{} error: {}", name, error_message),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compile_basic_code() {
        let res = compile_bin(" nop", "nop_code");
        assert_eq!(true, res.is_ok());
        let data = res.unwrap();

        // verify that we got a nop instruction back
        assert_eq!(2, data.len());
        assert_eq!(0x4e, data[0]);
        assert_eq!(0x71, data[1]);
    }

    #[test]
    fn compile_basic_code_fail() {
        let res = compile_bin(" np", "nop_error");
        assert_eq!(true, res.is_err());
    }
}
