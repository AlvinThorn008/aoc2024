use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("inputs/input11.txt").unwrap();
    let input = input.trim();
    let numbers: Vec<u64> = input.split(' ').map(str::parse::<u64>).map(Result::unwrap).collect();
    let mut cache: HashMap<(u64, u8), u64> = HashMap::new();

    let num_stones_1: u64 = count(&numbers, 25, &mut cache);
    let num_stones_2: u64 = count(&numbers, 75, &mut cache); 

    println!("Part 1: {num_stones_1}\nPart 2: {num_stones_2}");
}

fn count(stones: &[u64], blinks: u8, cache: &mut HashMap<(u64, u8), u64>) -> u64 {
    stones.into_iter()
    .map(|stone| count_stones(*stone, blinks, cache))
    .sum()
}

fn count_stones(stone: u64, blinks: u8, cache: &mut HashMap<(u64, u8), u64>) -> u64 {
    if blinks == 0 { return 1; }
    else if let Some(count) = cache.get(&(stone, blinks)) {
        return *count;
    }

    let count = match stone {
        0 => count_stones(1, blinks - 1, cache),
        n => {
            match n.ilog10() + 1 {
                a if a % 2 == 0 => {
                    let base = 10u64.pow(a/2);
                    count_stones(stone / base, blinks - 1, cache) +
                    count_stones(stone % base, blinks - 1, cache)
                }
                _ => count_stones(stone * 2024, blinks - 1, cache)
            }
        }
    };
    cache.insert((stone, blinks), count);

    count
}

/*
872027 227 18 9760 0 4 67716 9245696

872 27 -> 872*1024 2 7 -> 

count([a1, a2, ..., an], M) = count([a1], M) + count([a2], M) + ... + count([an], M)

count([a1], M) = case a1
    0               => count([1], M-1),
    len(a1) is even => count([a1.left, a1.right], M-1),
    _               => count([a1 * 1024], M-1)


 */