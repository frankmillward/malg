use crate::Matrix;
use num_traits::*;

impl<const N: usize, T: Num + Copy> Matrix<N, N, T> {
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
        let mut trace = T::one();
        for i in 0..N {
            trace = trace * self.data[i][i];
        }
        trace
    }
}
