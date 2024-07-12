use num_traits::{One, Zero};
use std::ops::{Div, Sub};

use crate::MatrixEntry;

pub trait Scalar: MatrixEntry + Div + Sub + Zero + One {}
impl<T: MatrixEntry + Div<Output = T> + Sub<Output = T> + Zero + One> Scalar for T {}

/// Provides a set of elementary row operations for an object, where elements of the object are scaled by type `Scalar`
/// pub trait RowOps<Scalar: MatrixEntry + Div<Output = Scalar> + Sub<Output = Scalar> + Zero + One> {
pub trait RowOps<const N: usize, T: Scalar> {
    fn as_rows<'a>(&'a self) -> impl Iterator<Item = &'a [T; N]>
    where
        T: 'a;
    fn as_mut_rows<'a>(&'a mut self) -> impl Iterator<Item = &'a mut [T; N]>
    where
        T: 'a;
    /// True if `self` is in row echelon form
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
}
