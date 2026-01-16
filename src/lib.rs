#![feature(portable_simd)]

pub mod matrix;
pub mod vector;

#[cfg(test)]
mod tests {
    use matrix::Matrix;

    use super::*;

    #[test]
    fn it_works() {
        let a = Matrix::new([
            [1., 2., 3.],
            [7., 3., 4.],
            [4., 2., 6.]
        ]);
        println!("{:?}", a.col(0));
    }
}
