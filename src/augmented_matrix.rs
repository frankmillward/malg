use crate::{Matrix, MatrixEntry};

/// `M`-by-`(N+P)` rectangular matrix `[A|B]` formed by augmenting a `M`-by-`N` matrix `A` with a `M`-by-`P` matrix `B`.
#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub struct AugmentedMatrix<const M: usize, const N: usize, const P: usize, T: MatrixEntry> {
    left: Matrix<M, N, T>,
    right: Matrix<M, P, T>,
}

impl<const M: usize, const N: usize, const P: usize, T: MatrixEntry> AugmentedMatrix<M, N, P, T> {
    /// Append matrix `b` to the matrix `a`, creating the augmented matrix `[a|b]`.
    pub fn new(a: Matrix<M, N, T>, b: Matrix<M, P, T>) -> AugmentedMatrix<M, N, P, T> {
        AugmentedMatrix::<M, N, P, T> { left: a, right: b }
    }
    /// Left hand part of the augmented matrix.
    pub fn get_left(&self) -> &Matrix<M, N, T> {
        &self.left
    }
    /// Right hand part of the augmented matrix.
    pub fn get_right(&self) -> &Matrix<M, P, T> {
        &self.right
    }
}
