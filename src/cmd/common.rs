use easy_error::{Error, bail};
use std::fs;

pub fn read_file(file_path: &str) -> Result<String, Error> {
    match fs::read_to_string(file_path) {
        Ok(res) => return Ok(res),
        Err(err) => bail!("{}", err),
    }
}

pub fn write_file(file_path: &str, data: Vec<u8>) -> Result<(), Error> {
    match fs::write(file_path, data) {
        Ok(res) => return Ok(res),
        Err(err) => bail!("{}", err),
    }
}
