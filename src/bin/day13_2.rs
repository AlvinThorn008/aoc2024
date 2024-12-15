use structure::{Mat2, inv, mul};

fn main() {

    let data = std::fs::read_to_string("inputs/input13.txt").unwrap();
    let mut lines = data.lines();
    let mut min_tokens = 0;

    /*
    coefs = [ 3 4; 4 5 ];
    out = [ 5; 6 ];
    disp(linsolve(coefs, out))
     */
    while let Some(line) = lines.next() {
        if line.len() < 20 { continue; }
        let vec1 = btn_to_vec(line);
        let vec2 = btn_to_vec(lines.next().unwrap());
        let prize = prize_to_vec(lines.next().unwrap());
        let mat = Mat2(vec1.0, vec2.0, vec1.1, vec2.1);
        if vec1.0 / vec1.1 == vec2.0 / vec2.1 { // vectors on same line
            let s = prize.0 / vec1.0;
            if (s == prize.1 / vec1.1) && (s.round() - s).abs() <= 0.0001 { // prize can be reached
                min_tokens += s.round() as i64; // Least tokens spent with button B
                continue;
            }
        } else if let Some(tokens) = solve(&mat, prize) { // a solution exists
            min_tokens += 3 * tokens.0.round() as i64 + tokens.1.round() as i64;
        }
    }

    println!("{min_tokens}");
}

fn solve(mat: &Mat2, res: (f64, f64)) -> Option<(f64, f64)> {
    let inv = inv(mat)?;
    
    let tokens = mul(&inv, res);
    if tokens.0 < 0.0 || tokens.1 < 0.0 || (tokens.0.round() - tokens.0).abs() > 0.0001 || (tokens.1.round() - tokens.1).abs() > 0.0001 { 
        None
    }
    else { Some((tokens.0, tokens.1)) }

}

fn btn_to_vec(line: &str) -> (f64, f64) {
    (
        (line.as_bytes()[12] - b'0') as f64 * 10.0 + (line.as_bytes()[13] - b'0') as f64,
        (line.as_bytes()[18] - b'0') as f64 * 10.0 + (line.as_bytes()[19] - b'0') as f64
    )
}

fn prize_to_vec(line: &str) -> (f64, f64) {
    let mut nums = [0f64;2];
    let mut i = 0;
    for b in line.as_bytes() {
        if *b == b',' { i = 1; continue; }
        if !b.is_ascii_digit() { continue; }       
        nums[i] = nums[i] * 10.0 + (*b - b'0') as f64;
    }
    (nums[0] + 10000000000000.0, nums[1] + 10000000000000.0)
}

mod structure {

    /*
    [ a  b ] -> [ 0, 1 ]
    [ c  d ] -> [ 2, 3 ]
     */
    #[derive(Debug, Clone, Copy)]
    #[repr(packed)]
    pub struct Mat2(pub f64, pub f64, pub f64, pub f64);

    const fn det(mat: &Mat2) -> f64 { mat.0 * mat.3 - mat.1 * mat.2 }

    pub const fn inv(mat: &Mat2) -> Option<Mat2> {
        let det = det(mat);
        if det == 0.0 { None }
        else { 
            let i_det = 1.0 / det;
            Some(Mat2(i_det * mat.3, i_det * -mat.1, i_det * -mat.2, i_det * mat.0))
        }
    }

    pub const fn mul(mat: &Mat2, c_vec: (f64, f64)) -> (f64, f64) {
        (
            mat.0 * c_vec.0 + mat.1 * c_vec.1,
            mat.2 * c_vec.0 + mat.3 * c_vec.1
        )
    }
}

