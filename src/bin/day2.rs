use std::{cmp::Ordering, ops::ControlFlow, process::Output};

fn main() {
    let data = std::fs::read_to_string("inputs/input2.txt").unwrap();

    let reports = data.split('\n');

    part2(reports);
}

fn part1<'a>(reports: impl Iterator<Item = &'a str>) {
    let mut buf = Vec::with_capacity(20);

    let mut safes = 0;

    for report in reports {
        let levels_iter = report.split(' ').map(|x| str::parse::<u8>(x).unwrap());
        buf.extend(levels_iter);

        if is_safe(&buf) { safes += 1; }

        buf.clear();
    }
    println!("{safes}");
}

fn part2<'a>(reports: impl Iterator<Item = &'a str>) {
    let mut buf = Vec::with_capacity(20);

    let mut safes = 0;

    for report in reports {
        let levels_iter = report.split(' ').map(|x| str::parse::<u8>(x).unwrap());

        buf.extend(levels_iter.clone());
        let buf_len = buf.len();

        if is_safe(&buf) {
            buf.clear();
            safes += 1;
            continue;
        }

        buf.clear();
        for i in 0..buf_len {
            buf.extend(levels_iter.clone());
            buf.remove(i);
            if is_safe(&buf) { safes += 1; buf.clear(); break; }
            buf.clear();
        }

    }
    println!("{safes}");
}

fn is_safe(buf: &Vec<u8>) -> bool {
    let mut windows = buf.windows(2);
    windows.try_fold(buf[0].cmp(&buf[1]), |acc, window| {
        let ord = window[0].cmp(&window[1]);
        (ord != Ordering::Equal && (ord == acc) && (window[0].abs_diff(window[1]) < 4)).then_some(ord)
    }).is_some()

}