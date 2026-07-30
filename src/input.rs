use std::io::stdin;

use crate::Errors::{self, InputError};

pub fn user_input() -> Result<String, Errors>{
    let mut input = String::new();
    if stdin().read_line(&mut input).is_err() {
        return Err(InputError);
    }
    Ok(input.trim().to_string())
}