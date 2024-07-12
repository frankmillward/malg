use crate::ScalarMatrixEntry;
use num_traits::Zero;
use std::ops::{Add, Div, Mul, Sub};

// TODO: Row or RowVector struct which can be used to tighten up RowOps API

/// Behaviours associated with an object constructed of [`RowVector`](`crate::RowVector`)s
pub trait RowOps<const N: usize, T: ScalarMatrixEntry>: Sized {
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
    /// True if `self` is in reduced row echelon form.
    fn is_reduced_row_echelon(&self) -> bool {
        if self.is_row_echelon() {
            let mut is_reduced_row_echelon = true;
            let mut leading_row_index = 0;
            let rows = self.as_rows().collect::<Vec<&[T; N]>>();
            for column_index in 0..N {
                let column_sum: usize = self.as_rows().fold(usize::zero(), |acc, x| {
                    acc + <bool as Into<usize>>::into(!x[column_index].is_zero())
                });
                if !rows[leading_row_index][column_index].is_zero() && column_sum > 1 {
                    is_reduced_row_echelon = false;
                    break;
                } else if rows[leading_row_index][column_index].is_one() {
                    leading_row_index += 1;
                }
            }
            is_reduced_row_echelon
        } else {
            false
        }
    }
    /// `self` in row echelon form.
    fn into_row_echelon(self) -> Self {
        let mut leading_row_index = 0;
        let mut rows = self.into_rows().collect::<Vec<[T; N]>>();
        for pivot_index in 0..N {
            let mut pivot_found = false;
            for row_index in leading_row_index..rows.len() {
                if !rows[row_index][pivot_index].is_zero() {
                    if !pivot_found {
                        pivot_found = true;
                        rows.swap(leading_row_index, row_index);
                        rows[leading_row_index] = rows[leading_row_index]
                            .map(|entry| entry / rows[leading_row_index][pivot_index]);
                    } else {
                        let leading_row = rows[leading_row_index].clone();
                        let leading_entry = rows[row_index][pivot_index];
                        rows[row_index]
                            .iter_mut()
                            .zip(leading_row)
                            .for_each(|(a, b)| *a = *a - b * leading_entry)
                    }
                }
            }
            if pivot_found {
                leading_row_index += 1;
            }
        }
        Self::from_rows(rows.into_iter()).unwrap()
    }
    /// `self` in reduced row echelon form.
    fn into_reduced_row_echelon(self) -> Self {
        let row_echelon_form = self.into_row_echelon();
        let mut rows = row_echelon_form.into_rows().collect::<Vec<[T; N]>>();
        for pivot_column_index in 0..N {
            let mut pivot_found = false;
            let mut pivot_row = [T::zero(); N];
            for row_index in 0..rows.len() {
                let pivot_row_index = rows.len() - 1 - row_index;
                if !rows[pivot_row_index][pivot_column_index].is_zero() {
                    if !pivot_found {
                        pivot_found = true;
                        pivot_row = rows[pivot_row_index].clone();
                    } else {
                        let non_zero_entry = rows[pivot_row_index][pivot_column_index];
                        rows[pivot_row_index]
                            .iter_mut()
                            .zip(pivot_row)
                            .for_each(|(a, b)| *a = *a - b * non_zero_entry);
                    }
                }
            }
        }
        Self::from_rows(rows.into_iter()).unwrap()
    }
}

/// A row vector
pub trait Row<const N: usize, T: ScalarMatrixEntry>: Add + Sub + Div<T> + Mul<T> + Sized {
    fn as_array(&self) -> &[T; N];
}
