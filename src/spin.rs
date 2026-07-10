use crate::symbols::{ALL_SYMBOLS, WEIGHTS, Symbol};
use crate::user::User;
use::rand::prelude::*;
use::rand::distr::weighted::WeightedIndex;


pub fn full_spin() -> (Symbol,Symbol,Symbol) {
    let dist = WeightedIndex::new(&WEIGHTS).unwrap();
    let mut rng = rand::rng();

    let left   = ALL_SYMBOLS[dist.sample(&mut rng)];
    let middle = ALL_SYMBOLS[dist.sample(&mut rng)];
    let right  = ALL_SYMBOLS[dist.sample(&mut rng)];

    println!("{} | {} | {}", left.display(), middle.display(), right.display());
    
    (left, middle, right)
}
 
pub fn spin_result(user: &mut User, betval: u32) {
    let (x,y,z) = full_spin();
    if x == y && y == z {
        let win_amount = betval * x.payout();
        user.add_score(win_amount);
        println!("Congrats")
    } else {
        println!("No win");
    };
}