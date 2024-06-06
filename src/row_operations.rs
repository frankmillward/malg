use std::ops::{Div, Sub};

use num_traits::{One, Zero};

use crate::MatrixEntry;

/// Provides a set of elementary row operations for an object, where elements of the object are scaled by type `Scalar`
pub trait RowOps<Scalar: MatrixEntry + Div<Output = Scalar> + Sub<Output = Scalar> + Zero + One> {
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
    /// Calculate the row echelon form of `self` in place.
    fn transform_to_row_echelon_form(&mut self) {
        let mut i = 0;
        for j in 0..self.n_cols() {
            let mut pivot_found = false;
            for k in i..self.n_rows() {
                if !self.get_row(k)[j].is_zero() {
                    if pivot_found {
                        let leading_entry = self.get_row(k)[j];
                        self.add_rows(k, i, Scalar::zero() - Scalar::one() * leading_entry);
                    } else {
                        pivot_found = true;
                        self.swap_rows(i, k);
                        let pivot_value = self.get_row(i)[j];
                        self.scale_row(i, Scalar::one() / pivot_value);
                    }
                }
            }
            if pivot_found {
                i += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use crate::*;
    /// Check we can find a row-echelon form of a diagonal [`Matrix`]
    #[test]
    fn check_diagonal_matrix_row_echelon_form() -> Result<(), Box<dyn Error>> {
        let mut input_matrix =
            Matrix::<3, 3, f32>::new([[3.0, 0.0, 0.0], [0.0, 2.0, 0.0], [0.0, 0.0, 1.0]]);
        let target_matrix =
            Matrix::<3, 3, f32>::new([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]);
        input_matrix.transform_to_row_echelon_form();
        assert_eq!(input_matrix, target_matrix);
        Ok(())
    }
    ///  [`Matrix`] of full rank.
    #[test]
    fn check_full_rank_matrix_row_echelon_form() -> Result<(), Box<dyn Error>> {
        let mut input_matrix = Matrix::<3, 4, f32>::new([
            [3.0, 3.0, 2.0, 1.0],
            [1.0, 2.0, 6.0, 0.0],
            [1.0, 0.0, 1.0, 0.0],
        ]);
        input_matrix.transform_to_row_echelon_form();
        assert!(
            input_matrix.get_entry(0, 0).expect("No value").is_one()
                && input_matrix.get_entry(1, 0).expect("No value").is_zero()
                && input_matrix.get_entry(1, 1).expect("No value").is_one()
                && input_matrix.get_entry(2, 0).expect("No value").is_zero()
                && input_matrix.get_entry(2, 1).expect("No value").is_zero()
                && input_matrix.get_entry(2, 2).expect("No value").is_one()
        );
        Ok(())
    }
    /// Check we can find a row echelon form of a rectangular, non-square [`Matrix`] of partial rank
    #[test]
    fn check_partial_rank_matrix_row_echelon_form() -> Result<(), Box<dyn Error>> {
        let mut input_matrix = Matrix::<4, 3, f32>::new([
            [3.0, 3.0, 3.0],
            [1.0, 2.0, 6.0],
            [2.0, 0.0, 2.0],
            [4.0, 0.0, 4.0],
        ]);
        input_matrix.transform_to_row_echelon_form();
        assert!(
            input_matrix.get_entry(0, 0).expect("No value").is_one()
                && input_matrix.get_entry(1, 0).expect("No value").is_zero()
                && input_matrix.get_entry(1, 1).expect("No value").is_one()
                && input_matrix.get_entry(2, 0).expect("No value").is_zero()
                && input_matrix.get_entry(2, 1).expect("No value").is_zero()
                && input_matrix.get_entry(2, 2).expect("No value").is_one()
                && input_matrix
                    .get_row(3)
                    .iter()
                    .map(|a| a.is_zero())
                    .reduce(|acc, e| acc && e)
                    .expect("No value")
        );
        Ok(())
    }
}
