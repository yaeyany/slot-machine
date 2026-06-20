use std::io::stdin;

use crate::user::User;

pub fn input() -> String{
    let mut bet = String::new();
    stdin().read_line(&mut bet).expect("Failed_bet");
    let bet_trim = bet.trim().to_lowercase();
    bet_trim
}

pub fn match_input(user: &mut User, bet_trim: &String) -> bool {
    match bet_trim.as_str() {
            "q"| "quit"| "exit"| "cancel" => {
                println!("\nGoodbye, you ended with {} and your highest score was {}", &user.score(), &user.high_score());
                true
            },
            "r"| "topup" => {
                user.topup();
                false
            }
            "supersecretcheatcode" => {
                user.supersecretcheatcode();
                false
            }
            "stats" | "s" | "i" => {
                println!("\nPlayer: {}, highscore {}\nYou currently have {}", &user.name, &user.high_score(), &user.score());
                false
            }
            "help" | "h" => {
                println!("\nq, quit, exit, cancel - to quit\nr, topup - to add 1000\nsupersecretcheatcode - to cheat\ni, s, stats - for info\nhelp/h for help");
                false
            }
            _ => false,
        }
}
