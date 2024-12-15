use std::i32;

fn main() {
    let data = std::fs::read_to_string("inputs/input14-processed.txt").unwrap();
    let numbers: Vec<i32> = data
        .split(&[',', '\n'])
        .map(|n| n.parse().unwrap())
        .collect();

    let p1 = part1(&numbers);
    println!("Part 1: {p1}");

    let p2 = part2(&numbers);
    println!("Part 2: {p2}");
}

fn part1(numbers: &[i32]) -> u64 {
    let mut quadrants = [0u64; 4];
    for robot in numbers.chunks(4) {
        let (x, y, vx, vy) = (robot[0], robot[1], robot[2], robot[3]);

        let nx = (x + 100 * vx).rem_euclid(101);
        let ny = (y + 100 * vy).rem_euclid(103);

        if nx == 50 || ny == 51 { continue; }

        let quadrant = (ny < 51) as usize * 2 + (nx > 50) as usize;

        quadrants[quadrant] += 1;
    }

    quadrants.iter().product()
}

fn part2(numbers: &[i32]) -> i32 {
    let mut min_var_x = (0, i32::MAX);
    let mut min_var_y = (0, i32::MAX);
    let mut xs = Vec::with_capacity(103);
    let mut ys = Vec::with_capacity(103);
    for t in 0..103 {
        for robot in numbers.chunks(4) {
            let (x, y, vx, vy) = (robot[0], robot[1], robot[2], robot[3]);
    
            let nx = (x + t * vx).rem_euclid(101);
            let ny = (y + t * vy).rem_euclid(103);
            xs.push(nx); ys.push(ny);
        }
        xs.truncate(101);
        let xs_mean = xs.iter().sum::<i32>() / xs.len() as i32;
        let ys_mean = ys.iter().sum::<i32>() / ys.len() as i32;

        let varx = xs.iter().map(|&x| i32::pow(x - xs_mean, 2)).sum::<i32>() / xs.len() as i32;
        let vary = ys.iter().map(|&y| i32::pow(y - ys_mean, 2)).sum::<i32>() / ys.len() as i32;

        if varx < min_var_x.1 { min_var_x = (t, varx); }
        if vary < min_var_y.1 { min_var_y = (t, vary); }

        xs.clear(); ys.clear();
    }

    let k = extended_euler(101, -103) * (min_var_y.0 - min_var_x.0);
    let a = (103 - k).div_euclid(103);

    101 * (k + a * 103) + 4
}

fn extended_euler(a: i32, b: i32) -> i32 {
    let (mut or, mut r, mut os, mut s) = (a, b, 1, 0);

    while r != 0 {
        let q = or.div_euclid(r);
        (or, r) = (r, or - q * r);
        (os, s) = (s, os - q * s);
    }

    os
}

