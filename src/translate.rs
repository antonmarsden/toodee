use crate::ops::*;
use crate::toodee::*;
use crate::view::*;

/// Provides implementations for translate (also known as scroll) operations, and other internal data
/// movement operations such as flipping.
pub trait TranslateOps<T> : TooDeeOpsMut<T> {

    /// Translate (or scroll) the area by moving `col_mid` to the first column and
    /// `row_mid` to the first row.
    /// 
    /// All data is preserved by wrapping at the array edges, so `fill()` could be used
    /// to clear old data if required.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use toodee::{TooDee,TooDeeOps,TranslateOps};
    /// let v = vec![42u32; 15];
    /// let mut toodee : TooDee<u32> = TooDee::from_vec(5, 3, v);
    /// toodee[1][1] = 1;
    /// // move (1, 1) to (0, 0)
    /// toodee.translate_with_wrap((1, 1));
    /// assert_eq!(toodee[0][0], 1);
    /// assert_eq!(toodee[1][1], 42);
    /// ```
    /// 
    /// ```
    /// use toodee::{TooDee,TooDeeOps,TranslateOps};
    /// let v = vec![42u32; 15];
    /// let mut toodee : TooDee<u32> = TooDee::from_vec(5, 3, v);
    /// // set (4, 2) to 1
    /// toodee[2][4] = 1;
    /// // move (4, 2) to (0, 0)
    /// toodee.translate_with_wrap((4, 2));
    /// assert_eq!(toodee[0][0], 1);
    /// assert_eq!(toodee[2][4], 42);
    /// ```
    fn translate_with_wrap(&mut self, mid: Coordinate) {

        let (mut col_mid, mut row_mid) = mid;
        
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
                        
                        // The following logic performs a rotate while swapping, and
                        // is more efficient than doing a swap then rotate.
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
    
    /// Flips (or mirrors) the rows.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use toodee::{TooDee,TooDeeOps,TranslateOps};
    /// let v = vec![42u32; 15];
    /// let mut toodee : TooDee<u32> = TooDee::from_vec(5, 3, v);
    /// toodee[0][1] = 1;
    /// toodee.flip_rows();
    /// assert_eq!(toodee[2][1], 1);
    /// ```
    fn flip_rows(&mut self) {
        let mut iter = self.rows_mut();
        while let (Some(r1), Some(r2)) = (iter.next(), iter.next_back()) {
            r1.swap_with_slice(r2);
        }
    }

    /// Flips (or mirrors) the columns.
    /// 
    /// ```
    /// use toodee::{TooDee,TooDeeOps,TranslateOps};
    /// let v = vec![42u32; 15];
    /// let mut toodee : TooDee<u32> = TooDee::from_vec(5, 3, v);
    /// toodee[1][1] = 1;
    /// toodee.flip_cols();
    /// assert_eq!(toodee[1][3], 1);
    /// ```
    fn flip_cols(&mut self) {
        for r in self.rows_mut() {
            r.reverse();
        }
    }
    
}


impl<T> TranslateOps<T> for TooDeeViewMut<'_, T> {}

impl<T> TranslateOps<T> for TooDee<T> {}
