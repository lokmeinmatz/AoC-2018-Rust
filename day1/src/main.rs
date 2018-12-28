use std::collections::HashSet;

fn part1(input: &str) {
  println!("Solving day 1 part 1");

  let res : i64 = input
    .split("\n")
    .map(|x| x.trim().parse::<i64>().unwrap_or(0))
    .sum();

  println!("{:?}", res);
}

fn in_vec(v: &Vec<i64>, n: i64) -> bool {
  for elmt in v {
    if *elmt == n {
      return true;
    }
  }
  return false;
}

fn part2(input: &str) {
  println!("Solving day 1 part 2");
  let mut res = 0i64;

  let input = input
    .split("\n")
    .map(|x| x.trim().parse::<i64>().unwrap_or(0))
    .filter(|x| *x != 0);

  let mut allready_encountered_vars: HashSet<i64> = HashSet::new();
  
  for num in input.cycle() {
    allready_encountered_vars.insert(res);
    res += num;
    if allready_encountered_vars.contains(&res) {
      println!("Found twice: {:?}", res);
      //println!("{:?}", allready_encountered_vars);
      break;
    }
  }
  
}

fn main() {
  let input = include_str!("input.in");

  part1(&input);
  part2(&input);
}
