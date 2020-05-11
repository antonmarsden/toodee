use core::cmp::Ordering;

pub use crate::iter::*;
pub use crate::ops::*;
pub use crate::toodee::*;
pub use crate::view::*;

extern crate alloc;

use alloc::vec::Vec;

/// Common re-indexing logic used internally by the SortOps trait.
/// May need to re-visit performance of this algorithm at some stage.
fn reindex_in_place<F>(ordering : &mut [usize], mut swap_func : F)
where F: FnMut(usize, usize)
{
    let len = ordering.len();
    // Set up a reverse lookup
    let mut inverse : Vec<usize> = vec![0usize; len];
    for i in 0..len {
        inverse[ordering[i]] = i;
    }
    
    // Swap until everything is in the right position.
    for i in 0..len {
        let other = ordering[i];
        if i != other {
            swap_func(i, other);
            let inv_i = inverse[i];
            if inv_i > i {
                ordering[inv_i] = other;
                inverse[other] = inv_i;
            }
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

        // Build up a "trace" of swaps, then apply the swap trace to each row
        // This is faster than applying swap_cols() directly.
        let mut swap_trace : Vec<(usize, usize)> = Vec::with_capacity(ordering.len());
        reindex_in_place(&mut ordering, |a, b| swap_trace.push((a, b)));
        
        for r in self.rows_mut() {
            for (a, b) in swap_trace.iter() {
                r.swap(*a, *b);
            }
        }
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
        reindex_in_place(&mut ordering, |a, b| self.swap_rows(a, b));
    }
}

impl<T> SortOps<T> for TooDeeViewMut<'_, T> {}

impl<T> SortOps<T> for TooDee<T> {}

// This was the original re-indexing algorithm, which didn't perform well in benchmarks.
// Leaving it here for future reference :)
//
//fn reindex_in_place<F>(ordering : &mut [usize], mut swap_func : F)
//where F: FnMut(usize, usize)
//{
//    // swap rows/columns until everything is in the right spot
//    for i in 0..ordering.len() {
//        if i != ordering[i] {
//            let mut j = i;
//            loop {
//                let k = ordering[j];
//                if i == k {
//                    break;
//                }
//                swap_func(j, k);
//                ordering[j] = j;
//                j = k;
//            }
//            ordering[j] = j;
//        }
//    }
//}

