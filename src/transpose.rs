use crate::{CopyOps, TooDee, TooDeeOps};
use alloc::vec::Vec;
use alloc::vec;

/// Provides implementations for transpose operations
pub trait TransposeOps<T> {
    /// Transposes the 2D array
    fn transpose(&mut self);
}

impl<T> TransposeOps<T> for TooDee<T> where T : Default + Copy {
    fn transpose(&mut self) {
        let num_cols = self.num_cols();
        let num_rows = self.num_rows();
        let len: usize = num_cols * num_rows;
        let mut output: Vec<T> = vec![T::default(); len];
        transpose::transpose(&self.data(), &mut output, num_cols, num_rows);
        self.copy_from_slice(&mut output);
        self.swap_dimensions();
    }
}
