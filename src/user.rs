pub struct User {
    name: String,
    score: u32
}

impl User {
    pub fn new(name: String, score: u32) -> User {
        User {
            name,
            score
        }
    }
}

fn name_check(name: &String) {
    if name.is_empty() {
        println!("Name cannot be empty")
    }
}