use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

fn part1(input: &String) {
  println!("Solving day 2 part 1");

}



fn part2(input: &String) {  
}

fn main() {
  let mut file = File::open("input").unwrap();
  let mut input = String::new();
  file.read_to_string(&mut input).unwrap();

  part1(&input);
  part2(&input);
}
