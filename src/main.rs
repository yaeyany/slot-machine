mod symbols;
mod spin;
mod user;
mod input;
mod commands;


use thiserror::Error;
use crate::{commands::{Commands, execute_command}, input::input, user::User};

#[derive(Error, Debug)]
pub enum Errors {
    #[error("Command unknown")]
    CommandError,
}

fn main() {

    println!("Welcome to our slot machine");
    let mut user = User::new();
    loop {
        if user.score() == 0 {
            println!("Game over, high score is {}\nWrite anything to start again, Q to quit", &user.high_score());
            let input = input();
            match Commands::try_from(input) {
                Ok(Commands::Quit) => break,
                _ => user.restart(),
            }
        }

        println!("\nYou have {}. Please make a bet: ", user.score());
        
        let input = input();

        if let Ok(bet) = input.parse::<u32>() {
            user.place_bet(bet);
        } else {
            if let Ok(command) = input.try_into() {
                execute_command(command, &mut user);
            }
        }
    };
}
