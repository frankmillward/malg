use num_traits::Zero;
use std::{
    num::NonZeroUsize,
    ops::{Add, Mul, Sub},
};

mod row_operations;
#[allow(unused_imports)]
pub use row_operations::*;

mod square_matrix;
#[allow(unused_imports)]
pub use square_matrix::*;

pub trait MatrixEntry: Copy + Default + PartialEq {}
impl<T: Copy + Default + PartialEq> MatrixEntry for T {}

/// `M`-by-`N` rectangular matrix with entries of type `T`.
#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub struct Matrix<const M: usize, const N: usize, T: MatrixEntry> {
    data: [[T; N]; M],
}

impl<const M: usize, const N: usize, T: MatrixEntry> Matrix<M, N, T> {
    /// A new [`Matrix`] created from nested arrays.
    pub fn new(data: [[T; N]; M]) -> Self {
        Matrix::<M, N, T> { data }
    }

    /// A slice containing the entire matrix as an array of rows.
    pub fn as_slice(&self) -> &[[T; N]; M] {
        &self.data
    }

    /// The number of rows in the matrix, `M`.
    ///
    /// # Examples
    ///
    /// Get the number of rows in a 3-by-4 rectangular `u8` matrix,
    ///
    /// ```
    /// # use::num_traits::*;
    /// use malg::Matrix;
    /// let a = Matrix::<3,4,u8>::zero();
    /// let n_rows = a.n_rows();
    /// assert_eq!(n_rows, 3);
    /// ```
    pub fn n_rows(&self) -> usize {
        M
    }

    /// The number of columns in the matrix, `N`.
    ///
    /// # Examples
    ///
    /// Get the number of columns in a 3-by-4 rectangular `u8` matrix,
    ///
    /// ```
    /// # use::num_traits::*;
    /// use malg::Matrix;
    /// let a = Matrix::<3,4,u8>::zero();
    /// let n_cols = a.n_cols();
    /// assert_eq!(n_cols, 4);
    /// ```
    pub fn n_cols(&self) -> usize {
        N
    }

    /// A specific entry of a matrix, accessed using zero-based indexing.
    /// If the indices lie outside of the matrix, get [`None`] instead.
    ///
    /// # Examples
    ///
    /// We can get a reference to the (1, 2)<sup>th</sup> entry of the 2-by-3 matrix `a`,
    ///
    /// ```
    /// # use::num_traits::*;
    /// use malg::Matrix;
    /// let a = Matrix::<2,3,u8>::new([[1,2,3],[4,5,6]]);
    /// let a12 = a.get_entry(0,1).unwrap();
    /// assert_eq!(*a12, 2);
    /// ```
    ///
    /// But trying to access the (3,2)<sup>th</sup> entry returns [`None`].
    ///
    /// ```
    /// # use::num_traits::*;
    /// # use malg::Matrix;
    /// # let a = Matrix::<2,3,u8>::new([[1,2,3],[4,5,6]]);
    /// let a32 = a.get_entry(2,1);
    /// assert_eq!(a32, None);
    /// ```
    pub fn get_entry(&self, i: usize, j: usize) -> Option<&T> {
        let row = self.data.get(i)?;
        row.get(j)
    }

    /// A specific entry of the matrix, accessed using zero-based indexing.
    /// If the indices lie outside of the matrix, get [`None`] instead.
    ///
    /// # Examples
    ///
    /// We can get and modify a mutable reference to the (1,2)<sup>th</sup> entry of the 2-by-3 matrix `a`,
    ///
    /// ```
    /// # use::num_traits::*;
    /// use malg::Matrix;
    /// let mut a = Matrix::<2,3,u8>::new([[1,2,3],[4,5,6]]);
    /// let mut a12 = a.get_mut_entry(0,1).unwrap();
    /// *a12 = 10;
    /// assert_eq!(a, Matrix::<2,3,u8>::new([[1,10,3],[4,5,6]]));
    /// ```
    pub fn get_mut_entry(&mut self, i: usize, j: usize) -> Option<&mut T> {
        let row = self.data.get_mut(i)?;
        row.get_mut(j)
    }

    /// A specific entry of the matrix, accessed using one-based indexing.
    /// If the indices lie outside of the matrix, get [`None`] instead.
    ///
    /// # Examples
    ///
    /// We can get a reference to the (1,2)<sup>th</sup> entry of the 2-by-3 matrix `a`,
    ///
    /// ```
    /// # use num_traits::*;
    /// # use std::num::*;
    /// use malg::Matrix;
    /// let a = Matrix::<2,3,u8>::new([[1,2,3],[4,5,6]]);
    /// let a12 = a.entry(NonZeroUsize::new(1).unwrap(),NonZeroUsize::new(2).unwrap()).unwrap();
    /// assert_eq!(*a12, 2);
    /// ```
    ///
    /// But trying to access the (3,2)<sup>th</sup> entry returns [`None`].
    ///
    /// ```
    /// # use::num_traits::*;
    /// # use std::num::*;
    /// # use malg::Matrix;
    /// # let a = Matrix::<2,3,u8>::new([[1,2,3],[4,5,6]]);
    /// let a32 = a.entry(NonZeroUsize::new(3).unwrap(),NonZeroUsize::new(2).unwrap());
    /// assert_eq!(a32, None);
    /// ```
    pub fn entry(&self, i: NonZeroUsize, j: NonZeroUsize) -> Option<&T> {
        self.get_entry(usize::from(i) - 1, usize::from(j) - 1)
    }

    /// A specific entry of the matrix, accessed using one-based indexing.
    /// If the indices lie outside of the matrix, get [`None`] instead.
    ///
    /// # Examples
    ///
    /// We can get and modify a mutable reference to the (1,2)<sup>th</sup> entry of the 2-by-3 matrix `a`,
    ///
    /// ```
    /// # use::num_traits::*;
    /// # use std::num::*;
    /// use malg::Matrix;
    /// let mut a = Matrix::<2,3,u8>::new([[1,2,3],[4,5,6]]);
    /// let mut a12 = a.mut_entry(NonZeroUsize::new(1).unwrap(),NonZeroUsize::new(2).unwrap()).unwrap();
    /// *a12 = 10;
    /// assert_eq!(a, Matrix::<2,3,u8>::new([[1,10,3],[4,5,6]]));
    /// ```
    pub fn mut_entry(&mut self, i: NonZeroUsize, j: NonZeroUsize) -> Option<&mut T> {
        self.get_mut_entry(usize::from(i) - 1, usize::from(j) - 1)
    }

    /// The transpose of a [`Matrix`].
    ///
    /// # Examples
    ///
    /// Get the transpose of a 2-by-3 matrix,
    ///
    /// ```
    /// # use::num_traits::*;
    /// # use std::num::*;
    /// use malg::Matrix;
    /// let mut a = Matrix::<2,3,u8>::new([[1,2,3],[4,5,6]]);
    /// let a_t = a.transpose();
    /// assert_eq!(a_t, Matrix::<3,2,u8>::new([[1,4],[2,5],[3,6]]));
    /// ```
    pub fn transpose(&self) -> Matrix<N, M, T> {
        let mut transpose_data = [[T::default(); M]; N];
        for (i, row) in transpose_data.iter_mut().enumerate().take(N) {
            for (j, entry) in row.iter_mut().enumerate().take(M) {
                *entry = self.data[j][i];
            }
        }
        Matrix::<N, M, T>::new(transpose_data)
    }
}

impl<const M: usize, const N: usize, T: MatrixEntry + Zero> Zero for Matrix<M, N, T> {
    /// The matrix with all entries equal to zero.
    fn zero() -> Self {
        Matrix::<M, N, T>::new([[T::zero(); N]; M])
    }
    fn is_zero(&self) -> bool {
        *self == Matrix::<M, N, T>::zero()
    }
}

impl<const M: usize, const N: usize, T: MatrixEntry + Add<Output = T>> Add for Matrix<M, N, T> {
    type Output = Self;
    /// Natural definition of matrix addition for type `T`.
    ///
    /// # Examples
    ///
    /// Add two 2-by-2 matrices,
    ///
    /// ```
    /// use malg::Matrix;
    /// let a = Matrix::<2,2,u8>::new([[1, 2], [3, 4]]);
    /// let b = Matrix::<2,2,u8>::new([[14, 5], [9, 2]]);
    /// let c = a + b;
    /// assert_eq!(c, Matrix::<2,2,u8>::new([[15, 7], [12, 6]]));
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

impl<const M: usize, const N: usize, T: MatrixEntry + Sub<Output = T>> Sub for Matrix<M, N, T> {
    type Output = Self;
    /// Natural definition of matrix subtraction for type `T`.
    ///
    /// # Examples
    ///
    /// Subtract one 2-by-2 matrix from another,
    ///
    /// ```
    /// use malg::Matrix;
    /// let a = Matrix::<2,2,u8>::new([[7, 2], [9, 7]]);
    /// let b = Matrix::<2,2,u8>::new([[2, 1], [3, 3]]);
    /// let c = a - b;
    /// assert_eq!(c,Matrix::<2,2,u8>::new([[5, 1], [6, 4]]));
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

impl<
        const M: usize,
        const N: usize,
        const P: usize,
        T: MatrixEntry + Mul<Output = T> + Add<Output = T>,
    > Mul<Matrix<N, P, T>> for Matrix<M, N, T>
{
    /// Natural definition of Matrix multiplication for type `T`.
    ///
    /// # Examples
    ///
    /// Mulitply a 2-by-3 and a 3-by-2 matrix, to get a 2-by-2 matrix.
    ///
    /// ```
    /// # use malg::Matrix;
    /// let a = Matrix::<2,3,u8>::new([[5, 1, 2], [7, 1, 2]]);
    /// let b = Matrix::<3,2,u8>::new([[1, 2], [3, 4], [5, 6]]);
    ///
    /// let c: Matrix<2,2,u8> = a*b;
    ///
    /// assert_eq!(c, Matrix::<2,2,u8>::new([[18, 26], [20, 30]]))
    /// ```
    type Output = Matrix<M, P, T>;
    fn mul(self, rhs: Matrix<N, P, T>) -> Self::Output {
        let mut product = [[T::default(); P]; M];
        for (i, row) in product.iter_mut().enumerate().take(M) {
            for (j, entry) in row.iter_mut().enumerate().take(P) {
                for k in 0..N {
                    *entry = *entry + self.data[i][k] * rhs.data[k][j];
                }
            }
        }
        Matrix::<M, P, T>::new(product)
    }
}

impl<const M: usize, const N: usize, T: MatrixEntry + Mul<Output = T>> Mul<T> for Matrix<M, N, T> {
    type Output = Matrix<M, N, T>;

    /// Scale a matrix by post-multiplying by a scalar value
    ///
    /// # Examples
    ///
    /// ```
    /// use malg::Matrix;
    /// let a = Matrix::<2,3,u8>::new([[1, 2, 2], [3, 4, 6]]);
    /// let b = a*2;
    /// assert_eq!(b, Matrix::<2,3,u8>::new([[2, 4, 4], [6, 8, 12]]));
    /// ```
    fn mul(self, rhs: T) -> Self::Output {
        let mut scaled = self.data;
        for row in scaled.iter_mut() {
            for entry in row.iter_mut() {
                *entry = *entry * rhs
            }
        }
        Matrix::<M, N, T>::new(scaled)
    }
}

impl<const M: usize, const N: usize, T: MatrixEntry + Mul<Output = T> + Add<Output = T>> RowOps<T>
    for Matrix<M, N, T>
{
    /// Swap rows `i` and `j` in place.
    ///
    /// ## Panics
    ///
    /// Panics if `i` or `j` are out of bounds. That is `i>=M` or `j>=N`.
    ///
    /// ## Examples
    ///
    /// ```
    /// # use crate::malg::RowOps;
    /// # use::num_traits::*;
    /// use malg::Matrix;
    /// let mut a = Matrix::<3,2,u8>::new([[1,2], [3,4], [5,6]]);
    /// let b = Matrix::<3,2,u8>::new([[1,2], [5,6], [3,4]]);
    ///
    /// a.swap_rows(1,2);
    ///
    /// assert_eq!(a,b)
    /// ```
    fn swap_rows(&mut self, i: usize, j: usize) {
        self.data.swap(i, j);
    }
    /// Scale row `i` by scalar value `a` in place.
    ///
    /// ## Panics
    ///
    /// Panics if `i` is out of bounds. That is `i>=M`.
    ///
    /// ## Examples
    ///
    /// ```
    /// # use crate::malg::RowOps;
    /// # use::num_traits::*;
    /// use malg::Matrix;
    /// let mut a = Matrix::<3,2,u8>::new([[1,2], [3,4], [5,6]]);
    /// let b = Matrix::<3,2,u8>::new([[1,2], [6,8], [5,6]]);
    ///
    /// a.scale_row(1,2);
    ///
    /// assert_eq!(a,b)
    ///
    /// ```
    ///
    fn scale_row(&mut self, i: usize, a: T) {
        self.data[i]
            .iter_mut()
            .for_each(|entry| *entry = *entry * a);
    }
    /// Replace row `i` with the sum of row `i` and `a` times row `j`.
    ///
    /// ## Panics
    ///
    /// Panics if `i` or `j` are out of bounds. That is `i>=M` or `j>=N`.
    ///
    /// ## Examples
    ///
    /// ```
    /// # use crate::malg::RowOps;
    /// # use::num_traits::*;
    /// use malg::Matrix;
    /// let mut a = Matrix::<3,2,u8>::new([[1,2], [3,4], [5,6]]);
    /// let b = Matrix::<3,2,u8>::new([[1,2], [3,4], [7,10]]);
    ///
    /// a.add_rows(2,0,2);
    ///
    /// assert_eq!(a,b)
    ///
    /// ```
    ///
    fn add_rows(&mut self, i: usize, j: usize, a: T) {
        let add_row: Vec<T> = self.data[j].iter().map(|entry| *entry * a).collect();
        self.data[i]
            .iter_mut()
            .zip(add_row)
            .for_each(|(entry_i, entry_j): (&mut T, T)| *entry_i = *entry_i + entry_j);
    }
    /// The `ì`th row of `self`/
    ///
    /// ## Panics
    ///
    /// Panics if `i` is out of bounds. That is `i>=M`.
    ///
    /// ## Examples
    ///
    /// ```
    /// # use crate::malg::RowOps;
    /// # use::num_traits::*;
    /// use malg::Matrix;
    /// let mut a = Matrix::<3,2,u8>::new([[1,2], [3,4], [5,6]]);
    ///
    /// let row = a.get_row(1);
    ///
    /// assert_eq!(row, vec![3, 4])
    /// ```
    fn get_row(&self, i: usize) -> Vec<T> {
        self.data[i].into()
    }
}
