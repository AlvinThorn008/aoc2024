use core::str;
use std::io::Write;

fn main() {
    let mut data = std::fs::read("inputs/input9.txt").unwrap();
    // Preprocess
    for b in data.iter_mut() { *b -= b'0'; }
    let p1 = part1(&mut data);

    println!("Checksum P1: {p1}");
}

fn part1(data: &mut [u8]) -> usize {
    let mut front_idx = 1; // Space idx
    let mut back_idx = ((data.len() - 1) / 2) * 2; // File idx
    let mut checksum = 0;
    let mut position = data[0] as usize; // First file never moves

    loop {
        let file_size = data[back_idx];
        let space = data[front_idx];

        let prev_front_idx = front_idx;
        if space > 0 {
            let id = back_idx / 2; // even file idx / 2 = file id

            if space >= file_size { // Whole file moved
                checksum += id * sum_k_to(position, file_size as u32);
                position += file_size as usize;
                data[front_idx] -= file_size;
                back_idx -= 2 // next file to move
            } else {
                checksum += id * sum_k_to(position, space as u32);
                position += space as usize;
                data[back_idx] -= space;
                // front_idx += 2; // next space to fill
            }
        }

        // Space has just been completely filled or space was empty
        // so calculate checksum for next file (ahead of space)
        if space <= file_size || space == 0 {
            front_idx += 2; // move to next space (not performed in greater | equal branch)
            let next_file_size = data[prev_front_idx + 1];
            let id = (prev_front_idx + 1) / 2;      // Next file id
            checksum += id * sum_k_to(position, next_file_size as u32); // TODO: 
            position += next_file_size as usize;
        }

        // Next space is ahead of next file so no more spaces to fill
        if front_idx > back_idx { break; }
    }

    checksum
}

/// Sums of `count` values starting from k.
/// 
/// ## Example
/// `sum_k_to(3, 4) = 3 + 4 + 5 + 6 = 18`
fn sum_k_to(k: usize, count: u32) -> usize {
    (count as usize * (2 * k + count as usize - 1)) / 2
}


fn part2(data: &mut [u8]) -> usize {
    let mut front_idx = 1; // Space idx
    let mut back_idx = ((data.len() - 1) / 2) * 2; // File idx
    let mut checksum = 0;
    let mut position = data[0] as usize; // First file never moves

    loop {
        let file_size = data[back_idx];
        let space = data[front_idx];
        let prev_front_idx = front_idx;

        if space > 0 {
            let id = back_idx / 2; // even file idx / 2 = file id
            if space == file_size {
                checksum += id * sum_k_to(position, file_size as u32);
                position += file_size as usize;
                data[front_idx] -= file_size;
                back_idx -= 2 // next file to move
            } else {
                position += space as usize;
            }
        }

        // Space has just been completely filled or space was empty
        // so calculate checksum for next file (ahead of space)
        if space == file_size || space == 0 {
            front_idx += 2; // move to next space (not performed in greater | equal branch)
            let next_file_size = data[prev_front_idx + 1];
            let id = (prev_front_idx + 1) / 2;      // Next file id
            checksum += id * sum_k_to(position, next_file_size as u32);
            position += next_file_size as usize;
        }

        // Next space is ahead of next file so no more spaces to fill
        if front_idx > back_idx { break; }
    }

    checksum
}


// Limited to u8 sized file IDs
fn print_diskmap(data: &[u8]) {
    let mut stdout = std::io::stdout();

    for (i, &b) in data.iter().enumerate() {
        let n = if i % 2 == 0 { ((i / 2) as u8) + b'0' } else { b'.' };
        for _ in 0..b { let _ = write!(&stdout, "{}", n as char); }
    }
    let _ = write!(&stdout, "\n");

    let _ = stdout.flush();
}

/* Pseudo-code, math and other stuff 

//preprocess
let data 
let front_idx = 1 // first space
let back_idx = ? // last file
let checksum = 
let position = data[0] as usize

// sum of `count` values starting from k
fn sum_k_to(k: usize, count: u32) -> usize {
    (count as usize * (2 * k + count as usize - 1)) / 2
}

//code: Not very pseudo ngl
loop {
    let file_size = data[back_idx]
    let space = data[front_idx]
    if space > 0 {
        let p_front_idx = front_idx

        match space.cmp(file_size) {
            Greater | Equal => { // Whole file taken
                let id = back_idx / 2
                checksum += id * sum_k_to(position, file_size) 
                position += file_size
                data[front_idx] -= file_size
                back_idx -= 2 // next file to move
            }
            Less => { // Partial of file taken - whole space used
                let id = back_idx / 2
                checksum += id * sum_k_to(position, space)
                position += space
                data[back_idx] -= space
            }
        }
    }

    // Space has just been completely filled or space was empty
    // so calculate checksum for next file (ahead of space)
    if space <= file_size || space == 0 {
        front_idx += 2; // move to next space (not performed in greater | equal branch)
        let next_file_size = data[p_front_idx + 1]
        let id = (p_front_idx + 1) / 2      // Next file id
        checksum += id * sum_k_to(position, next_file_size)
        position += next_file_size
    }

    // TODO: Find suitable place to check front_idx has not pass back_idx
    // done??
    if front_idx > back_idx { break; }
}

sum(1 to n) = n(n + 1)/2
sum(k to k + a - 1) = sum(1 to k + a - 1) - sum(1 to k-1)
                    = (k + a - 1)(k + a)/2 - (k - 1)k/2
                    = 1/2 ((k + a - 1)(k + a) - (k - 1)k)
                    = 1/2 (k^2 + 2ka - k + a^2 - a - k^2 + k)
                    = 1/2 (2ka + a^2 - a)
                    = 1/2 * a * (2k + a - 1)

 */