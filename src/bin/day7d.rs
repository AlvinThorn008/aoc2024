use std::hint::black_box;

fn main() {
    // black_box(part1());
    
    println!("Result: {}", part1());

}

pub fn part1() -> u64 {
    let data = std::fs::read_to_string("inputs/input7.txt").unwrap();

    let equations = data.lines().map(|line| {
        line.split(&[':', ' '])
            .filter(|elem| elem.len() != 0)
            .map(str::parse::<u64>).map(Result::unwrap)
    });

    let mut sum = 0;

    let mut eq_buf: Vec<u64> = Vec::with_capacity(20);

    for eq in equations {
        eq_buf.extend(eq);
        let eq = &eq_buf;
        println!("{eq:?}");
        let ops = eq.len() as u32 - 2; // Number of operators required
        // Upper bound permutation: (high - 1) is highest permutation
        // in lexiographical order
        let high = 1u16 << ops; 
        
        for perm in 0..high {
            let mut result = eq[1]; // First operand
            let mut ops1 = ops;
            let mut opi = 2; // 2nd operand index
            loop {
                ops1 -= 1;
                match (perm >> ops1) & 1 {
                    0 => result += eq[opi],
                    1 => result *= eq[opi],
                    _ => unreachable!("perm >> ops1 should shift `first`(perm is an ops-bit number) bit to LSB")
                }
                opi += 1;
                if opi == eq.len() { break; }
            }
            println!("Perm {:#w$b} {}", perm, result == eq[0], w = ops as usize + 2);
            if result == eq[0] { sum += eq[0]; break; }
        }
        eq_buf.clear();
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

  where ADD = 0, MUL = 0
  
  If we generate the permutations up to the highest number of operators required (M), it is equivalent to the range 0..2^M (right exclusive).


 */