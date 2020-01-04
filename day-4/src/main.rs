fn is_valid(guess: &str) -> bool {
  let gb = guess.as_bytes();
  let mut curr = gb[0];
  let mut next;
  let mut has_repeat = false;
  for c in 1..6 {
    next = gb[c].clone();
    if next < curr {
        return false;
    };
    if next == curr {
        has_repeat = true;
    };
    curr = next;
  }
  return has_repeat;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order(){
        assert_eq!(is_valid("111111"), true);
        assert_eq!(is_valid("223450"), false);
        assert_eq!(is_valid("923446"), false);
    }

    #[test]
    fn test_repeat(){
        assert_eq!(is_valid("123789"), false);
    }
}

fn puzzle_1(start: u32, end: u32) -> u32 {
  let mut c = 0;
  for guess in start..end {
    if is_valid(&guess.to_string()){
      c += 1;
    };
  }

  return c;
}

fn main() {
    println!("puzzle 1: {}", puzzle_1(123257, 647015));
    println!("Hello, world!");
}
