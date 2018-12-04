use std::collections::HashMap;

#[derive(Debug)]
struct Guard {
  id : u16,
  total_sleep_time: u64,
  max_sleep_minute: (u8, u16),
  sleep_time: Vec<u16>
}

impl Guard {
  fn new(line: &str) -> Guard {
    let content = line.split(" ")
    .enumerate()
    .filter_map(|(i, c)| if i == 1 || i == 2 || i == 3 {Some(c)} else {None}).collect::<Vec<_>>();
    println!("{:?}", content);
    if content[1] != "Guard" {
      panic!("Fatal error: {:?} is no constructor", line);
    }
    Guard {id:content[2].trim_start_matches("#").parse::<_>().unwrap(), sleep_time: vec![0;60],
    max_sleep_minute: (0, 0),
    total_sleep_time:0}
  }

  fn get_id(line: &str) -> Option<u16> {
    line.split(" ").nth(3).expect("No third element here :(").trim_start_matches("#").parse::<u16>().ok()
  }

  fn add_sleep_time(&mut self, start: u8, end: u8) {
    //let start : usize = start.split(":").last().unwrap().trim_end_matches("]").parse().expect("start cannot be converted");
    //let end : usize = end.split(":").last().unwrap().trim_end_matches("]").parse().expect("end cannot be converted");
    for s_minute in start..end {
      self.sleep_time[s_minute as usize] += 1;
    }
    //println!("{} > {}", start, end);
    self.total_sleep_time += (end - start) as u64;
    println!("#{} > {}min", self.id, end -start);
  }

  fn calc_max_sleep_minute(&mut self) {
    let max_val = self.sleep_time.iter().max().unwrap();
    let max_idx = self.sleep_time.iter().position(|x| x == max_val);
    self.max_sleep_minute = (max_idx.unwrap() as u8, *max_val);
  }
}

fn get_minutes(line : &str) -> Option<u8> {
  line.split(" ").nth(1).unwrap().split(":").last().unwrap().trim_end_matches("]").parse().ok()
}

fn part1(input: &Vec<&str>) {
  println!("Solving day 4 part 1");
  let mut guards : HashMap<u16, Guard> = HashMap::new();
  let mut current_gid : u16 = 0;
  let mut input_iter = input.into_iter();
  let mut start_end : (Option<u8>, Option<u8>) = (None, None);

  while let Some(line) = input_iter.next() {
    if line.contains("Guard") {
      let gid = Guard::get_id(line).expect("no id found");
      current_gid = gid;
      

      if !guards.contains_key(&gid) {
        let nguard = Guard::new(line);
        guards.insert(gid, nguard);
      }
    } else if line.contains("falls asleep") {
      start_end.0 = get_minutes(line);
      //guards.get_mut(&current_gid).expect("No guard matching current_gid")
    } else if line.contains("wakes up") {
      start_end.1 = get_minutes(line);
      //println!("{:?}", start_end);
      //add sleep time to guard
      guards.get_mut(&current_gid).unwrap().add_sleep_time(start_end.0.unwrap(), start_end.1.unwrap());
    }
  }
  //count
  //part 1
  {
    let longest_sleeping_guard = guards.values().max_by_key(|g| g.total_sleep_time).unwrap();
    let mut max_idx_val = (0, 0);
    for (idx, val) in longest_sleeping_guard.sleep_time.iter().enumerate() {
      if *val > max_idx_val.1 {
        max_idx_val = (idx, *val);
      }
    }
    println!("{:?}", longest_sleeping_guard);
    println!("minute with most sleeps: {} (sleeps: {}", max_idx_val.0, max_idx_val.1);
  }
  for g in guards.values_mut() {
    g.calc_max_sleep_minute();
  }

  let best = guards.values().max_by(|x, y| x.max_sleep_minute.1.cmp(&y.max_sleep_minute.1));
  println!("\nLongest sleep: {:?}", best);



}



fn part2(input: &Vec<&str>) {  
}

fn main() {
  let input = include_str!("../sorted_input").lines().collect::<Vec<&str>>();

  part1(&input);
  //part2(&input);
}
