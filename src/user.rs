use crate::{errors::{Errors, helper}, input::user_input, spin::spin_result};


pub struct User {
    pub name: String,
    score: u32,
    high_score: u32
}

impl TryFrom<&str> for User {
    type Error = Errors;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.is_empty() || value.len() > 50 {
            Err(Errors::UsernameError)
        } else {
            Ok(User {
            name: value.to_string(),
            score: 1000u32,
            high_score: 1000u32,
            })
        }
    }
}

impl TryFrom<String> for User {
    type Error = Errors;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.as_str().try_into()
    }
}

pub fn parse_bet(bet: String) -> Result<u32,Errors> {
    if let Ok(bet) = bet.parse::<u32>() {
        Ok(bet)
    } else {
        Err(Errors::BetError)
    }
}

impl User {
    pub fn new() -> Result<User, Errors> {
        println!("Please choose a name");
        let name = user_input()?; 
        name.try_into()      
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

    pub fn place_bet(&mut self) -> Result<(), Errors> {
        let input = user_input()?;
        let bet = parse_bet(input)?;
        self.process_bet(bet);
        Ok(())

    }

    pub fn process_bet(&mut self, bet: u32) {
        if bet > self.score() {
            println!("Not enough credits! Current: {}", &self.score());
        } else if bet == 0 {
            println!("Cannot bet 0");
        } else {
            self.deduct_bet(bet);
            spin_result(self, bet);
        }
    }

    pub fn restart(&mut self) {
        self.score = 1000;
        helper();
    }

    pub fn deduct_bet(&mut self, betval: u32) {
        self.score -= betval;
    }
}

