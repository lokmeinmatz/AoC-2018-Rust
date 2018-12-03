use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;


fn get_used_tiles(rect: Rect) -> Vec<(u16, u16)> {
  let mut used_tiles = Vec::new();

  for x in rect.0..rect.0 + rect.2 {
    for y in rect.1..rect.1+rect.3 {
      used_tiles.push((x, y));
    }
  }

  used_tiles
}

fn do_overlap(rect0 : Rect, rect1 : Rect) -> bool {
  
  //x-axis
  if rect1.0 > rect0.0 + rect0.2 || rect1.0 + rect1.2 < rect0.0 {return false;}
  //y-axis
  else if rect1.1 > rect0.1 + rect0.3 || rect1.1 + rect1.3 < rect0.1 {
    return false;
  } 

  true
}

fn to_rect(line : &str) -> Rect {
  let line = line.split_whitespace()
    .enumerate()
    .filter_map(|(i, s)| {
      if i == 2 || i == 3 {return Some(s);}
      None
    }).collect::<Vec<&str>>();

    let mut rect = (0, 0, 0, 0);

    let start_coords = line[0].split(",").collect::<Vec<&str>>();

    rect.0 = start_coords[0].parse::<u16>().unwrap();

    rect.1 = start_coords[1].trim_end_matches(":").parse::<u16>().unwrap();

    let dimensions = line[1].split("x").collect::<Vec<&str>>();

    rect.2 = dimensions[0].parse::<u16>().unwrap();
    rect.3 = dimensions[1].parse::<u16>().unwrap();

    rect
}

fn part1(input: &String) {
  println!("Solving day 3 part 1");

  let mut used_tiles : HashMap<(u16, u16), u16> = HashMap::new();

  for line in input.split("\n").map(|l| l.trim()) {
    let rect = to_rect(line);

    let current_used_tiles = get_used_tiles(rect);
    for tile in current_used_tiles {
      let old_count = *used_tiles.get(&tile).unwrap_or(&0);
      used_tiles.insert(tile, old_count + 1);
    }
  }

  // all rects evaluated
  let mut total_overlapping_area = 0u32;
  for (_, count) in used_tiles {
    if count > 1 {
      total_overlapping_area += 1;
      println!("Total overlapping sqinches : {}", total_overlapping_area);
    }
  }


}

type Rect = (u16, u16, u16, u16);

fn part2(input: &String) {
  println!("Solving day 3 part 2"); 
  let mut all_rects : Vec<Rect> = Vec::with_capacity(1500);
  
  for line in input.split("\n").map(|l| l.trim()) {
    let rect = to_rect(line);
    all_rects.push(rect);
  }
  println!("{:?}", all_rects.len());

  'outer: for i in 0..all_rects.len() {
    let rect0 = all_rects[i];
    for j in 0..all_rects.len() {
      if j == i {continue;}
      let rect1 = all_rects[j];
      if do_overlap(rect0, rect1) {
        continue 'outer;
      }
    }
    //no other rect overlapped
    println!("{:?} did not overlap with any other rect", rect0);
    println!("ID: {}", i);
    break;
  }
  println!("Finished part 2");
}

fn main() {
  let mut file = File::open("input").unwrap();
  let mut input = String::new();
  file.read_to_string(&mut input).unwrap();

  //part1(&input);
  part2(&input);
}
