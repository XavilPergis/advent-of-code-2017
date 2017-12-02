#![cfg_attr(test, feature(test))]
#![allow(unused)]

mod challenges;

use std::path::Path;
use std::fs::File;
use std::io::Read;

pub fn read_file<P: AsRef<Path>>(path: P) -> String {
    let mut buf = String::new();
    let mut file = File::open(path).expect("Could not open file");
    file.read_to_string(&mut buf).unwrap(); // should never fail

    buf
}

pub fn first_arg() -> String {
    std::env::args().nth(1).expect("No input found")
}

static CHALLENGES: &[fn()] = &[
    challenges::day_1::puzzle,
    challenges::day_2::puzzle,
    challenges::day_3::puzzle,
];

pub fn run_challenge(challenge: usize) {
    CHALLENGES[challenge - 1]();
}
