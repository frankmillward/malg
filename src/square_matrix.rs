use std::ops::Mul;

use crate::{Matrix, MatrixEntry};

impl<const N: usize, T: MatrixEntry + Mul<Output = T>> Matrix<N, N, T> {
    /// The trace of a square matrix.
    ///
    /// # Examples
    ///
    /// ```
    /// # use malg::Matrix;
    /// let a = Matrix::<3,3,u8>::new([[1, 2, 3], [1, 2, 3], [1, 2,3]]);
    /// let trace = a.trace();
    /// assert_eq!(trace,6)
    /// ```
    pub fn trace(&self) -> T {
        let mut trace = self.data[0][0];
        for i in 1..N {
            trace = trace * self.data[i][i];
        }
        trace
    }
}
