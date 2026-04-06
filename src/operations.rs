use crate::matrix::Matrix;

pub trait Operation<const N: usize> {
    fn new() -> Self;
    fn swap(self, src: usize, dest: usize) -> Self;
    fn mult(self, src: usize, mult: f64) -> Self;
    fn add_mult(self, src: usize, dest: usize, mult: f64) -> Self;
    fn build(self) -> Matrix<N, N>;
}

#[derive(Clone, Copy)]
pub struct RowOperation<const N: usize>(Matrix<N, N>);

#[derive(Clone, Copy)]
pub struct ColOperation<const N: usize>(Matrix<N, N>);

impl<const N: usize> Operation<N> for RowOperation<N> {
    fn new() -> Self {
        Self(Matrix::<N, N>::identity())
    }

    // Doing this the old fashioned way for perf
    fn swap(mut self, src: usize, dest: usize) -> Self {
        let src = src - 1;
        let dest = dest - 1;
        self.0.raw().swap(src, dest);
        return self;
    }

    fn mult(mut self, src: usize, mult: f64) -> Self {
        let src = src - 1;

        // self.0[src][src] = mult;
        self.0[src].iter_mut().for_each(|x| *x *= mult);

        return self;
    }

    fn add_mult(mut self, src: usize, dest: usize, mult: f64) -> Self {
        let src = src - 1;
        let dest = dest - 1;

        let x = self.0.raw();
        let cpy = x[dest];

        // Im hoping the compilter auto-vectorizes, otherwise...
        // TODO: Make this simd
        x[src].iter_mut().zip(cpy).for_each(|(y, x)| *y += mult * x);
        return self;
    }

    fn build(self) -> Matrix<N, N> {
        self.0
    }
}

impl<const N: usize> Operation<N> for ColOperation<N> {
    fn new() -> Self {
        Self(Matrix::<N, N>::identity())
    }

    fn swap(mut self, src: usize, dest: usize) -> Self {
        todo!()
    }

    fn mult(mut self, src: usize, mult: f64) -> Self {
        todo!()
    }

    fn add_mult(mut self, src: usize, dest: usize, mult: f64) -> Self {
        todo!()
    }

    fn build(self) -> Matrix<N, N> {
        todo!()
    }
}
