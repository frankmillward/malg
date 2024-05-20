use num_traits::{One, Zero};
use std::ops::Mul;

use crate::{Matrix, MatrixEntry};

/// `N`-by-`N` square matrix with entries of type `T`.
pub type SquareMatrix<const N: usize, T> = Matrix<N, N, T>;

impl<const N: usize, T: MatrixEntry + Mul<Output = T>> SquareMatrix<N, T> {
    /// The trace of a square matrix.
    ///
    /// # Examples
    ///
    /// ```
    /// # use malg::SquareMatrix;
    /// let a = SquareMatrix::<3,u8>::new([[1, 2, 3], [1, 2, 3], [1, 2,3]]);
    /// let trace = a.trace();
    /// assert_eq!(trace,6)
    /// ```
    ///
    /// ```
    /// # use malg::SquareMatrix;
    /// let b = SquareMatrix::<1,u8>::new([[4]]);
    /// let trace = b.trace();
    /// assert_eq!(trace, 4)
    /// ```
    pub fn trace(&self) -> T {
        let mut trace = self.data[0][0];
        for i in 1..N {
            trace = trace * self.data[i][i];
        }
        trace
    }
}

impl<const N: usize, T: MatrixEntry + One + Zero> One for SquareMatrix<N, T> {
    /// The N-by-N identity matrix
    ///
    /// # Examples
    ///
    /// ```
    /// # use num_traits::*;
    /// # use malg::{SquareMatrix, Matrix};
    /// let identity = SquareMatrix::<3,u8>::new([[1, 0, 0], [0, 1, 0], [0, 0, 1]]);
    /// assert_eq!(Matrix::<3,3,u8>::one(), identity)
    /// ```
    fn one() -> Self {
        let mut identity = Matrix::<N, N, T>::zero();
        for i in 0..N {
            identity.data[i][i] = T::one();
        }
        identity
    }
}
