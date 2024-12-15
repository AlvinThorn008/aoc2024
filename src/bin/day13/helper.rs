pub fn btn_to_vec(line: &str) -> (f64, f64) {
    (
        (line.as_bytes()[12] - b'0') as f64 * 10.0 + (line.as_bytes()[13] - b'0') as f64,
        (line.as_bytes()[18] - b'0') as f64 * 10.0 + (line.as_bytes()[19] - b'0') as f64
    )
}

pub fn prize_to_vec(line: &str) -> (f64, f64) {
    let mut nums = [0f64;2];
    let mut i = 0;
    for b in line.as_bytes() {
        if *b == b',' { i = 1; continue; }
        if !b.is_ascii_digit() { continue; }       
        nums[i] = nums[i] * 10.0 + (*b - b'0') as f64;
    }
    (nums[0], nums[1])
}

pub mod structure {
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