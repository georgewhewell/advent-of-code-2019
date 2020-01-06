use std::fs;

fn parse_program(input: &str) -> Vec<i32> {
    return input
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
}

fn _execute(mem: &mut Vec<i32>, ptr: &mut usize, input: Option<i32>) -> bool {
    println!("ptr: {}, running: {:?}", ptr, mem);
    if mem.get(*ptr) == None {
        println!("pointer out of bounds- done??");
        return false;
    };
    let op = mem[*ptr];
    match op % 10 {
        1 => {
            let a;
            let b;
            match (op / 100) % 10 {
              0 => {
                a = mem[mem[*ptr + 1] as usize];
              }
              1 => {
                a = mem[*ptr + 1];
              }
              _ => { panic!("bad param mode: {} (op: {}(", (op / 100) % 10, op); }
            }
            match (op / 1000) % 10 {
              0 => {
                b = mem[mem[*ptr + 2] as usize];
              }
              1 => {
                b = mem[*ptr + 2];
              }
              _ => { panic!("bad param mode"); }
            }
            let c = mem[*ptr + 3];
            mem[c as usize] = a + b;
            *ptr += 4;
        }
        2 => {
            let a;
            let b;
            match (op / 100) % 10 {
              0 => {
                a = mem[mem[*ptr + 1] as usize];
              }
              1 => {
                a = mem[*ptr + 1];
              }
              _ => { panic!("bad param mode: {}", (op / 10) % 10); }
            }
            match (op / 1000) % 10 {
              0 => {
                  println!("pos mod");
                b = mem[mem[*ptr + 2] as usize];
              }
              1 => {
                  println!("im mod");
                b = mem[*ptr + 2];
              }
              _ => { panic!("bad param mode"); }
            }
            let c = mem.get(*ptr + 3).cloned().unwrap();
            println!("writing mult {}, {} to {}", a, b, mem[*ptr+3]);
            mem[c as usize] = a * b;
            *ptr += 4;
        }
        3 => {
            let a = mem[*ptr+1];
            match input {
              Some(i) => {
                println!("storing {} at {} (should be {})", i, a, mem[*ptr+1]);
                mem[a as usize] = i;
              }
              None => { panic!("No input for input instruction"); }
            }
            *ptr += 2;
        }
        4 => {
            let a = mem.get(mem[*ptr + 1] as usize).cloned().unwrap();
            println!("Outputting- {}", a);
            *ptr += 2;
        }
        9 => {
            return false;
        }
        _ => {
            panic!("unmatched op: {}- {}", op, op % 10);
        }
    };
    return true;
}

fn execute(mem: &mut Vec<i32>, input: Option<i32>) -> &[i32] {
    let mut ptr = 0;
    while _execute(mem, &mut ptr, input) {}
    return mem;
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
        let mut input = vec![1, 0, 0, 0, 99];
        execute(&mut input, None);
        assert_eq!(input, [2, 0, 0, 0, 99]);

        input = vec![2, 3, 0, 3, 99];
        execute(&mut input, None);
        assert_eq!(input, [2, 3, 0, 6, 99]);

        input = vec![2, 4, 4, 5, 99, 0];
        execute(&mut input, None);
        assert_eq!(input, [2, 4, 4, 5, 99, 9801]);

        input = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        execute(&mut input, None);
        assert_eq!(input, [30, 1, 1, 4, 2, 5, 6, 0, 99]);

        input = vec![1002,4,3,4,33];
        execute(&mut input, None);
        assert_eq!(input, [1002, 4, 3, 4, 99]);

        input = vec![1102, 99, 1, 4, 0];
        execute(&mut input, None);
        assert_eq!(input, [1102, 99, 1, 4, 99]);
        
        input = vec![1101,100,-1,4,0];
        execute(&mut input, None);
        assert_eq!(input, [1101, 100, -1, 4, 99]);

        input = vec![3, 0, 4, 0, 99];
        execute(&mut input, Some(99));
        assert_eq!(input, [99, 0, 4, 0, 99]);

        input = vec![3, 2, 0];
        execute(&mut input, Some(99));
        assert_eq!(input, [3, 2, 99]);

        input = vec![4, 0, 99];
        execute(&mut input, Some(99));
        assert_eq!(input, [4, 0, 99]);
    }
}

fn puzzle_1() {
    let contents = fs::read_to_string("./input.txt").unwrap();
    let mut input = parse_program(&contents.trim());

    // input[1] = 12;
    // input[2] = 2;

    let result = execute(&mut input, Some(1));
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
            execute(&mut _input, None);
            if _input[0] == 19690720 {
                println!("100 * {} + {} = {}", x, y, 100 * x + y);
                return;
            }
        }
    }
}

fn main() {
    puzzle_1();
 //   puzzle_2();
}
