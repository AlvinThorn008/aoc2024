use std::{cmp::Ordering, collections::HashSet};

fn main() {
    let mut rules: HashSet<(i32, i32)> = HashSet::new();
    let mut updates = vec![];
    let ans;

    let data = std::fs::read_to_string("inputs/input5.txt").unwrap();
    parse(&data, &mut rules, &mut updates);
    ans = solve(&rules, &updates);
    println!("{}, {}", ans.0, ans.1);
}

fn solve(rules: &HashSet<(i32, i32)>, updates: &Vec<Vec<i32>>) -> (i32, i32) {
    let mut sum1 = 0;
    let mut sum2 = 0;

    for update in updates {
        if is_valid(&update, &rules) {
            let mid = (update.len() - 1) / 2;
            sum1 += update[mid];
        } else {
            sum2 += part2(update, rules);
        }
    }

    return (sum1, sum2);
}

fn part2(invalid_update: &[i32], rules: &HashSet<(i32, i32)>) -> i32 {
    let mut val = 0;
    let mut sorted = invalid_update.to_vec();

    sorted.sort_by(|a, b| {
        if rules.contains(&(*a, *b)) {
            return Ordering::Less;
        }

        Ordering::Greater
    });

    val += sorted[(sorted.len() - 1) / 2];

    return val;
}

fn parse(data: &str, rules: &mut HashSet<(i32, i32)>, updates: &mut Vec<Vec<i32>>,) {
    let mut lines = data.lines();
    rules.extend(
        lines.by_ref()
        .take_while(|&line| line != "")
        .map(|line| {
            let mut elems = line.split('|').map(|elem| elem.parse().unwrap());
            (elems.next().unwrap(), elems.next().unwrap())
        })
    );
    updates.extend(
        lines.map(|line| 
            line.split(',')
                .map(str::parse)
                .map(Result::unwrap)
                .collect()
        )
    );
}

// Perform a togological sort on inputs
fn is_valid(update: &Vec<i32>, rules: &HashSet<(i32, i32)>) -> bool {
    for (idx, page) in update.iter().enumerate() {
        for (_, b) in rules.iter().filter(|r| r.0 == *page) {
            if let Some(idx1) = update.iter().position(|n| *n == *b) {
                if idx1 < idx {
                    return false;
                }
            }
        }
    }

    true
}
