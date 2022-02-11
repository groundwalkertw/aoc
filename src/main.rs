mod day1;
mod day2;
mod day3;
mod day4;

pub use std::{fs::File, io::{BufReader, BufRead}};

fn main() {
    day1::task1();
    day2::task2();
    day3::task2();
}

