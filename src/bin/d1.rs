//! Day 1 challenge is just to count some shit
use std::io;
use std::io::*;

fn main() -> () {
  let net: i64 =
    io::BufReader::new(io::stdin()).lines().map(
      |line|
      line.expect("Every line should be valid Unicode!")
      .parse::<i64>().expect("Every line should be a parsable i64!")
    ).sum();
  println!("Sum: {}", net);
}
