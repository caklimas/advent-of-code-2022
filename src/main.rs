use day2::calculate_score;

use crate::day2::calculate_outcome_score;

pub mod day1;
pub mod day2;

fn main() {
    println!("Day2: {}", calculate_score());
    println!("Day2-2 {}", calculate_outcome_score());
}
