#![feature(portable_simd)]

pub mod matrix;
pub mod vector;

#[cfg(test)]
mod tests {
    use matrix::Matrix;

    use super::*;

    #[test]
    fn it_works() {
        let a = Matrix::new([[1., 2., 3.], [2., 3., 4.]]);
        let b = Matrix::new([[1., 2., 3.], [2., 3., 4.]]);
        let result = a * b.transpose();
        println!("{:?}", result);
    }
}
