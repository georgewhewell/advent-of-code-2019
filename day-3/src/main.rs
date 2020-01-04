use std::fs;
use std::cmp::{min, max};

fn parse_path(input: &str) -> Vec<(char, i32)> {
  return input.split(",").map(|vec|
    (vec.chars().next().unwrap(), vec[1..].parse::<i32>().unwrap())
  ).collect::<Vec<(char, i32)>>()
}

fn get_coords(input: Vec<(char, i32)>) -> Vec<(i32, i32)> {
  let [mut x, mut y] = [0, 0];
  return input.iter().flat_map(|(dir, length)|
    match dir {
      'R' => { x += length; ((x-length+1)..x+1).map(|x1| (x1, y)).collect() }
      'L' => { x -= length; (x..(x+length)).rev().map(|x1| (x1, y)).collect() }
      'D' => { y += length; ((y-length+1)..y+1).map(|y1| (x, y1)).collect() }
      'U' => { y -= length; (y..(y+length)).rev().map(|y1| (x, y1)).collect() }
      _ => { vec![(x, y)] }
    }
  ).collect();
}

fn get_intersections(p1: &Vec<(i32, i32)>, p2: &Vec<(i32, i32)>) -> Vec<((i32, i32), (i32, i32))>{
  let mut i = Vec::new();
  for (l1, c) in p1.iter().enumerate() {
    for (l2, d) in p2.iter().enumerate() {
      if (c == d){
        println!("pushing: {:?} {:?} ({})", c, d, l1 + l2 + 2);
        i.push((c.clone(), (l1 as i32, l2 as i32)));
      }
    }
  }
  return i;
}

fn get_distance((x, y): (i32, i32)) -> i32 {
   return x.abs() + y.abs(); 
}

fn get_nearest_intersection(p1: &Vec<(i32, i32)>, p2: &Vec<(i32, i32)>) -> (i32, i32) {
  let intersections = get_intersections(p1, p2);
  let mut smallest_inter = (-1, -1);
  let mut smallest_dist = 9999999;
  for (i, (d1, d2)) in get_intersections(p1, p2){
    println!("checking: {:?}", i);
    if get_distance(i) < smallest_dist {
      smallest_inter = i;
      smallest_dist = get_distance(i);
    };
  }
  return smallest_inter;
}

fn get_shortest_intersection(p1: &Vec<(i32, i32)>, p2: &Vec<(i32, i32)>) -> i32 {
  let intersections = get_intersections(p1, p2);
  let mut smallest_inter = (-1, -1);
  let mut smallest_dist = 9999999;
  for (i, (d1, d2)) in get_intersections(p1, p2){
    println!("checking: {:?}", i);
    if (d1 + d2) < smallest_dist {
      smallest_inter = i;
      smallest_dist = d1 + d2;
    };
  }
  return smallest_dist + 2;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_path(){
      assert_eq!(parse_path("R1,L22"), [('R', 1), ('L', 22)]);
    }

    #[test]
    fn test_get_coords(){
      assert_eq!(get_coords(vec![('R', 2), ('L', 2)]), vec![(1, 0), (2, 0), (1, 0), (0, 0)]);
      assert_eq!(get_coords(vec![('U', 2), ('D', 2)]), vec![(0, -1), (0, -2), (0, -1), (0, 0)]);
      assert_eq!(get_coords(vec![('U', 2)]), vec![(0, -1), (0, -2)]);
      assert_eq!(get_coords(vec![('D', 2)]), vec![(0, 1), (0, 2)]);
    }

    #[test]
    fn test_get_intersections(){
      let p1 = vec![(0, 1), (1, 1)];
      let p2 = vec![(1, 1)];
      assert_eq!(get_intersections(&p1, &p2), vec![((1, 1), (1, 0))]);
    }

    #[test]
    fn test_get_distance(){
      assert_eq!(get_distance((5, 5)), 10);
      assert_eq!(get_distance((-5, 5)), 10);
    }

    #[test]
    fn test_get_nearest_intersection_simple(){
      let p1 = get_coords(parse_path("R8,U5,L5,D3"));
      let p2 = get_coords(parse_path("U7,R6,D4,L4"));
      assert_eq!(get_distance(get_nearest_intersection(&p1, &p2)), 6);
    }

    #[test]
    fn test_get_nearest_intersection(){
      let p1 = get_coords(parse_path("R75,D30,R83,U83,L12,D49,R71,U7,L72"));
      let p2 = get_coords(parse_path("U62,R66,U55,R34,D71,R55,D58,R83"));
      assert_eq!(get_distance(get_nearest_intersection(&p1, &p2)), 159);

      let p3 = get_coords(parse_path("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"));
      let p4 = get_coords(parse_path("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"));
      assert_eq!(get_distance(get_nearest_intersection(&p3, &p4)), 135);
    }

    #[test]
    fn test_get_shortest_intersection(){
      let p1 = get_coords(parse_path("R75,D30,R83,U83,L12,D49,R71,U7,L72"));
      let p2 = get_coords(parse_path("U62,R66,U55,R34,D71,R55,D58,R83"));
      assert_eq!(get_shortest_intersection(&p1, &p2), 610);

      let p3 = get_coords(parse_path("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"));
      let p4 = get_coords(parse_path("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"));
      assert_eq!(get_shortest_intersection(&p3, &p4), 410);
    }
}

fn puzzle_1(){
    let p1 = fs::read_to_string("./input.txt").unwrap();
    let p2 = fs::read_to_string("./input-2.txt").unwrap();
    let result = get_distance(
        get_nearest_intersection(
            &get_coords(parse_path(&p1.trim())),
            &get_coords(parse_path(&p2.trim()))
        )
    );
    println!("puzzle 1: {}", result);
}

fn puzzle_2(){
    let p1 = fs::read_to_string("./input.txt").unwrap();
    let p2 = fs::read_to_string("./input-2.txt").unwrap();
    let result = get_shortest_intersection(
        &get_coords(parse_path(&p1.trim())),
        &get_coords(parse_path(&p2.trim()))
    );
    println!("puzzle 2: {}", result);
}

fn main() {
    puzzle_2();
    println!("Hello, world!");
}
