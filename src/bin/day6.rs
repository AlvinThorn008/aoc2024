fn main() {
    let data = std::fs::read_to_string("inputs/input6.txt").unwrap();
    let grid: Vec<&[u8]> = data.lines().map(str::as_bytes).collect();

}

use core::slice;
use std::io::Read;

use Direction::*;

const DIR_VECS: [(i32, i32); 4] = [ (1, 0), (0, 1), (-1, 0), (0, -1) ];

#[derive(Clone, Copy)]
#[repr(u8)]
enum Direction {
    Up,
    Right,
    Down,
    Left
}
fn turn(dir: Direction) -> Direction {
    match dir {
        Up => Right,
        Right => Down,
        Down => Left,
        Left => Up
    }
}
fn dir(ch: u8) -> Option<Direction> {
    Some(match ch {
        b'^' => Up,
        b'>' => Right,
        b'v' => Down,
        b'<' => Left,
        _ => return None
    })
}

fn follow_line(grid: &[&[u8]], state: &mut (Direction, i32, i32)) -> bool {
    let dir_vec = DIR_VECS[state.0 as usize];
    loop {
        let new_state = (state.0, state.1 + dir_vec.0, state.2 + dir_vec.1);
        if new_state.1 < 0 || new_state.1 == grid.len() as i32 || 
        new_state.2 < 0 || new_state.2 == grid[0].len() as i32 {
            return true;
        }
        else if grid[new_state.1 as usize][new_state.2 as usize] == b'#' {
            state.0 = turn(state.0);
            return false;
        }
        *state = new_state;
    }  
}

fn find_start(grid: &[&[u8]]) -> (Direction, i32, i32) {
    for (i, line) in grid.into_iter().enumerate() {
        for (j, c) in line.into_iter().enumerate() {
            if matches!(c, b'^' | b'>' | b'v' | b'<') { 
                return (dir(*c).unwrap(), i as i32, j as i32);
            }
        }
    }
    (Up, 0, 0)
}

fn part1(grid: &[&[u8]]) {
    let mut start = find_start(grid);

    loop {
        if follow_line(grid, &mut start) { break; }
    }

}