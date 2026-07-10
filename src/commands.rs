use std::process::exit;

use crate::{Errors::{self, CommandError}, user::User};
pub enum Commands {
    Quit,
    Help,
    Stats,
    Topup,
    Cheat
}

impl TryFrom<&str> for Commands {
    type Error = Errors;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "q"| "quit"| "exit"| "cancel" => Ok(Commands::Quit),
            "h" | "help" => Ok(Commands::Help),
            "stats" | "s" | "i" => Ok(Commands::Stats),
            "r"| "topup" => Ok(Commands::Topup),
            "supersecretcheatcode" => Ok(Commands::Cheat),
            _ => Err(CommandError)
        }
    }
}

impl TryFrom<String> for Commands {
    type Error = Errors;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

pub fn execute_command(command: Commands, user: &mut User) {
    match command {
        Commands::Quit => {
            println!("\nGoodbye, you ended with {} and your highest score was {}", &user.score(), &user.high_score());
            exit(1)
        },
        Commands::Help => println!("\nq, quit, exit, cancel - to quit\nr, topup - to add 1000\nsupersecretcheatcode - to cheat\ni, s, stats - for info\nhelp/h for help"),
        Commands::Stats => println!("\nPlayer: {}, highscore {}\nYou currently have {}", &user.name, &user.high_score(), &user.score()),
        Commands::Topup => user.topup(),
        Commands::Cheat => user.supersecretcheatcode(),
    }
}