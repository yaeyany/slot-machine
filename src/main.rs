mod symbols;
mod spin;
use::std::io::*;

use spin::full_spin;

fn main() {

    let mut score = 1000u32;
    println!("Welcome to our slot machine");
    loop {

        if score == 0 {
            println!("Game over");
            break;
        }

        println!("You currently have {}.\nPlease make a bet: ", score);
        
        let mut bet = String::new();
        stdin().read_line(&mut bet).expect("Failed_bet");
        let bet_trim = bet.trim().to_lowercase();
        
        match bet_trim.as_str() {
            "q"| "quit"| "exit"| "cancel" => {
                println!("Goodbye, your score was {}", score);
                break;
            },
            "topup" => {
                score += 1000;
                println!("The house always wins");
                continue;
            }
            "supersecretcheatcode" => {
                score += 1000000;
                println!("Why?");
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
            
        if betval > score {
            println!("Not enough credits! Current: {}", score);
            continue;
        }

        score -= betval;
        let (x,y,z) = full_spin();

        if x == y && y == z {
            let win_amount = betval * x.payout();
            score += win_amount;
            println!("Congrats")
        } else {
            println!("No win");
        };
    };
}
