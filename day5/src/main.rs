use std::collections::*;



fn react(polymer : &mut Vec<char>) {
  loop {
      let mut changed = false;
      let mut i = 0;
      while i < polymer.len() - 1 {

        let ch1 = polymer[i];
        let ch2 = polymer[i+1];

        if ch1.to_lowercase().next().unwrap() == ch2.to_lowercase().next().unwrap() && ch1 != ch2 {
          //remove
          polymer.remove(i);
          polymer.remove(i);
          changed == true;
          //println!("{:?} <==> {:?}", ch1, ch2);
          if i > 0 {i -= 1;}
        }
        else {
          i += 1;
        }
      }

      if !changed {break;}
  }
}

const chrs_to_remove : [char; 26] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];

fn part1(input: &str) {
  println!("Solving day 2 part 1");
  let mut polymer = input.trim().chars().collect::<Vec<_>>();
  react(&mut polymer);
  
  println!("{:?}", polymer);
  println!("{:?}", polymer.len());
}



fn part2(input: &str) {  
  println!("Solving day 2 part 1");
  let mut polymer = input.trim().chars();
  for chr_to_remove in &chrs_to_remove {
    //println!("Removing {:?}", chr_to_remove);
    let mut reduced_polymer = polymer.clone().filter(|x| !x.eq_ignore_ascii_case(chr_to_remove)).collect::<Vec<_>>();
    react(&mut reduced_polymer);
    println!("{:?} : {}", chr_to_remove, reduced_polymer.len());
  }
}

fn main() {
  let mut input = include_str!("input");

  //part1(&input);
  part2(&input);
}
