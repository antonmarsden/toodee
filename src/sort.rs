use core::cmp::Ordering;

pub use crate::iter::*;
pub use crate::ops::*;
pub use crate::toodee::*;
pub use crate::view::*;

extern crate alloc;

use alloc::vec::Vec;

/// Common re-ordering logic used internally by the SortOps trait.
fn re_order<F>(ordering : &mut [usize], mut swap_func : F)
where F: FnMut(usize, usize)
{
    // swap rows/columns until everything is in the right spot
    for i in 0..ordering.len() {
        if i != ordering[i] {
            let mut j = i;
            loop {
                let k = ordering[j];
                if i == k {
                    break;
                }
                swap_func(j, k);
                ordering[j] = j;
                j = k;
            }
            ordering[j] = j;
        }
    }
}

/// This trait provides sorting capabilities. Sorting of the rows and columns is performed in-place,
/// and care is taken to reduce row/col swaps. This is achieved by sorting with the original
/// indices, then repositioning the rows/columns once the new sort order has been determined.
pub trait SortOps<T> : TooDeeOpsMut<T> {
    
    /// Sort the entire two-dimensional array by comparing elements on a specific row.
    fn sort_by_row<F>(&mut self, row: usize, mut compare: F)
        where
        F: FnMut(&T, &T) -> Ordering, 
    {
        assert!(row < self.num_rows());
        let mut sort_data : Vec<(usize, &T)> = self[row].iter().enumerate().collect();
        sort_data.sort_by(|(_, vi), (_, vj)| compare(vi, vj));
        
        let mut ordering : Vec<usize> = sort_data.iter().map(|(i, _)| *i).collect();
        
        re_order(&mut ordering, |a, b| self.swap_cols(a, b));
    }

    /// Sort the entire two-dimensional array by comparing elements on in a specific column.
    fn sort_by_col<F>(&mut self, col: usize, mut compare: F)
        where
        F: FnMut(&T, &T) -> Ordering, 
    {
        assert!(col < self.num_cols());
        let mut sort_data : Vec<(usize, &T)> = self.col(col).enumerate().collect();
        sort_data.sort_by(|(_, vi), (_, vj)| compare(vi, vj));
        
        let mut ordering : Vec<usize> = sort_data.iter().map(|(i, _)| *i).collect();
        
        re_order(&mut ordering, |a, b| self.swap_rows(a, b));
    }
}

impl<T> SortOps<T> for TooDeeViewMut<'_, T> {}

impl<T> SortOps<T> for TooDee<T> {}
