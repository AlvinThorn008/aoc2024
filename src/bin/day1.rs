use std::{collections::HashMap, fs::File, io::Read};

fn main() {
    let mut file = File::open("inputs/input1.txt").unwrap();
    let mut input = String::with_capacity(15 * 1024);
    file.read_to_string(&mut input).unwrap();

    let mut nums = input.split_ascii_whitespace().map(|x| str::parse::<i32>(x).unwrap());
    
    let mut left = Vec::with_capacity(1000);
    let mut right = Vec::with_capacity(1000);

    loop {
        if let Some(l) = nums.next() {
            left.push(l);
        } else { break; }
        right.push(nums.next().unwrap());
    }

    left.sort();
    right.sort();

    part2(&left[..], &right);

}

fn part1(left: &[i32], right: &[i32]) {
    let sum = left.iter().zip(right.iter()).fold(0, |acc, (&l, &r)| {
        acc + i32::abs_diff(l, r)
    });

    println!("{}", sum);
}

fn part2(left: &[i32], right: &[i32]) {
    let mut sum = 0;
    // println!("{:?}", right);
    for &num in left.iter() {
        let count = right.iter().filter(|&&x| x == num).count();
        sum += num * count as i32;
        println!("{num} * {count}");
    }
    println!("{sum}");
}