mod helper;

use helper::*;
use structure::{Mat2, inv, mul};

fn main() {
    let data = std::fs::read_to_string("inputs/input13.txt").unwrap();
    
    // Part 1
    let min_tokens = find_min_tokens(&data, |x| x);
    println!("Part 1: {min_tokens}");
    // Part 2
    let min_tokens = find_min_tokens(&data, |x| (x.0 + 10_000_000_000_000.0, x.1 + 10_000_000_000_000.0));
    println!("Part 2: {min_tokens}");
}

fn find_min_tokens(data: &str, prize_func: impl Fn((f64, f64)) -> (f64, f64)) -> i64 {
    let mut lines = data.lines();
    let mut min_tokens = 0;

    while let Some(line) = lines.next() {
        if line.len() < 20 { continue; }
        let vec1 = btn_to_vec(line);
        let vec2 = btn_to_vec(lines.next().unwrap());
        let prize = prize_func(prize_to_vec(lines.next().unwrap()));
        let mat = Mat2(vec1.0, vec2.0, vec1.1, vec2.1);
        // Ok, this won't work for more general inputs. Luckily, it doesn't matter hehe
        if vec1.0 / vec1.1 == vec2.0 / vec2.1 { // vectors on same line
            let s = prize.0 / vec1.0;
            if (s == prize.1 / vec1.1) && (s.round() - s).abs() <= 0.0001 { // prize can be reached
                min_tokens += s.round() as i64; // Least tokens spent with button B
                continue;
            }
        } else if let Some(tokens) = solve(&mat, prize) { // a solution exists
            min_tokens += 3 * tokens.0.round() as i64 + tokens.1.round() as i64;
        }
    }

    min_tokens
}

fn solve(mat: &Mat2, res: (f64, f64)) -> Option<(f64, f64)> {
    let inv = inv(mat)?;
    
    let tokens = mul(&inv, res);
    if tokens.0 < 0.0 || tokens.1 < 0.0 || (tokens.0.round() - tokens.0).abs() > 0.0001 || (tokens.1.round() - tokens.1).abs() > 0.0001 { 
        None
    }
    else { Some((tokens.0, tokens.1)) }
}

/*
system
[a1 b1 p1]
[a2 b2 p2]
 */
fn solve_gauss(system: &mut [[i64; 3];2]) -> Option<(i64, i64)> {
    let b_coef = system[1][0]*system[0][1] - system[0][0]*system[1][1];
    let b_constant = system[1][0]*system[0][2] - system[0][0]*system[1][2];
    let a_constant = system[0][2] - b_coef * system[0][1];

    if b_constant % b_coef != 0 { return None; }
    let b = b_constant / b_coef;
    if a_constant % system[0][0] != 0 { return None; }
    let a = a_constant / system[0][0];
    if b < 0 || a < 0 { return None; }
    
    Some((a, b))
}