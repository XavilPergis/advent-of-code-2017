extern crate advent_of_code;
use advent_of_code::puzzle;

fn main() {
    println!("{}", puzzle(std::env::args().nth(1).expect("No input found")));
}