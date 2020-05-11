pub use crate::iter::*;
pub use crate::ops::*;
pub use crate::toodee::*;
pub use crate::view::*;

/// This trait provides implementations for translate (also known as scroll) operations.
pub trait TranslateOps<T> : TooDeeOpsMut<T> {

    /// Translate (or scroll) the area by moving col_mid to the first column and
    /// row_mid to the first row.
    /// 
    /// All data is preserved by wrapping at the array edges, so `fill()` should be used
    /// to destroy old data as required.
    /// 
    /// Some clever logic ensures that the algorithm performs efficiently when `col_mid` and
    /// `row_mid` are both non-zero.
    fn translate_with_wrap(&mut self, mut col_mid: usize, mut row_mid: usize) {
        
        let num_cols = self.num_cols();
        let num_rows = self.num_rows();

        assert!(col_mid <= num_cols);
        assert!(row_mid <= num_rows);
        
        if col_mid == num_cols {
            col_mid = 0;
        }

        if row_mid == num_rows {
            row_mid = 0;
        }

        if row_mid == 0 {
            if col_mid != 0 {
                // apply column rotation only
                for r in self.rows_mut() {
                    r.rotate_left(col_mid);
                }
            }
            return;
        }

        if row_mid != 0 {
            
            let row_adj_abs = num_rows - row_mid;

            // This row swapping algorithm is pretty cool. I came up with it independently,
            // but it turns out that the concept is fairly well known. See
            // `core::slice::ptr_rotate()` for various strategies.
            
            let mut swap_count = 0;
            let mut base_row = 0;
            
            // TODO: tidy up and possibly create a simpler loop if col_mid == 0

            while swap_count < num_rows {

                let mut mid = col_mid;

                let mut next_row = base_row + row_adj_abs;
                
                loop {
                    if next_row >= num_rows {
                        next_row -= num_rows;
                    }
                    
                    swap_count += 1;

                    if base_row == next_row {
                        // finish up with a rotate
                        if mid > 0 {
                            self[base_row].rotate_left(mid);
                        }
                        break;
                    } else {
                        
                        // the following logic performs a rotate while swapping :)
                        let (base_ref, next_ref) = self.row_pair_mut(base_row, next_row);
                        if mid > 0 {
                            base_ref[..mid].swap_with_slice(&mut next_ref[num_cols-mid..num_cols]);
                        }
                        if mid < num_cols {
                            base_ref[mid..num_cols].swap_with_slice(&mut next_ref[..num_cols-mid]);
                        }
                        
                        mid += col_mid;
                        if mid >= num_cols {
                            mid -= num_cols;
                        }
                    }
                    
                    next_row += row_adj_abs;
                }
                
                // TODO: We now know that we'll loop a further N = (num_rows / swap_count - 1) times.
                // This means we could start swapping in chunks of N, i.e.,
                // ([base_row..base_row+N] -> [base_row+row_adj_abs..base_row+row_adj_abs+N],
                // which should more cache-friendly.
                if swap_count >= num_rows {
                    break;
                }
                
                base_row += 1; // advance the base
                
            }
        }
        
    }
    
    /// Flips (or mirrors) the columns in the array.
    fn flip_rows(&mut self) {
        let mut iter = self.rows_mut();
        while let (Some(r1), Some(r2)) = (iter.next(), iter.next_back()) {
            r1.swap_with_slice(r2);
        }
    }

    /// Flips (or mirrors) the columns in the array.
    fn flip_cols(&mut self) {
        for r in self.rows_mut() {
            r.reverse();
        }
    }
    
}


impl<T> TranslateOps<T> for TooDeeViewMut<'_, T> {}

impl<T> TranslateOps<T> for TooDee<T> {}
