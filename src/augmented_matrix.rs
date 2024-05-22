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
    /// Left hand part of the augmented matrix.
    pub fn get_left(&self) -> &Matrix<M, N, T> {
        &self.left
    }
    /// Right hand part of the augmented matrix.
    pub fn get_right(&self) -> &Matrix<M, P, T> {
        &self.right
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
    /// Panics if `i` or `j` are out of bounds. That is `i>=M` or `j>=M`.
    ///
    /// ## Examples
    ///
    /// Swapping the rows of an augmented matrix affects both the left and right parts.
    ///
    /// ```
    /// # use crate::malg::RowOps;
    /// # use::num_traits::*;
    /// # use malg::Matrix;
    /// let a = Matrix::<3,2,u8>::new([[1,2], [3,4], [5,6]]);
    /// let b = Matrix::<3,1,u8>::new([[1], [2], [3]]);
    ///
    /// let a_swapped = Matrix::<3,2,u8>::new([[3,4], [1,2], [5,6]]);
    /// let b_swapped = Matrix::<3,1,u8>::new([[2], [1], [3]]);
    ///
    /// let mut c = a.augment(&b);
    /// let c_swapped = a_swapped.augment(&b_swapped);
    ///
    /// c.swap_rows(0,1);
    ///
    /// assert_eq!(c, c_swapped)
    ///
    /// ```
    fn swap_rows(&mut self, i: usize, j: usize) {
        self.left.swap_rows(i, j);
        self.right.swap_rows(i, j);
    }
    /// Scale row `i` by scalar value `a` in place.
    ///
    /// ## Panics
    ///
    /// Panics if `i` is out of bounds. That is `i>=M`.
    ///
    /// ## Examples
    ///
    /// Scaling a row of an augmented matrix scales both the left and right parts.
    ///
    /// ```
    /// # use crate::malg::RowOps;
    /// # use::num_traits::*;
    /// # use malg::Matrix;
    /// let a = Matrix::<3,2,u8>::new([[1,2], [3,4], [5,6]]);
    /// let b = Matrix::<3,1,u8>::new([[1], [2], [3]]);
    ///
    /// let a_scaled = Matrix::<3,2,u8>::new([[1,2], [6,8], [5,6]]);
    /// let b_scaled = Matrix::<3,1,u8>::new([[1], [4], [3]]);
    ///
    /// let mut c = a.augment(&b);
    /// let c_scaled = a_scaled.augment(&b_scaled);
    ///
    /// c.scale_row(1,2);
    ///
    /// assert_eq!(c, c_scaled)
    ///
    /// ```
    fn scale_row(&mut self, i: usize, a: T) {
        self.left.scale_row(i, a);
        self.right.scale_row(i, a);
    }
    /// Replace row `i` with the sum of row `i` and `a` times row `j`.
    ///
    /// ## Panics
    ///
    /// Panics if `i` or `j` are out of bounds. That is `i>=M` or `j>=M`.
    ///
    /// ## Examples
    ///
    /// Summing the rows of an augmented matrix affects both the left and right parts.
    ///
    /// ```
    /// # use crate::malg::RowOps;
    /// # use::num_traits::*;
    /// # use malg::Matrix;
    /// let a = Matrix::<3,2,u8>::new([[1,2], [3,4], [5,6]]);
    /// let b = Matrix::<3,1,u8>::new([[1], [2], [3]]);
    ///
    /// let a_summed = Matrix::<3,2,u8>::new([[1,2], [3,4], [7,10]]);
    /// let b_summed = Matrix::<3,1,u8>::new([[1], [2], [5]]);
    ///
    /// let mut c = a.augment(&b);
    /// let c_summed = a_summed.augment(&b_summed);
    ///
    /// c.add_rows(2,0,2);
    ///
    /// assert_eq!(c, c_summed)
    ///
    /// ```
    fn add_rows(&mut self, i: usize, j: usize, a: T) {
        self.left.add_rows(i, j, a);
        self.right.add_rows(i, j, a);
    }
    /// The `i`th row of `self`
    ///
    /// ## Panics
    ///
    /// Panics if `i` is out of bounds. That is `i>=M`.
    ///
    /// ## Examples
    ///
    /// `get_row` returns only the left part of the augmented matrix.
    ///
    /// ```
    /// # use crate::malg::RowOps;
    /// # use::num_traits::*;
    /// # use malg::Matrix;
    /// let a = Matrix::<3,2,u8>::new([[1,2], [3,4], [5,6]]);
    /// let b = Matrix::<3,1,u8>::new([[1], [2], [3]]);
    ///
    /// let row = a.augment(&b).get_row(1);
    ///
    /// assert_eq!(row, vec![3,4])
    /// ```
    fn get_row(&self, i: usize) -> Vec<T> {
        self.left.get_row(i)
    }
    /// The number of rows in the left part of the matrix, `M`.
    ///
    /// ## Examples
    ///
    /// ```
    /// # use malg::*;
    /// let a = Matrix::<3,2,u8>::new([[1,2], [3,4], [5,6]]);
    /// let b = Matrix::<3,2,u8>::new([[1,2], [2,3], [3,4]]);
    ///
    /// let number_of_rows = a.augment(&b).n_rows();
    ///
    /// assert_eq!(number_of_rows, 3)
    /// ```
    fn n_rows(&self) -> usize {
        M
    }
    /// The number of rows in the right part of the matrix, `N`.
    ///
    /// ## Examples
    ///
    /// ```
    /// # use malg::*;
    /// let a = Matrix::<3,2,u8>::new([[1,2], [3,4], [5,6]]);
    /// let b = Matrix::<3,1,u8>::new([[1], [2], [3]]);
    ///
    /// let number_of_columns = a.augment(&b).n_cols();
    ///
    /// assert_eq!(number_of_columns, 2)
    /// ```
    fn n_cols(&self) -> usize {
        N
    }
}
