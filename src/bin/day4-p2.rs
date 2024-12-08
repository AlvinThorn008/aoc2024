fn main() {
    let data = std::fs::read_to_string("inputs/input4.txt").unwrap();

    // Converted the input file to LF
    let lines: Vec<&[u8]> = data.split('\n').map(|line| line.as_bytes()).collect();

    let mut xmases = 0;
    let mut row: isize = 0;
    let mut col: isize = 0;
    let mut next_X: (isize, isize) = (0,0);

    loop {
        let mut found = false;
        'o1: for line in lines[row as usize..].iter() {
            for &c in line[col as usize..].iter() {
                col += 1;
                if c == b'A' { 
                    next_X = (row, col - 1); 
                    found = true;
                    break 'o1; 
                }
            }
            row += 1;
            col = 0;
        }
        if !found { break; }

        if check_x_mas(&lines[..], next_X).is_some() { xmases += 1; }
    }

    println!("{xmases}");
}

fn check_x_mas(lines: &[&[u8]], pos: (isize, isize)) -> Option<()> {
    let top_left = get_char(lines, (pos.0 - 1, pos.1 - 1))?;
    let bottom_right = get_char(lines, (pos.0 + 1, pos.1 + 1))?;

    if !matches!((top_left, bottom_right), (b'M', b'S') | (b'S', b'M')) { return None; }

    let top_right = get_char(lines, (pos.0 - 1, pos.1 + 1))?;
    let bottom_left = get_char(lines, (pos.0 + 1, pos.1 - 1))?;

    if !matches!((top_right, bottom_left), (b'M', b'S') | (b'S', b'M')) { return None; }
    
    Some(())
}

fn get_char(lines: &[&[u8]], pos: (isize, isize)) -> Option<u8> {
    if pos.0 < 0 || pos.1 < 0 {
        return None;
    }
    
    lines.get(pos.0 as usize).and_then(|line| line.get(pos.1 as usize).copied())
}
