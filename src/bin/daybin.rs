fn main() {

}

/*
OLD CODE
 */

// day9
/*fn part1(data: &mut [u8]) {
    let mut spaces = Vec::with_capacity(data.len() / 2 + 1);
    let mut result = Vec::new();

    let block = data.len() % 2 != 0;

    println!("data: {:?}", str::from_utf8(data).unwrap());
    println!("{data:?}");
    // check operations
    for i in (0..data.len()-1).step_by(2) {
        data[i] -= b'0';
        // spaces.push(data[i + 1] - b'0');
    }

    /* 5
    01
    23

      4
    01
    23
     */

    if block { data[data.len() - 1] -= b'0'; }
 
    for i in (1..data.len()).step_by(2).rev() { spaces.push(data[i] - b'0'); }

    // (ID, size)
    result.push((0usize, data[0]));

    // Last file idx
    let mut idx = data.len() - 2 + block as usize;
    println!("{data:?}");
    println!("last is block: {block} | idx: {idx} | last element: {}", data.last().unwrap());

    let num_spaces = spaces.len();
    println!("Spaces: {spaces:?}");
    let mut c = 0;

    while let Some(space) = spaces.last_mut() {
        if *space == 0 { 
            spaces.pop();
            c += 1;
            continue;
        }

        let file_size = data[idx];
        let id = idx / 2;
        println!("Space #{c}: {space} | File #{id}: {file_size}");
        let cur_space = *space;
        if cur_space >= file_size {
            println!("Space #{c}: {file_size}/{cur_space} used");
            result.push((id, file_size));
            *space -= file_size;
            // data[idx] = 0;
            idx -= 2;
        } else {
            if id == 1 { break; }
            result.push((id, cur_space));
            data[idx] -= cur_space;
        }
        if cur_space <= file_size { 
            println!("Space #{c} Full");
            let next_blocks_idx = c * 2 + 2;
            if let Some(&block_size) = data.get(next_blocks_idx) {
                debug_assert_ne!(block_size, 0);
                result.push((c + 1, block_size));
            }
            spaces.pop();
            c += 1;
        }

        println!("{result:?}");
        // TODO: Break when next space is ahead of current block

        if id == 1 { break; }
    }

    println!("{result:?}");
}
*/