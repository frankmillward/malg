use num_traits::{Num, Zero};
use std::ops::{Add, Sub};

/// An m-by-n matrix
#[derive(Eq, PartialEq)]
pub struct Matrix<const M: usize, const N: usize, T: Num + Copy> {
    data: [[T; N]; M],
}

impl<const M: usize, const N: usize, T: Num + Copy> Matrix<M, N, T> {
    /// Create a new [`Matrix`] from a 2-dimensional array.
    pub fn new(data: [[T; N]; M]) -> Self {
        Matrix::<M, N, T> { data }
    }

    /// Extracts a slice containing the entire matrix
    pub fn as_slice(&self) -> &[[T; N]; M] {
        &self.data
    }
}

impl<const M: usize, const N: usize, T: Num + Copy> Zero for Matrix<M, N, T> {
    fn zero() -> Self {
        Matrix::<M, N, T>::new([[T::zero(); N]; M])
    }
    fn is_zero(&self) -> bool {
        *self == Matrix::<M, N, T>::zero()
    }
}

impl<const M: usize, const N: usize, T: Num + Copy> Add for Matrix<M, N, T> {
    type Output = Self;
    /// The natural definition of matrix addition for type `T`
    ///
    /// # Examples
    ///
    /// Add two 2x2 matrices
    ///
    /// ```
    /// use malg::Matrix;
    /// let a = Matrix::<2,2,u8>::new([[1, 2], [3, 4]]);
    /// let b = Matrix::<2,2,u8>::new([[14, 5], [9, 2]]);
    /// let c = a + b;
    /// assert_eq!(*c.as_slice(),[[15, 7], [12, 6]]);
    /// ```
    fn add(self, rhs: Self) -> Self::Output {
        let mut sum = self.data;
        for (a_row, b_row) in sum.iter_mut().zip(rhs.data) {
            for (a, b) in a_row.iter_mut().zip(b_row) {
                *a = *a + b;
            }
        }
        Matrix::<M, N, T>::new(sum)
    }
}

impl<const M: usize, const N: usize, T: Num + Copy> Sub for Matrix<M, N, T> {
    type Output = Self;
    /// The natural defintion of matrix subtraction for type `T`
    ///
    /// # Examples
    ///
    /// Subtract one 2x2 matrix from another
    ///
    /// ```
    /// use malg::Matrix;
    /// let a = Matrix::<2,2,u8>::new([[7, 2], [9, 7]]);
    /// let b = Matrix::<2,2,u8>::new([[2, 1], [3, 3]]);
    /// let c = a - b;
    /// assert_eq!(*c.as_slice(),[[5, 1], [6, 4]]);
    /// ```
    fn sub(self, rhs: Self) -> Self::Output {
        let mut difference = self.data;
        for (a_row, b_row) in difference.iter_mut().zip(rhs.data) {
            for (a, b) in a_row.iter_mut().zip(b_row) {
                *a = *a - b;
            }
        }
        Matrix::<M, N, T>::new(difference)
    }
}
