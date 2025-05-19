use std::ops::{Add, Index, IndexMut, Mul, Sub};
use std::fmt::Debug;
use crate::vector::Vector;

#[derive(Clone, Copy)]
pub struct Matrix<const N: usize, const M: usize>([[f64; M]; N]);
// [1, 2, 3] | N
// [1, 2, 3] |
// --- M ---
// N rows
// M columns

impl<const N: usize, const M: usize> Matrix<N, M>
{
    pub fn new(mat: [[f64; M]; N]) -> Self {
        Self(mat)
    }

    pub fn splat(x: f64) -> Self {
        Self([[x; M]; N])
    }

    pub fn transpose(&self) -> Matrix<M, N> {
        let mut rx = Matrix::<M, N>::splat(0.);

        for i in 0..N {
            for j in 0..M {
                rx.0[j][i] = self.0[i][j];
            }
        }

        return rx;
    }

    pub fn col(&self, n: usize) -> Vector<M> {
        Vector(self.0[n])
    }

    pub fn rref(&self) -> Self {
        let rx = self;

        for i in 0..N {
            let piv = rx.0[i][i];
            let piv_c = self.col(i);

            for j in 0..N {
                if i == j {
                    continue;
                }
            }
        }

        return self.clone();
    }
}

impl<const N: usize, const M: usize> Add<Matrix<N, M>> for Matrix<N, M> {
    type Output = Matrix<N, M>;

    fn add(self, rhs: Matrix<N, M>) -> Self::Output {
        let mut rx = Matrix::<N, M>::splat(0.);
        for i in 0..N {
            for j in 0..M {
                rx.0[i][j] = self.0[i][j] + rhs.0[i][j]
            }
        }

        return rx;
    }
}

impl<const N: usize, const M: usize> Sub<Matrix<N, M>> for Matrix<N, M> {
    type Output = Matrix<N, M>;

    fn sub(self, rhs: Matrix<N, M>) -> Self::Output {
        let mut rx = Matrix::<N, M>::splat(0.);
        for i in 0..N {
            for j in 0..M {
                rx.0[i][j] = self.0[i][j] - rhs.0[i][j]
            }
        }

        return rx;
    }
}

impl<const M: usize, const N: usize, const P: usize> Mul<Matrix<N, P>> for Matrix<M, N>
{
    type Output = Matrix<M, P>;

    fn mul(self, rhs: Matrix<N, P>) -> Self::Output {
        let mut rx = Matrix::<M, P>::splat(0.);
        let rhs_t = rhs.transpose();

        for i in 0..M {
            for j in 0..P {
                rx.0[i][j] = Vector(self.0[i]) * Vector(rhs_t.0[j]);
            }
        }

        return rx;
    }
}

impl<const N: usize, const M: usize> Debug for Matrix<N, M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(for i in 0..N {
            write!(f, "| ")?;
            for j in 0..M {
                write!(f, "{} ", self.0[i][j])?;
            }
            write!(f, "|\n")?;
        })
    }
}

impl<const N: usize, const M: usize> Index<usize> for Matrix<N, M> {
    type Output = [f64; M];

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<const N: usize, const M: usize> IndexMut<usize> for Matrix<N, M> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}
