use thiserror::Error;
use colored::Colorize;

#[derive(Error, Debug)]
pub enum Errors {
    #[error("Code 0. Unknown command")]
    CommandError,
    #[error("Code 1. Error getting input")]
    InputError,
    #[error("Code 2. Name cannot be empty or longer than 50 characters")]
    UsernameError,
    #[error("Code 3. Bet must be a number")]
    BetError,
}

pub fn handle_error(e: Errors) {
    println!("{e}");
    helper();
}

pub fn helper() {
    println!("Write {} to bet or {} to view all available options.",
    "\"b\"".bold(),
    "\"h\"".bold(),);
}