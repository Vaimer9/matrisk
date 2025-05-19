use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Vector<const N: usize>(pub [f64; N]);

impl<const N: usize> Vector<N> {
    pub fn splat(val: f64) -> Self {
        Self([val; N])
    }
}

// Just the inner product
// TODO: Different length vectors
impl<const N: usize> Mul<Vector<N>> for Vector<N> {
    type Output = f64;

    fn mul(self, rhs: Vector<N>) -> Self::Output {
        self.0.iter()
            .zip(rhs.0)
            .map(|(lhs, rhs)| lhs * rhs)
            .sum()
    }
}

impl<const N: usize> Mul<f64> for Vector<N> {
    type Output = Vector<N>;

    fn mul(self, rhs: f64) -> Self::Output {
        let mut rx = Vector::<N>::splat(0.);
        for i in 0..N {
            rx.0[i] = self.0[i] * rhs;
        }
        return rx;
    }
}

impl<const N: usize> Add<Vector<N>> for Vector<N> {
    type Output = Vector<N>;

    fn add(self, rhs: Vector<N>) -> Self::Output {
        let mut rx = Vector::<N>::splat(0.);
        for i in 0..N {
            rx.0[i] = self.0[i] + rhs.0[i];
        }

        return rx;
    }
}

impl<const N: usize> Sub<Vector<N>> for Vector<N> {
    type Output = Vector<N>;

    fn sub(self, rhs: Vector<N>) -> Self::Output {
        let mut rx = Vector::<N>::splat(0.);
        for i in 0..N {
            rx.0[i] = self.0[i] - rhs.0[i];
        }

        return rx;
    }
}
