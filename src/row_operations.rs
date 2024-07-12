use num_traits::{One, Zero};
use std::ops::{Div, Sub};

use crate::MatrixEntry;

pub trait Scalar: MatrixEntry + Div<Output = Self> + Sub<Output = Self> + Zero + One {}
impl<T: MatrixEntry + Div<Output = Self> + Sub<Output = Self> + Zero + One> Scalar for T {}

/// Provides a set of elementary row operations for an object, where elements of the object are scaled by type `Scalar`
/// pub trait RowOps<Scalar: MatrixEntry + Div<Output = Scalar> + Sub<Output = Scalar> + Zero + One> {
pub trait RowOps<const N: usize, T: Scalar>: Sized {
    /// The rows of `self`.
    fn as_rows<'a>(&'a self) -> impl Iterator<Item = &'a [T; N]>
    where
        T: 'a;
    /// The rows of `self`.
    fn as_mut_rows<'a>(&'a mut self) -> impl Iterator<Item = &'a mut [T; N]>
    where
        T: 'a;
    ///
    fn into_rows(self) -> impl Iterator<Item = [T; N]>;
    fn from_rows(rows: impl Iterator<Item = [T; N]>) -> Option<Self>;
    /// True if `self` is in row echelon form.
    fn is_row_echelon(&self) -> bool {
        let mut is_row_echelon = true;
        let mut pivot_index: usize = 0;
        'outer: for (row_index, row) in self.as_rows().enumerate() {
            'inner: for (column_index, entry) in row.iter().enumerate() {
                if !entry.is_zero() {
                    if !entry.is_one() || column_index <= pivot_index && row_index != 0 {
                        is_row_echelon = false;
                        break 'outer;
                    } else {
                        pivot_index = column_index;
                        break 'inner;
                    }
                }
            }
        }
        is_row_echelon
    }
    /// `self` in row echelon form.
    fn into_row_echelon(self) -> Self {
        let mut leading_row_index = 0;
        let mut rows = self.into_rows().collect::<Vec<[T; N]>>();
        for pivot_index in 0..N {
            let mut pivot_found = false;
            for row_index in 0..rows.len() {
                if !rows[row_index][pivot_index].is_zero() {
                    if !pivot_found {
                        pivot_found = true;
                        rows.swap(leading_row_index, row_index);
                        rows[leading_row_index] = rows[leading_row_index]
                            .map(|entry| entry / rows[leading_row_index][pivot_index]);
                    } else {
                        let leading_row = rows[leading_row_index].clone();
                        rows[row_index]
                            .iter_mut()
                            .zip(leading_row)
                            .for_each(|(a, b)| *a = *a - b * *a)
                    }
                }
            }
            if pivot_found {
                leading_row_index += 1;
            }
        }
        Self::from_rows(rows.into_iter()).unwrap()
    }
}
