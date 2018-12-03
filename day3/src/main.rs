use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn get_used_tiles(rect: &Rect) -> Vec<(u16, u16)> {
  let mut used_tiles = Vec::new();

  for x in rect.x..rect.x + rect.w {
    for y in rect.y..rect.y+rect.h {
      used_tiles.push((x, y));
    }
  }

  used_tiles
}

fn do_overlap(rect0 : &Rect, rect1 : &Rect) -> bool {
  
  //x-axis
  if rect1.x >= rect0.x + rect0.w || rect1.x + rect1.w <= rect0.x {return false;}
  //y-axis
  else if rect1.y >= rect0.y + rect0.h || rect1.y + rect1.h <= rect0.y {
    return false;
  } 

  true
}

#[test]
fn test_overlap() {
  let a = Rect::new(0, 0, 5, 5, 0);
  let b = Rect::new(5, 0, 5, 5, 0);
  let c = Rect::new(0, 5, 5, 5, 0);
  let d = Rect::new(0, 0, 10, 5, 0);
  assert_eq!(do_overlap(&a, &b), false);
  assert_eq!(do_overlap(&a, &c), false);
  assert_eq!(do_overlap(&a, &d), true);
}

fn to_rect(line : &str) -> Rect {
  let line = line.split_whitespace().collect::<Vec<&str>>();

  let id = line[0].trim_start_matches("#").parse::<u16>().unwrap();
  
  let mut rect = Rect::new(0, 0, 0, 0, id);

  let start_coords = line[2].split(",").collect::<Vec<&str>>();

  rect.x = start_coords[0].parse::<u16>().unwrap();

  rect.y = start_coords[1].trim_end_matches(":").parse::<u16>().unwrap();

  let dimensions = line[3].split("x").collect::<Vec<&str>>();

  rect.w = dimensions[0].parse::<u16>().unwrap();
  rect.h = dimensions[1].parse::<u16>().unwrap();

  rect
}

fn part1(all_rects: &Vec<Rect>, used_tiles: &HashMap<(u16, u16), u16>) {
  println!("Solving day 3 part 1");

  

  // all rects evaluated
  let mut total_overlapping_area = 0u32;
  for (_, count) in used_tiles {
    if *count > 1 {
      total_overlapping_area += 1;
      println!("Total overlapping sqinches : {}", total_overlapping_area);
    }
  }


}

//type Rect = (u16, u16, u16, u16);
#[derive(Debug)]
struct Rect {
  x: u16,
  y: u16,
  w: u16,
  h: u16,
  id: u16
}

impl Rect {
    pub fn new(x: u16, y: u16, w: u16, h: u16, id: u16) -> Rect {
      Rect {x, y, w, h, id}
    }
}

impl PartialEq for Rect {
    fn eq(&self, other: &Rect) -> bool {
      self.id == other.id
    }
}

fn part2(all_rects: &Vec<Rect>, used_tiles: &HashMap<(u16, u16), u16>) {

  println!("Solving day 3 part 2"); 
  for rect in all_rects {
    let single = get_used_tiles(&rect).into_iter().all(|t| *used_tiles.get(&t).unwrap() == 1);

    if single {
      println!("Found {:?}", rect);
      break;
    }
  }
  println!("Finished part 2");
}

fn main() {
  let mut file = File::open("input").unwrap();
  let mut input = String::new();
  file.read_to_string(&mut input).unwrap();


  let mut used_tiles : HashMap<(u16, u16), u16> = HashMap::new();
  let mut all_rects = Vec::new();
  for line in input.split("\n").map(|l| l.trim()) {
    let rect = to_rect(line);

    let current_used_tiles = get_used_tiles(&rect);

    all_rects.push(rect);
    for tile in current_used_tiles {
      let old_count = *used_tiles.get(&tile).unwrap_or(&0);
      used_tiles.insert(tile, old_count + 1);
    }
  }

  //part1(&input);
  part2(&all_rects, &used_tiles);
}
