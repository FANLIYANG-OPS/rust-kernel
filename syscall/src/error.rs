use core::{fmt, result};

#[derive(PartialEq, Eq)]
pub struct Error {
    pub err_num: i32,
}
pub type Result<T, E = Error> = result::Result<T, E>;
impl Error {
    pub fn new(err: i32) -> Error {
        Error { err_num: err }
    }

    pub fn mux(result: Result<usize>) -> usize {
        match result {
            Ok(value) => value,
            Err(error) => -error.err_num as usize,
        }
    }

    pub fn demux(value: usize) -> Result<usize> {
        let err_num = -(value as i32);
        if err_num >= -1 && err_num < STR_ER
    }
}

