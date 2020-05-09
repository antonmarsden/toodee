use core::cmp::Ordering;

pub use crate::iter::*;
pub use crate::ops::*;
pub use crate::toodee::*;
pub use crate::view::*;

/// This trait provides implementations for translate (also known as scroll or slide) operations.
pub trait TranslateOps<T> : TooDeeOpsMut<T> {
    
    /// Translate (or scroll/slide) the area in the specified direction, wrapping
    /// the content at the edges of the array.
    fn translate_with_wrap(&mut self, mut col_adj: isize, mut row_adj: isize) {
        
        let num_rows = self.num_rows();
        let num_cols = self.num_cols();

        if num_rows == 0 || num_cols == 0 {
            return;
        }
        
        {
            let irows = num_rows as isize;
            // no modulo arithmetic required for very large arrays
            if irows > 0 {
                row_adj %= irows;
            }
            let icols = num_cols as isize;
            // no modulo arithmetic required for very large arrays
            if icols > 0 {
                col_adj %= icols;
            }
        }
        
        let row_adj_abs = {
            if row_adj >= 0 {
                row_adj as usize
            } else {
                num_rows - ((-row_adj) as usize)
            }
        };

        let col_mid = {
            match col_adj.cmp(&0) {
                Ordering::Less => (-col_adj) as usize,
                Ordering::Greater => num_cols - col_adj as usize,
                Ordering::Equal => 0,
            }
        };
        
        if row_adj == 0 {
            if col_mid != 0 {
                // apply column rotation only
                for r in self.rows_mut() {
                    r.rotate_left(col_mid);
                }
            }
            return;
        }

        if row_adj != 0 {
            
            // This row swapping algorithm is pretty cool. I came up with it independently,
            // but it turns out that the concept is fairly well known. See slice.rotate_left()
            // and slice.rotate_right() for various strategies.
            let mut swap_count = 0;
            let mut base_row = 0;
            
            let mut mid = col_mid;
            
            // TODO: tidy up and possibly create a simpler loop if col_mid == 0

            while swap_count < num_rows {
                
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
                            base_ref[0..mid].swap_with_slice(&mut next_ref[num_cols-mid..num_cols]);
                        }
                        if mid < num_cols {
                            base_ref[mid..num_cols].swap_with_slice(&mut next_ref[0..num_cols-mid]);
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
    
    /// Translate (or scroll/slide) the area in the specified direction, filling
    /// the old space with a specified value.
    fn translate_with_fill(&mut self, col_adj: isize, row_adj: isize, fill: &T) where T: Clone {
        
        let num_rows = self.num_rows();
        let num_cols = self.num_cols();

        let irows = num_rows as isize;
        let icols = num_cols as isize;
        
        // check for complete fill scenarios
        if (irows < 0 || isize::abs(row_adj) < irows) &&
           (icols < 0 || isize::abs(col_adj) < icols) {

            // at this stage, we know that some original data will remain after sliding
            
            let mut sub_view = {
                match row_adj.cmp(&0) {
                    Ordering::Less => {
                        self.view_mut(0, 0, num_cols, (-row_adj) as usize).fill(fill);
                        self.view_mut(0, (-row_adj) as usize, num_cols, num_rows)
                    },
                    Ordering::Greater => {
                        self.view_mut(0, num_rows - row_adj as usize, num_cols, num_rows).fill(fill);
                        self.view_mut(0, 0, num_cols, num_rows - row_adj as usize)
                    },
                    Ordering::Equal => self.view_mut(0, 0, num_cols, num_rows),
                }
            };
            
            match col_adj.cmp(&0) {
                Ordering::Less => {
                    sub_view.view_mut(0, 0, (-col_adj) as usize, sub_view.num_rows()).fill(fill);
                },
                Ordering::Greater => {
                    sub_view.view_mut(num_cols - col_adj as usize, 0, num_cols, sub_view.num_rows()).fill(fill);
                },
                Ordering::Equal => {},
            }
            
            self.translate_with_wrap(col_adj, row_adj);
            
        } else {
            self.fill(fill);
        }
    }


}


impl<T> TranslateOps<T> for TooDeeViewMut<'_, T> {}

impl<T> TranslateOps<T> for TooDee<T> {}
