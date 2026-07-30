use colored::Colorize;

use crate::{Errors::{self, CommandError}, commands::{CommandOutcome::{Continue, Quit}, Commands::Help}, errors::helper, input::user_input, user::{self, User}};
pub enum Commands {
    Bet,
    Stats,
    Topup,
    Help,
    Quit,
    Cheat
}

pub enum CommandOutcome {
    Continue,
    Quit,
}

impl TryFrom<&str> for Commands {
    type Error = Errors;

    fn try_from(value: &str) -> Result<Commands, Errors> {
        match value.to_lowercase().as_str() {
            "1" | "b" | "bet" => Ok(Commands::Bet),
            "2" | "s" | "stats" => Ok(Commands::Stats),
            "3" | "r" | "topup" => Ok(Commands::Topup),
            "4" | "h" | "help" => Ok(Commands::Help),
            "5" | "q" | "quit" => Ok(Commands::Quit),
            "supersecretcheatcode" => Ok(Commands::Cheat),
            _ => Err(CommandError)
        }
    }
}

impl TryFrom<String> for Commands {
    type Error = Errors;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.as_str().try_into()
    }
}

pub fn welcome() {
    println!("Welcome to our slot machine");
    helper();
}

pub fn farewell(user: &User) {
    println!(
        "Goodbye and thank you for playing.\nYou ended with {} and your highest score was {}",
        &user.score(),
        &user.high_score()
    );
}

pub fn game_loss(user: &mut User) -> Result<CommandOutcome,Errors>{
    println!("Game over, high score is {}\nWrite anything to start again, Q to quit", &user.high_score());
    let input = user_input()?;
    match Commands::try_from(input) {
        Ok(Commands::Quit) => {
            farewell(user);
            Ok(CommandOutcome::Quit)
        }
        _ => {
        user.restart();
        Ok(CommandOutcome::Continue)
        }
    }
}

pub fn execute_command(command: Commands, user: &mut User) -> Result<CommandOutcome, Errors>{
    match command {
        Commands::Bet => {
            println!("You currently have {} credits. Please make a bet:", user.score());
            user.place_bet()?;
            Ok(Continue)
        }
        Commands::Stats => {
            println!(
                "Player: {}, highscore {}\nYou currently have {}",
                &user.name,
                &user.high_score(),
                &user.score()
            );
            Ok(Continue)
        }

        Commands::Topup => {
            user.topup();
            Ok(Continue)
        }

        Commands::Help => {
            println!(
                "{} - make a bet\n{} - show statistics\n{} - add 1000 credits\n{} - show help\n{} - quit\n{} - cheat",
                "\"1\", \"b\"".bold(),
                "\"2\", \"s\"".bold(),
                "\"3\", \"r\"".bold(),
                "\"4\", \"h\"".bold(),
                "\"5\", \"q\"".bold(),
                "\"supersecretcheatcode\"".bold()
            );
            Ok(Continue)
        }

        Commands::Quit => {
            farewell(user);
            Ok(Quit)
        }

        Commands::Cheat => {
            user.supersecretcheatcode();
            Ok(Continue)
        }
    }
}

