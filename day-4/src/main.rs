fn is_valid(guess: &str) -> bool {
  let gb = guess.as_bytes();
  let mut curr = gb[0];
  let mut next;

  for c in 1..6 {
    next = gb[c];
    if next < curr {
        return false;
    };
    curr = next;
  }

  let mut cnt = 0;
  curr = gb[0];
  for c in 1..6 {
     if gb[c] == curr {
        cnt += 1;
     } else {
        if cnt == 1 {
          return true;
        }
        curr = gb[c];
        cnt = 0;
     }
  }
  
  return cnt == 1;
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

    #[test]
    fn test_no_larger(){
        assert_eq!(is_valid("112233"), true);
        assert_eq!(is_valid("123444"), false);
        assert_eq!(is_valid("111122"), true);
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
