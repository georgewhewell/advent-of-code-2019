use std::fs;

fn parse_program(input: &str) -> Vec<usize> {
  return input.split(",").map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>()
}

fn execute(mut input: Vec<usize>) -> Vec<usize> {
  let mut ptr: usize = 0;
  let mut op: usize = input[ptr];
  while op != 99 {
    let a = input.get(input[ptr+1]).cloned().unwrap();
    let b = input.get(input[ptr+2]).cloned().unwrap();
    let c = input.get(ptr+3).cloned().unwrap();
    match op {
      1 => { input[c] = a + b; }
      2 => { input[c] = a * b; }
      _ => { }
    };
    ptr += 4;
    op = input.get(ptr).cloned().unwrap();
  }
  input
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
      assert_eq!(parse_program("1,2,3"), [1, 2, 3]);
    }

    #[test]
    fn test_execute() {
      assert_eq!(execute(vec![1,0,0,0,99]), [2,0,0,0,99]);
      assert_eq!(execute(vec![2,3,0,3,99]), [2,3,0,6,99]);
      assert_eq!(execute(vec![2,4,4,5,99,0]), [2,4,4,5,99,9801]);
      assert_eq!(execute(vec![1,1,1,4,99,5,6,0,99]), [30,1,1,4,2,5,6,0,99]);
    }
}

fn puzzle_1() {
    let contents = fs::read_to_string("./input.txt").unwrap();
    let mut input = parse_program(&contents.trim());

    input[1] = 12;
    input[2] = 2;
    
    let result = execute(input);
    println!("pos0: {}", result[0]);
}

fn puzzle_2() {
    let contents = fs::read_to_string("./input.txt").unwrap();
    let input = parse_program(&contents.trim());

    for x in 0..100 {
      for y in 0..100 {
        let mut _input = input.clone();
        _input[1] = x;
        _input[2] = y;
        let result = execute(_input);
        if result[0] == 19690720 {
            println!("100 * {} + {} = {}", x, y, 100*x+y);
            break;
        }
      }
    }
}

fn main() {
    puzzle_1();
    puzzle_2();
}
