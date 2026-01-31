use crate::matrix::Matrix;

pub trait Operation<const N: usize> {
    fn new() -> Self;
    fn swap(&mut self, src: usize, dest: usize) -> &mut Self;
    fn mult(&mut self, src: usize, mult: f64) -> &mut Self;
    fn add_mult(&mut self, src: usize, dest: usize, mult: f64) -> &mut Self;
    fn build(&self) -> Matrix<N, N>;
}

#[derive(Clone, Copy)]
pub struct RowOperation<const N: usize>(Matrix<N, N>);

#[derive(Clone, Copy)]
pub struct ColOperation<const N: usize>(Matrix<N, N>);

impl<const N: usize> Operation<N> for RowOperation<N> {
    fn new() -> Self {
        Self(Matrix::<N, N>::identity())
    }

    fn swap(&mut self, src: usize, dest: usize) -> &mut Self {
        let src = src - 1; // For indexing
        let dest = dest - 1;
        let t_s = self.0[src][src]; // Temporary Source

        self.0[src][src] = 0.;
        self.0[src][dest] = self.0[dest][dest];
        self.0[dest][dest] = 0.;
        self.0[dest][src] = t_s;
        return self;
    }

    fn mult(&mut self, src: usize, mult: f64) -> &mut Self {
        let src = src - 1;

        self.0[src][src] = mult;

        return self;
    }

    fn add_mult(&mut self, src: usize, dest: usize, mult: f64) -> &mut Self {
        let src = src - 1;
        let dest = dest - 1;

        self.0[dest][src] = mult;

        return self;
    }

    fn build(&self) -> Matrix<N, N> {
        self.0
    }
}

