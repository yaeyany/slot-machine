mod symbols;
mod spin;
mod user;
mod input;

use crate::input::{input, match_input};

fn main() {

    println!("Welcome to our slot machine");
    let mut user = user::User::new();
    loop {
        if user.score() == 0 {
            println!("Game over, high score is {}\nWrite anything to start again, Q to quit", &user.high_score());
            let bet_trim = input();
            if match_input(&mut user, &bet_trim) {
                break;
            };
            user.restart();
        }

        println!("\nYou have {}. Please make a bet: ", user.score());
        
        let bet_trim = input();
        if match_input(&mut user, &bet_trim) {
            break;
        };
        
        let betval: u32 = match bet_trim.parse::<u32>() {
            Ok(0) => {
                println!("Bet must be at least 1");
                continue;
            }
            Ok(num) => num,
            Err(_) => {
                println!("Please write a number");
                continue;
                }
            };
            
        if betval > user.score() {
            println!("Not enough credits! Current: {}", &user.score());
            continue;
        }
        user.place_bet(betval);
    };
}
