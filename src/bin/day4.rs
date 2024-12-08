fn main() {
    let data = std::fs::read_to_string("inputs/example4.txt").unwrap();

    // Converted the input file to LF
    let lines: Vec<&[u8]> = data.split('\n').map(|line| line.as_bytes()).collect();

    let dirs: [(isize, isize); 8] = [
        (0, 1), (1, 0), // Horizontal and Vertical
        (1, 1), (1, -1),// Diagonal

        // Reversed
        (0, -1), (-1, 0), // Horizontal and Vertical
        (-1, -1), (-1, 1)// Diagonal
    ];
    let xmas = b"XMAS";
    let mut xmases = 0;
    let mut row: isize = 0;
    let mut col: isize = 0;
    let mut next_X: (isize, isize) = (0,0);

    loop {
        let mut found = false;
        'o1: for line in lines[row as usize..].iter() {
            for &c in line[col as usize..].iter() {
                col += 1;
                if c == b'X' { 
                    next_X = (row, col - 1); 
                    found = true;
                    break 'o1; 
                }
            }
            row += 1;
            col = 0;
        }
        if !found { break; }

        for dir in dirs.iter() {
            let mut next_pos = next_X;
            let mut matched = true;
            for i in 0..4 {
                let Some(c) = get_char(&lines[..], next_pos) else { matched = false; break; };
                if c != xmas[i] {
                    matched = false;
                    break;
                }
                next_pos = (next_pos.0 + dir.0, next_pos.1 + dir.1);
            }

            if matched { xmases += 1; }
        }
    }

    println!("{xmases}");
}

fn get_char(lines: &[&[u8]], pos: (isize, isize)) -> Option<u8> {
    if pos.0 < 0 || pos.1 < 0 {
        return None;
    }
    
    lines.get(pos.0 as usize).and_then(|line| line.get(pos.1 as usize).copied())
}

