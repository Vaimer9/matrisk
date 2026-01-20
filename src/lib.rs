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
            [4., 5., 7.],
            [8., 2., 6.]
        ]);

        let c = Matrix::<3, 3>::identity();

        println!("{:?}", a * c );
    }
}
