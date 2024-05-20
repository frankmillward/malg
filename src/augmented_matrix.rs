use num_traits::{One, Zero};
use std::ops::{Add, Mul};

use crate::{Matrix, MatrixEntry, RowOps};

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
}

impl<
        const M: usize,
        const N: usize,
        const P: usize,
        T: MatrixEntry + Mul<Output = T> + Add<Output = T>,
    > RowOps<T> for AugmentedMatrix<M, N, P, T>
{
    /// Swap rows `i` and `j` in place.
    ///
    /// ## Panics
    ///
    /// Panics if `i` or `j` are out of bounds. That is `i>=M` or `j>=N`.
    fn swap_rows(&mut self, i: usize, j: usize) {
        self.left.swap_rows(i, j);
        self.right.swap_rows(i, j);
    }
    /// Scale row `i` by scalar value `a` in place.
    ///
    /// ## Panics
    ///
    /// Panics if `i` is out of bounds. That is `i>=M`.
    fn scale_row(&mut self, i: usize, a: T) {
        self.left.scale_row(i, a);
        self.right.scale_row(i, a);
    }
    /// Replace row `i` with the sum of row `i` and `a` times row `j`.
    ///
    /// ## Panics
    ///
    /// Panics if `i` or `j` are out of bounds. That is `i>=M` or `j>=N`.``
    fn add_rows(&mut self, i: usize, j: usize, a: T) {
        self.left.add_rows(i, j, a);
        self.right.add_rows(i, j, a);
    }
    /// The `i`th row of `self`
    ///
    /// ## Panics
    ///
    /// Panics if `i` is out of bounds. That is `i>=M`.
    fn get_row(&self, i: usize) -> Vec<T> {
        self.left.get_row(i)
    }
}
