mod symbols;
mod spin;
mod user;
mod input;
mod commands;
mod errors;

use crate::{commands::{CommandOutcome, Commands, execute_command, game_loss, welcome}, errors::{Errors, handle_error}, input::user_input, user::User};

fn main() -> Result<(), Errors>{
    let mut user = User::new()?;
    welcome();
    loop {
        if user.score() == 0 {
            if !should_continue(game_loss(&mut user)) {
                break Ok(());
            }
        }
        if !should_continue(command_cycle(&mut user)) {
            break Ok(());
        }
    }
}

fn command_cycle(user: &mut User) -> Result<CommandOutcome, Errors>{
    let input = user_input()?;
    let command = Commands::try_from(input)?;
    execute_command(command, user)
} 

fn should_continue(outcome: Result<CommandOutcome, Errors>) -> bool {
    match outcome {
        Ok(CommandOutcome::Continue) => true,
        Ok(CommandOutcome::Quit) => false,
        Err(e) => {
            handle_error(e);
            true
        },
    }
}