use std::fs;

fn parse_program(input: &str) -> Vec<i32> {
    return input
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
}

enum Dir {
 Read,
 Write,
 Skip,
}

fn op_arity(op: usize) -> [Dir; 3] {
    return match op % 10 {
        1 => [Dir::Read, Dir::Read, Dir::Write],
        2 => [Dir::Read, Dir::Read, Dir::Write],
        3 => [Dir::Write, Dir::Skip, Dir::Skip],
        4 => [Dir::Read, Dir::Skip, Dir::Skip],
        5 => [Dir::Read, Dir::Read, Dir::Skip],
        6 => [Dir::Read, Dir::Read, Dir::Skip],
        7 => [Dir::Read, Dir::Read, Dir::Write],
        8 => [Dir::Read, Dir::Read, Dir::Write],
        9 => [Dir::Skip, Dir::Skip, Dir::Skip],
        _ => panic!("Bad opcode: {}", op),
    };
}

struct VMState {
    ptr: usize,
    input: Option<i32>,
    output: Option<i32>,
    regs: [i32; 3],
}

fn _fetch(state: &mut VMState, mem: &Vec<i32>) {
    let op = mem[state.ptr] as usize;
    state.ptr += 1;
    let mut n = op.clone() / 10;
    for (i, dir) in op_arity(op).iter().enumerate() {
        n /= 10;
        match dir {
          Dir::Read => {
            if n % 10 == 0 {
                println!("position mode");
                state.regs[i] = mem[mem[state.ptr] as usize];
                println!("was: {}", state.regs[i]);
            } else {
                println!("immediate mode");
                state.regs[i] = mem[state.ptr];
            };
            state.ptr += 1;
          }
          Dir::Write => {
            state.regs[i] = mem[state.ptr];
            state.ptr += 1;
          }
          Dir::Skip => { }
        }
    }
}

fn _execute(state: &mut VMState, mem: &mut Vec<i32>) -> bool {
    println!("ptr: {}, running: {:?}", state.ptr, mem);
    let op = mem[state.ptr] as usize;
    _fetch(state, mem);
    match op % 10 {
        1 => {
            println!("adding: {:?}", state.regs);
            mem[state.regs[2] as usize] = state.regs[0] + state.regs[1];
        }
        2 => {
            mem[state.regs[2] as usize] = state.regs[0] * state.regs[1];
        }
        3 => {
            match state.input {
                Some(i) => {
                    // println!("storing {} at {} (should be {})", i, a, mem[*ptr+1]);
                    mem[state.regs[0] as usize] = i;
                }
                None => {
                    panic!("No input for input instruction");
                }
            }
        }
        4 => {
            state.output = Some(state.regs[0]);
            println!("Outputting- {:?}", state.output);
        }
        5 => {
            if state.regs[0] != 0 {
                println!("Jumping to {}", state.regs[1]);
                state.ptr = state.regs[1] as usize;
            }
        }
        6 => {
            if state.regs[0] == 0 {
                println!("Is Zero, jump to {}", state.regs[1]);
                state.ptr = state.regs[1] as usize;
            }
        }
        7 => {
            if state.regs[0] < state.regs[1] {
                mem[state.regs[2] as usize] == 1;
            } else {
                mem[state.regs[2] as usize] == 0;
            }
        }
        8 => {
            if state.regs[0] == state.regs[1] {
                mem[state.regs[2] as usize] == 1;
            } else {
                mem[state.regs[2] as usize] == 0;
            }
        }
        9 => {
            return false;
        }
        _ => {
            panic!("unmatched op: {}", op);
        }
    };
    return true;
}

fn execute(mem: &mut Vec<i32>, input: Option<i32>) -> (Option<i32>, &[i32]) {
    let mut state = VMState {
        ptr: 0,
        input: input,
        output: None,
        regs: [0, 0, 0],
    };
    while _execute(&mut state, mem) {}
    return (state.output, mem);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(parse_program("1,2,3"), [1, 2, 3]);
    }

    #[test]
    fn test_simple_jmp_execute_im() {
        let mut input = vec![1105, 1, 4, 0, 99];
        let (mut output, _) = execute(&mut input, Some(8));
        assert_eq!(output, None);
    }

    #[test]
    fn test_simple_jmp_execute_pos() {
        let mut input = vec![5, 1, 3, 4, 99];
        let (mut output, _) = execute(&mut input, Some(8));
        assert_eq!(output, None);
    }

    #[test]
    fn test_simple_jmp_false_execute_im() {
        let mut input = vec![1106, 0, 4, 0, 99];
        let (mut output, _) = execute(&mut input, Some(8));
        assert_eq!(output, None);
    }

    #[test]
    fn test_simple_jmp_false_execute_pos() {
        let mut input = vec![6, 0, 5, 99, 1, 99, 3, 4, 99];
        let (mut output, _) = execute(&mut input, Some(8));
        assert_eq!(output, None);
    }

    #[test]
    fn test_simple_jmp_lt_execute_im() {
        let mut input = vec![11106, 0, 4, 0, 99];
        let (mut output, _) = execute(&mut input, Some(8));
        assert_eq!(output, None);
    }

    #[test]
    fn test_simple_jmp_lt_execute_pos() {
        let mut input = vec![7, 0, 3, 4, 99];
        let (mut output, _) = execute(&mut input, Some(8));
        assert_eq!(output, None);
    }

    #[test]
    fn test_simple_jmp_eq_execute_im() {
        let mut input = vec![8, 1, 1, 5, 104, 0, 99];
        let (mut output, _) = execute(&mut input, Some(8));
        assert_eq!(output, Some(5));
    }

    #[test]
    fn test_simple_jmp_eq_execute_pos() {
        let mut input = vec![11108, 2, 3, 3, 99];
        let (mut output, _) = execute(&mut input, Some(8));
        assert_eq!(output, None);
    }

    #[test]
    fn test_jmp_execute() {
        let mut input = vec![3,9,8,9,10,9,4,9,99,-1,8];
        let (mut output, _) = execute(&mut input, Some(8));
        assert_eq!(output, Some(1));
        input = vec![3,9,8,9,10,9,4,9,99,-1,8];
        let (mut output, _) = execute(&mut input, Some(9));
        assert_eq!(output, Some(0));
    }

    #[test]
    fn test_jmp_execute_2() {
        let mut input = vec![3,9,7,9,10,9,4,9,99,-1,8];
        let (output, _) = execute(&mut input, Some(5));
        assert_eq!(output, Some(1));
        input = vec![3,9,7,9,10,9,4,9,99,-1,8];
        let (output, _) = execute(&mut input, Some(9));
        assert_eq!(output, Some(0));
    }

    #[test]
    fn test_jmp_execute_3() {
        let mut input = vec![3,3,1108,-1,8,3,4,3,99];
        let (output, _) = execute(&mut input, Some(8));
        assert_eq!(output, Some(1));
        input = vec![3,3,1108,-1,8,3,4,3,99];
        let (output, _) = execute(&mut input, Some(9));
        assert_eq!(output, Some(0));
    }

    #[test]
    fn test_jmp_execute_4() {
        let mut input = vec![3,3,1107,-1,8,3,4,3,99];
        let (output, _) = execute(&mut input, Some(5));
        assert_eq!(output, Some(1));
        input = vec![3,3,1108,-1,8,3,4,3,99];
        let (output, _) = execute(&mut input, Some(9));
        assert_eq!(output, Some(0));
    }

    #[test]
    fn test_jmp_execute_5() {
        let mut input = vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9];
        let (output, _) = execute(&mut input, Some(0));
        assert_eq!(output, Some(0));
        input = vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9];
        let (output, _) = execute(&mut input, Some(9));
        assert_eq!(output, Some(1));
    }

    #[test]
    fn test_jmp_execute_6() {
        let mut input = vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1];
        let (output, _) = execute(&mut input, Some(0));
        assert_eq!(output, Some(0));
        input = vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1];
        let (output, _) = execute(&mut input, Some(9));
        assert_eq!(output, Some(1));
    }

    #[test]
    fn test_jmp_execute_7() {
        let mut input = vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
        1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
        999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99];
        let (output, _) = execute(&mut input, Some(1));
        assert_eq!(output, Some(999));
        input = vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
        1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
        999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99];
        let (output, _) = execute(&mut input, Some(8));
        assert_eq!(output, Some(1000));
        input = vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
        1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
        999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99];
        let (output, _) = execute(&mut input, Some(9));
        assert_eq!(output, Some(1001));
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

        input = vec![1002, 4, 3, 4, 33];
        execute(&mut input, None);
        assert_eq!(input, [1002, 4, 3, 4, 99]);

        input = vec![1102, 99, 1, 4, 0];
        execute(&mut input, None);
        assert_eq!(input, [1102, 99, 1, 4, 99]);

        input = vec![1101, 100, -1, 4, 0];
        execute(&mut input, None);
        assert_eq!(input, [1101, 100, -1, 4, 99]);

        input = vec![3, 0, 4, 0, 99];
        execute(&mut input, Some(99));
        assert_eq!(input, [99, 0, 4, 0, 99]);

        input = vec![3, 2, 0];
        execute(&mut input, Some(99));
        assert_eq!(input, [3, 2, 99]);

        input = vec![4, 0, 99];
        let output = execute(&mut input, Some(99));
        assert_eq!(input, [4, 0, 99]);
    }
}

fn puzzle_1() {
    let contents = fs::read_to_string("./input.txt").unwrap();
    let mut input = parse_program(&contents.trim());
    let (output, mem) = execute(&mut input, Some(5));
}

fn main() {
    puzzle_1();
    //   puzzle_2();
}
