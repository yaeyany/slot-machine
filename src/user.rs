use std::io::stdin;

pub struct User {
    pub name: String,
    pub score: u32
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
                score: 1000u32
                }
            }
        }
        
    }

    pub fn score_look(&self) -> &u32 {
        &self.score
    }
    pub fn topup(&mut self) {
        self.score += 1000;
        println!("The house always wins");
    }
    pub fn supersecretcheatcode(&mut self) {
        self.score += 1000000;
        println!("Why?");
    }
}

fn name_check(name: &String) -> bool {
    name.trim().is_empty() 
}