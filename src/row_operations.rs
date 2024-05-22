use std::ops::{Add, Mul};

use crate::MatrixEntry;

/// Provides a set of elementary row operations for an object, where elements of the object are scaled by type `Scalar`
pub trait RowOps<Scalar: MatrixEntry + Mul<Output = Scalar> + Add<Output = Scalar>> {
    /// Swap rows `i` and `j` in place.
    fn swap_rows(&mut self, i: usize, j: usize);
    /// Scale row `i` by scalar value `a` in place.
    fn scale_row(&mut self, i: usize, a: Scalar);
    /// Replace row `i` with the sum of row `i` and `a` times row `j`.
    fn add_rows(&mut self, i: usize, j: usize, a: Scalar);
    /// The `i`th row of `self`.
    fn get_row(&self, i: usize) -> Vec<Scalar>;
    /// Number of rows in `self`
    fn n_rows(&self) -> usize;
    /// Number of columns in `self`
    fn n_cols(&self) -> usize;
}
