use alloc::boxed::Box;
use core::cmp::Ordering;

use crate::ops::*;
use crate::toodee::*;
use crate::view::*;

extern crate alloc;

use alloc::vec::Vec;

/// Common re-indexing logic used internally by the `SortOps` trait.
fn reindex_in_place<F>(ordering : &mut [(usize,usize)], mut swap_func : F)
where F: FnMut(usize, usize)
{
    let len = ordering.len();
    
    // create a reverse lookup
    for idx in 0..len {
        let v = ordering[idx].0;
        ordering[v].1 = idx;
    }
    
    // Swap until everything is in the right position.
    for i in 0..len {
        let other = ordering[i].0;
        if i != other {
            swap_func(i, other);
            let inv_i = ordering[i].1;
            if inv_i > i {
                ordering[inv_i].0 = other;
                ordering[other].1 = inv_i;
            }
        }
    }
}

/// Provides sorting capabilities to two-dimensional arrays. Sorting of the rows and columns
/// is performed in-place, and care is taken to minimise row/col swaps. This is achieved by
/// sorting the row/col and original index pair, then repositioning the rows/columns once the
/// new sort order has been determined.
pub trait SortOps<T> : TooDeeOpsMut<T> {

    /// Sort the entire two-dimensional array by comparing elements on a specific row, using the natural ordering.
    /// This sort is stable.
    fn sort_row_ord<F>(&mut self, row: usize) where T : Ord {
        self.sort_by_row(row, T::cmp);
    }
    
    /// Sort the entire two-dimensional array by comparing elements on a specific row, using the natural ordering.
    /// This sort is unstable.
    fn sort_unstable_row_ord<F>(&mut self, row: usize) where T : Ord {
        self.sort_unstable_by_row(row, T::cmp);
    }

    /// Sort the entire two-dimensional array by comparing elements on a specific row using the provided compare function.
    /// This sort is stable.
    fn sort_by_row<F>(&mut self, row: usize, mut compare: F)
        where
        F: FnMut(&T, &T) -> Ordering, 
    {
        assert!(row < self.num_rows());
        let mut sort_data : Box<[(usize, &T)]> = self[row].iter().enumerate().collect();
        sort_data.sort_by(|(_, vi), (_, vj)| compare(vi, vj));
        
        let mut ordering : Box<[(usize,usize)]> = sort_data.iter().map(|(i, _)| (*i, 0usize)).collect();

        // Build up a "trace" of swaps, then apply the swap trace to each row
        // This is faster than applying swap_cols() directly.
        let mut swap_trace : Vec<(usize, usize)> = Vec::with_capacity(ordering.len());
        reindex_in_place(&mut ordering, |a, b| swap_trace.push((a, b)));
        
        for r in self.rows_mut() {
            for &(a, b) in swap_trace.iter() {
                r.swap(a, b);
            }
        }
    }
    
    /// Sort the entire two-dimensional array by comparing elements on a specific row using the provided compare function.
    /// This sort is unstable.
    fn sort_unstable_by_row<F>(&mut self, row: usize, mut compare: F)
        where
        F: FnMut(&T, &T) -> Ordering, 
    {
        assert!(row < self.num_rows());
        let mut sort_data : Box<[(usize, &T)]> = self[row].iter().enumerate().collect();
        sort_data.sort_unstable_by(|(_, vi), (_, vj)| compare(vi, vj));
        
        let mut ordering : Box<[(usize,usize)]> = sort_data.iter().map(|(i, _)| (*i, 0usize)).collect();

        // Build up a "trace" of swaps, then apply the swap trace to each row
        // This is faster than applying swap_cols() directly.
        let mut swap_trace : Vec<(usize, usize)> = Vec::with_capacity(ordering.len());
        reindex_in_place(&mut ordering, |a, b| swap_trace.push((a, b)));
        
        for r in self.rows_mut() {
            for &(a, b) in swap_trace.iter() {
                r.swap(a, b);
            }
        }
    }

    /// Sort the entire two-dimensional array by comparing elements on a specific row using a key
    /// extraction function.
    /// This sort is stable.
    fn sort_by_row_key<B, F>(&mut self, row: usize, mut f: F)
        where
        B: Ord,
        F: FnMut(&T) -> B,
    {
        self.sort_by_row(row, |a, b| f(a).cmp(&f(b)));
    }

    /// Sort the entire two-dimensional array by comparing elements on a specific row using a key
    /// extraction function.
    /// This sort is unstable.
    fn sort_unstable_by_row_key<B, F>(&mut self, row: usize, mut f: F)
        where
        B: Ord,
        F: FnMut(&T) -> B,
    {
        self.sort_unstable_by_row(row, |a, b| f(a).cmp(&f(b)));
    }

    /// Sort the entire two-dimensional array by comparing elements on a specific column using the natural ordering.
    /// This sort is stable.
    fn sort_col_ord<F>(&mut self, col: usize) where T : Ord {
        self.sort_by_col(col, T::cmp);
    }
    
    /// Sort the entire two-dimensional array by comparing elements on in a specific column.
    /// This sort is stable.
    fn sort_by_col<F>(&mut self, col: usize, mut compare: F)
        where
        F: FnMut(&T, &T) -> Ordering, 
    {
        assert!(col < self.num_cols());
        let mut sort_data : Box<[(usize, &T)]> = self.col(col).enumerate().collect();
        sort_data.sort_by(|(_, vi), (_, vj)| compare(vi, vj));
        
        let mut ordering : Box<[(usize,usize)]> = sort_data.iter().map(|(i, _)| (*i, 0usize)).collect();
        reindex_in_place(&mut ordering, |a, b| self.swap_rows(a, b));
    }

    /// Sort the entire two-dimensional array by comparing elements on in a specific column.
    /// This sort is unstable.
    fn sort_unstable_by_col<F>(&mut self, col: usize, mut compare: F)
        where
        F: FnMut(&T, &T) -> Ordering, 
    {
        assert!(col < self.num_cols());
        let mut sort_data : Box<[(usize, &T)]> = self.col(col).enumerate().collect();
        sort_data.sort_unstable_by(|(_, vi), (_, vj)| compare(vi, vj));
        
        let mut ordering : Box<[(usize,usize)]> = sort_data.iter().map(|(i, _)| (*i, 0usize)).collect();
        reindex_in_place(&mut ordering, |a, b| self.swap_rows(a, b));
    }

    /// Sort the entire two-dimensional array by comparing elements on a specific column using a key
    /// extraction function.
    /// This sort is stable.
    fn sort_by_col_key<B, F>(&mut self, col: usize, mut f: F)
        where
        B: Ord,
        F: FnMut(&T) -> B,
    {
        self.sort_by_row(col, |a, b| f(a).cmp(&f(b)));
    }

    /// Sort the entire two-dimensional array by comparing elements on a specific column using a key
    /// extraction function.
    /// This sort is unstable.
    fn sort_unstable_by_col_key<B, F>(&mut self, col: usize, mut f: F)
        where
        B: Ord,
        F: FnMut(&T) -> B,
    {
        self.sort_unstable_by_row(col, |a, b| f(a).cmp(&f(b)));
    }
}

impl<T> SortOps<T> for TooDeeViewMut<'_, T> {}

impl<T> SortOps<T> for TooDee<T> {}
