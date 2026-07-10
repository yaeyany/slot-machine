use std::io::stdin;

pub fn input() -> String{
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_lowercase()
}