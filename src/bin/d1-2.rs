//! Day 1 challenge is just to count some shit
use std::io;
use std::io::*;
use std::collections::HashMap;

fn main() -> () {
  let (net, mut hashmap): (i64, HashMap<i64, ()>) =
    io::BufReader::new(io::stdin()).lines().map(
      |line|
      line.expect("Every line should be valid Unicode!")
      .parse::<i64>().expect("Every line should be a parsable i64!")
    ).collect::<Vec<_>>()
    .into_iter()
    .cycle()
    .fold((0, HashMap::with_capacity(100)), move |(accum, mut map), val| {
      if let Some(_) = map.insert(accum, ()) {
        println!("Value {} has been used more than once..", accum);
        std::process::exit(0);
      };
      (accum + val, map)
    });
}
