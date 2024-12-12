fn main() {
    let data = std::fs::read_to_string("inputs/input7.txt").unwrap();
    // TODO: Reduce allocs by flattening structure
    let equations: Vec<Vec<u64>> = data.lines().map(|line| {
        line.split(&[':', ' '])
            .filter(|elem| elem.len() != 0)
            .map(str::parse::<u64>).map(Result::unwrap)
            .collect()
    }).collect();
    println!("Answer is: {}", part1_2(&equations));
}

use std::{hint::black_box, time::Instant};

// Run function and return result with seconds duration
pub fn time<F, T>(f: F) -> (T, f64)
  where F: FnOnce() -> T {
  let start = Instant::now();
  let res = f();
  let end = Instant::now();

  let runtime_nanos = (end - start).as_nanos();
//   .expect("Benchmark iter took greater than 2^63 nanoseconds");
  let runtime_secs = runtime_nanos as f64 / 1_000_000_000.0;
  (res, runtime_secs)
}

fn part1_2(equations: &[Vec<u64>]) -> u64 {
    let mut sum = 0;

    fn possible(equation: &[u64], result: u64, idx: usize) -> bool {
        if idx == equation.len() { return equation[0] == result; }

        if possible(equation, result + equation[idx], idx + 1) { true }
        else { possible(equation, result * equation[idx], idx + 1) }
    }

    for eq in equations.into_iter() {
        if possible(&eq, eq[1], 2) { sum += eq[0]; }
    }

    sum
}

fn part1_3(equations: &[Vec<u64>]) -> u64 {
    let mut sum = 0;
    let mut stack = Vec::with_capacity(20);

    for eq in equations.iter() {
        stack.push((2u8, eq[1]));
        let eq_len = eq.len() - 1;
        let target = eq[0];

        while let Some((idx, res)) = stack.pop() {
            if idx == eq_len as u8 {
                if res == target { sum += target; break; }
                continue;
            }
            let operand = eq[idx as usize];
            // Could prune maybe
            stack.push((idx + 1, res + operand));
            stack.push((idx + 1, res * operand));
        };
        stack.clear();
    }

    sum
}

fn part1(equations: &[Vec<u64>]) -> u64 {
    let mut sum = 0;

    for eq in equations {
        println!("{eq:?}");
        let ops = eq.len() as u32 - 2; // Number of operators required
        // Upper bound permutation: (high - 1) is highest permutation
        // in lexiographical order
        let high = 1u16 << ops;
        let mut result = eq[1];
        'perm_loop: for perm in 0..high {
            let mut ops1 = ops;
            for opi in 2..eq.len() {
                ops1 -= 1;
                match (perm >> ops1) & 1 {
                    0 => result += eq[opi],
                    1 => result *= eq[opi],
                    _ => unreachable!("perm >> ops1 should shift `first`(perm is an ops-bit number) bit to LSB")
                }
                if result > eq[0] { continue 'perm_loop; }
            }
            if result == eq[0] { sum += eq[0]; break; }
        }
    }

    sum
}

fn part2(equations: &[Vec<u64>]) -> u64 {
    let mut sum = 0;

    for eq in equations {
        let ops = eq.len() as u32 - 2; // Number of operators required
        // Upper bound permutation: (high - 1) is highest permutation
        // in lexiographical order
        let high = 3u32.pow(ops); 
        
        for mut perm in 0..high {
            let mut result = eq[1]; // First operand
            let mut place_value = high;
            for opi in 2..eq.len() { // opi - operand index - starts at 2. result is the first one
                place_value /= 3;
                let value = perm / place_value;
                match value {
                    0 => result += eq[opi],
                    1 => result *= eq[opi],
                    _ => result = result * 10u64.pow(eq[opi].ilog10() + 1) + eq[opi],
                    // _ => unreachable!("perm >> ops1 should shift `first`(perm is an ops-bit number) bit to LSB")
                }
                perm -= value * place_value;
            }
            if result == eq[0] { sum += eq[0]; break; }
        }
    }

    sum
}
/*
  = Part
  Each permutation can be represented by a binary number since there are only two operations
  
  For example, all ordering for OPS = 2 is given by numbers 0 to 3
  0 - 0 0 [ADD, ADD]
  1 - 0 1 [ADD, MUL]
  2 - 1 0 [MUL, ADD]
  3 - 1 1 [MUL, MUL]

  0 0 0 0
  0 0 0 1
  0 0 0 

 If we generate the permutations up to the highest number of operators required (M), it is equivalent to the range 0..2^M (right exclusive).

 
 


 for eq in eqs:
    stack = [eq.operands[0]]
    result = eq.operands[0]

    i = 1
    while i < eq.operands.len():
        result += eq.operands[i]
        stack.push(result)
        

    for oper in eq.operands:
        result += oper
        stack.push(result)

    i = eq.operands.len() - 1

    elem = stack.pop()
    while !stack.empty:
        if i == eq.operands.len() - 1:
            if stack.pop() == eq.result:
                break
            else:
                stack.push(stack.pop() * eq.operands[i])
                i -= 1;
        else:
            stack.push(stack.pop() + eq.operands[i])
            i += 1


    S = sum 4 9 10 3  25

    
        4
       / 
 */