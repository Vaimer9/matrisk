use std::ops::Mul;

pub struct Vector<const N: usize>(pub [f64; N]);

impl<const N: usize> Mul<Vector<N>> for Vector<N> {
    type Output = f64;

    fn mul(self, rhs: Vector<N>) -> Self::Output {
        // let va = Simd::<f64, 4>::from_slice(&self.0);
        self.0.iter()
            .zip(rhs.0)
            .map(|(lhs, rhs)| lhs * rhs)
            .sum()
    }
}
