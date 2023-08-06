use crate::{CopyOps, TooDee, TooDeeOps};
use alloc::vec::Vec;
use alloc::vec;

/// Defines the `transpose` function
pub trait TransposeOps<T> {
    /// Transposes a `TooDee` array
    fn transpose(&mut self);
}

impl<T> TransposeOps<T> for TooDee<T> where T : Default + Copy {
    /// Transposes a `TooDee` array. This implementation does an
    /// out-of-place transpose then copies the result back into
    /// the underlying array.
    fn transpose(&mut self) {
        let num_cols = self.num_cols();
        let num_rows = self.num_rows();
        let mut output: Vec<T> = vec![T::default(); num_cols * num_rows];
        transpose::transpose(self.data(), &mut output, num_cols, num_rows);
        self.copy_from_slice( &output);
        self.swap_dimensions();
    }
}
