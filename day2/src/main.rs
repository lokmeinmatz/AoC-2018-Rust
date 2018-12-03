use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn part1(input: &String) {
  println!("Solving day 2 part 1");
  let mut exactly2 = 0u64;
  let mut exactly3 = 0u64;

  for line in input.split("\n") {
    let charcount = count_chars(line);
    
    //check if charcount contains mn. 1 char === 2
    for (_, count) in &charcount {
      if *count == 2 {
        exactly2 += 1;
        break
      }
    }

    //check if charcount contains mn. 1 char === 2
    for (_, count) in &charcount {
      if *count == 3 {
        exactly3 += 1;
        break
      }
    }
  }
  println!("Exactly 2 : {}\nExactly 3 : {}\n Checksum : {}", exactly2, exactly3, exactly2 * exactly3);
}

fn count_chars(line : &str) -> HashMap<char, u32> {
  let mut charcount = HashMap::new();

  for chr in line.trim().chars() {
    let chr_current = charcount.get(&chr).unwrap_or(&0);
    charcount.insert(chr, *chr_current + 1);
  }

  charcount
}

fn part2(input: &String) {
  println!("Solving day 2 part 2");
  let mut allIDs : Vec<&str> = Vec::with_capacity(252);
  for line in input.split("\n") {
    allIDs.push(line);
  }

  'outer: for i in 0..allIDs.len() {
    let iID = allIDs[i];
    for j in (i+1)..allIDs.len() {
      let jID = allIDs[j];
      let identical = identical_chars(iID, jID);
      if identical.len() == iID.len() - 1 {
        println!("iID : {}\njID : {}\n idn : {}", iID, jID, identical);
        break 'outer;
      }
    }
  }
}

fn identical_chars(in0 : &str, in1 : &str) -> String {
  let mut outstr = String::new();
  for (chr0, chr1) in in0.chars().zip(in1.chars()) {
    if chr0 == chr1 {
      outstr.push(chr0);
    }
  }

  outstr
}

fn main() {
  let mut file = File::open("input").unwrap();
  let mut input = String::new();
  file.read_to_string(&mut input).unwrap();

  part1(&input);
  part2(&input);
}
