extern crate alloc;
use alloc::boxed::Box;
use core::cmp::Ordering;

use crate::ops::*;
use crate::toodee::*;
use crate::view::*;

/// Common re-indexing logic used internally by the `SortOps` trait.
fn build_swap_trace(ordering : &mut [(usize,usize)]) ->  &mut [(usize,usize)]
{
    let len = ordering.len();
    
    // Create a reverse lookup
    for idx in 0..len {
        let v = ordering[idx].0;
        ordering[v].1 = idx;
    }
    
    let mut swap_count = 0;
    
    // Build a swap trace that will shuffle everything into the right position.
    for i in 0..len {
        let other = ordering[i].0;
        if i != other {
            let inv_i = ordering[i].1;
            // we re-use the ordering slice to store the swap trace
            ordering[swap_count].0 = i;
            ordering[swap_count].1 = other;
            swap_count += 1;
            if inv_i > i {
                ordering[inv_i].0 = other;
                ordering[other].1 = inv_i;
            }
        }
    }
    
    &mut ordering[..swap_count]
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
        
        let mut sort_data : Box<[(usize, &T)]> = self[row].iter().enumerate().map(|(i, v)| (i, v)).collect();
        
        sort_data.sort_by(|i, j| compare(i.1, j.1));
        
        // Build up a "trace" of column swaps to apply
        let mut ordering : Box<[(usize, usize)]> = sort_data.iter().map(|x| (x.0, 0usize)).collect();
        let swap_trace = build_swap_trace(&mut ordering);
        
        // Apply the swap trace to each row. For larger arrays, this approach is faster than applying swap_cols() directly.
        for r in self.rows_mut() {
            for i in swap_trace.iter() {
                r.swap(i.0, i.1);
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

        let mut sort_data : Box<[(usize, &T)]> = self[row].iter().enumerate().map(|(i, v)| (i, v)).collect();
        
        sort_data.sort_unstable_by(|i, j| compare(i.1, j.1));

        // Build up a "trace" of column swaps to apply
        let mut ordering : Box<[(usize, usize)]> = sort_data.iter().map(|x| (x.0, 0usize)).collect();
        let swap_trace = build_swap_trace(&mut ordering);
        
        // Apply the swap trace to each row. For larger arrays, this approach is faster than applying swap_cols() directly.
        for r in self.rows_mut() {
            for i in swap_trace.iter() {
                r.swap(i.0, i.1);
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
        
        let mut sort_data : Box<[(usize, &T)]> = self.col(col).enumerate().map(|(i, v)| (i, v)).collect();

        sort_data.sort_by(|i, j| compare(i.1, j.1));
        
        let mut ordering : Box<[(usize, usize)]> = sort_data.iter().map(|x| (x.0, 0usize)).collect();
        let swap_trace = build_swap_trace(&mut ordering);
        
        for i in swap_trace.iter() {
            self.swap_rows(i.0, i.1);
        }
    }

    /// Sort the entire two-dimensional array by comparing elements on in a specific column.
    /// This sort is unstable.
    fn sort_unstable_by_col<F>(&mut self, col: usize, mut compare: F)
        where
        F: FnMut(&T, &T) -> Ordering, 
    {
        assert!(col < self.num_cols());
        let mut sort_data : Box<[(usize, &T)]> = self.col(col).enumerate().map(|(i, v)| (i, v)).collect();

        sort_data.sort_unstable_by(|i, j| compare(i.1, j.1));

        let mut ordering : Box<[(usize, usize)]> = sort_data.iter().map(|x| (x.0, 0usize)).collect();
        let swap_trace = build_swap_trace(&mut ordering);

        for i in swap_trace.iter() {
            self.swap_rows(i.0, i.1);
        }
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
