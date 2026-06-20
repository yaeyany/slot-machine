use std::io::stdin;

use crate::spin::full_spin;

pub struct User {
    pub name: String,
    score: u32,
    high_score: u32
}

impl User {
    pub fn new() -> User {
        loop {
            println!("Please choose a name");

            let mut name = String::new();
            stdin().read_line(&mut name).expect("Failed to read name");
            name = name.trim().to_string();

            if name_check(&name) {
                println!("Name cannot be empty");
                continue;
            } else {
                return User {
                name,
                score: 1000u32,
                high_score: 1000u32,
                }
            }
        }
        
    }

    pub fn score(&self) -> u32 {
        self.score
    }
    
    pub fn high_score(&self) -> u32 {
        self.high_score
    }

    pub fn add_score(&mut self, add: u32) {
        self.score += add;
        if self.score > self.high_score {
            self.high_score = self.score
        }
    }

    pub fn topup(&mut self) {
        self.add_score(1000);
        println!("\nThe house always wins");
    }

    pub fn supersecretcheatcode(&mut self) {
        self.add_score(1000000);
        println!("\nWhy?");
    }

    pub fn place_bet(&mut self, betval: u32){
        self.score -= betval;
        let (x,y,z) = full_spin();
        if x == y && y == z {
            let win_amount = betval * x.payout();
            self.add_score(win_amount);
            println!("Congrats")
        } else {
            println!("No win");
        };

    }

    pub fn restart(&mut self) {
        self.score = 1000;
    }
}

fn name_check(name: &str) -> bool {
    name.trim().is_empty() 
}