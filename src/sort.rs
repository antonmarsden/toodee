use core::cmp::Ordering;

pub use crate::iter::*;
pub use crate::ops::*;
pub use crate::toodee::*;
pub use crate::view::*;

extern crate alloc;

use alloc::vec::Vec;

/// This trait provides sorting capabilities. Sorting of the rows and columns is performed in-place,
/// and care is taken to minimise row/col swaps.
pub trait SortOps<T> : TooDeeOpsMut<T> {
    
    /// Sort the entire two-dimensional array by comparing elements on a specific row.
    fn sort_by_row<F>(&mut self, row: usize, ref mut compare: F)
        where
        F: FnMut(&T, &T) -> Ordering, 
    {
        assert!(row < self.num_rows());
        let mut sort_data : Vec<(usize, &T)> = self[row].iter().enumerate().collect();
        sort_data.sort_by(|(_, vi), (_, vj)| compare(vi, vj));
        
        let mut index : Vec<usize> = sort_data.iter().map(|(i, _)| *i).collect();

        // Now swap columns until everything is in the right spot
        for i in 0..index.len() {
            if i != index[i] {
                let mut j = i;
                loop {
                    let k = index[j];
                    if i == k {
                        break;
                    }
                    self.swap_cols(j, k);
                    index[j] = j;
                    j = k;
                }
                index[j] = j;
            }
        }
    }

    /// Sort the entire two-dimensional array by comparing elements on in a specific column.
    fn sort_by_col<F>(&mut self, col: usize, ref mut compare: F)
        where
        F: FnMut(&T, &T) -> Ordering, 
    {
        assert!(col < self.num_cols());
        let mut sort_data : Vec<(usize, &T)> = self.col(col).enumerate().collect();
        sort_data.sort_by(|(_, vi), (_, vj)| compare(vi, vj));
        
        let mut index : Vec<usize> = sort_data.iter().map(|(i, _)| *i).collect();
        
        // Now swap rows until everything is in the right spot
        for i in 0..index.len() {
            if i != index[i] {
                let mut j = i;
                loop {
                    let k = index[j];
                    if i == k {
                        break;
                    }
                    self.swap_rows(j, k);
                    index[j] = j;
                    j = k;
                }
                index[j] = j;
            }
        }
    }
}

impl<T> SortOps<T> for TooDeeViewMut<'_, T> {}

impl<T> SortOps<T> for TooDee<T> {}
