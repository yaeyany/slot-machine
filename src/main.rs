mod symbols;
mod spin;
mod user;

use::std::io::*;
use spin::full_spin;

fn main() {

    println!("Welcome to our slot machine");
    let mut user = user::User::new();
    loop {
        if user.score_look() == &0u32 {
            println!("Game over");
            break;
        }

        println!("Player: {}\nYou currently have {}.\nPlease make a bet: ", &user.name, user.score_look());
        
        let mut bet = String::new();
        stdin().read_line(&mut bet).expect("Failed_bet");
        let bet_trim = bet.trim().to_lowercase();
        
        match bet_trim.as_str() {
            "q"| "quit"| "exit"| "cancel" => {
                println!("Goodbye, your score was {}", user.score_look());
                break;
            },
            "topup" => {
                user.topup();
                continue;
            }
            "supersecretcheatcode" => {
                user.supersecretcheatcode();
                continue;
            }
            _ => (),
        }
        
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
            
        if &betval > user.score_look() {
            println!("Not enough credits! Current: {}", user.score_look());
            continue;
        }

        user.score -= betval;
        let (x,y,z) = full_spin();

        if x == y && y == z {
            let win_amount = betval * x.payout();
            user.score += win_amount;
            println!("Congrats")
        } else {
            println!("No win");
        };
    };
}
