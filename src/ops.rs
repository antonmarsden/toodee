#![forbid(unsafe_code)]

use core::ops::{Index, IndexMut};
use core::cmp::Ordering;
use core::borrow::Borrow;

pub use crate::iter::*;
pub use crate::view::*;
pub use crate::flattenexact::*;

/// An iterator over each "cell" in a 2D array
pub type Cells<'a, T> = FlattenExact<Rows<'a, T>>;
/// A mutable iterator over each "cell" in a 2D array
pub type CellsMut<'a, T> = FlattenExact<RowsMut<'a, T>>;

/// This trait defines operations common to both TooDee and TooDeeView. Default implementations are provided
/// where possible/practical.
pub trait TooDeeOps<T> : Index<usize,Output=[T]> {
    
    /// The number of columns in the area represented by this object.
    fn num_cols(&self) -> usize;
    /// The number of rows in the area represented by this object.
    fn num_rows(&self) -> usize;
    
    /// Returns the size/dimensions of the current object.
    fn size(&self) -> (usize, usize) {
        (self.num_cols(), self.num_rows())
    }

    /// Returns the bounds of the object's area within the original TooDee area (views
    /// are not nested).
    fn bounds(&self) -> (usize, usize, usize, usize);
    
    /// Returns a view (or subset) of the current area based on the coordinates provided.
    fn view(&self, col_start: usize, row_start: usize, col_end: usize, row_end: usize) -> TooDeeView<'_, T>;
    
    /// Returns an iterator of slices, where each slice represents the entire row of the object's area.
    fn rows(&self) -> Rows<'_, T>;
    
    /// Returns an iterator over a single column
    fn col(&self, col: usize) -> Col<'_, T>;

    /// Returns an iterator that traverses all cells within the area. You should iterate using rows()
    /// where possible because performance will be better.
    fn cells(&self) -> Cells<'_,T> {
        FlattenExact::new(self.rows(), self.num_cols())
    }

}

/// This trait defines operations common to both TooDee and TooDeeViewMut. Default implementations
/// are provided where possible/practical.
pub trait TooDeeOpsMut<T> : TooDeeOps<T> + IndexMut<usize> {

    /// Returns a mutable view (or subset) of the current area based on the coordinates provided.
    fn view_mut(&mut self, col_start: usize, row_start: usize, col_end: usize, row_end: usize) -> TooDeeViewMut<'_, T>;
    
    /// Returns a mutable iterator of slices, where each slice represents the entire row of the object's area.
    fn rows_mut(&mut self) -> RowsMut<'_, T>;
    
    /// Returns a mutable iterator over a single column
    fn col_mut(&mut self, col: usize) -> ColMut<'_, T>;
    

    /// Returns an iterator that traverses all cells within the area. You should iterate using
    /// rows_mut() where possible because performance will be better.
    fn cells_mut(&mut self) -> CellsMut<'_, T> {
        let num_cols = self.num_cols();
        FlattenExact::new(self.rows_mut(), num_cols)
    }
    
    /// Copies data from another slice into this area. The source slice's length
    /// must match the size of this object's area. Data is copied row by row.
    fn copy_from_slice(&mut self, src: &[T]) where T: Copy {
        let cols = self.num_cols();
        assert_eq!(cols * self.num_rows(), src.len());
        for (d, s) in self.rows_mut().zip(src.chunks_exact(cols)) {
            d.copy_from_slice(s)
        }
    }
    
    /// Clones data from another slice into this area. The source slice's length
    /// must match the size of this object's area. Data is cloned row by row.
    fn clone_from_slice(&mut self, src: &[T]) where T: Clone {
        let cols = self.num_cols();
        assert_eq!(cols * self.num_rows(), src.len());
        for (d, s) in self.rows_mut().zip(src.chunks_exact(cols)) {
            d.clone_from_slice(s)
        }
    }
    
    /// Copies data from another TooDeeOps object into this one. The source and
    /// destination dimensions must match.
    fn copy_from_toodee(&mut self, src: &impl TooDeeOps<T>) where T : Copy {
        assert_eq!(self.num_cols(), src.num_cols());
        assert_eq!(self.num_rows(), src.num_rows());
        // Data is copied row by row.
        for (d, s) in self.rows_mut().zip(src.rows()) {
            d.copy_from_slice(s);
        }
    }

    /// Fills the entire area with the specified value.
    fn fill<V>(&mut self, fill: V)
    where
        V: Borrow<T>,
        T: Clone {
        let value = fill.borrow();
        for r in self.rows_mut() {
            for v in r {
                v.clone_from(value);
            }
        }
    }
    
    /// Swap/exchange the data between two columns.
    fn swap_cols(&mut self, c1: usize, c2: usize) {
        assert!(c1 < self.num_rows());
        assert!(c2 < self.num_rows());
        for r in self.rows_mut() {
            r.swap(c1, c2);
        }
    }
    
    /// Swap/exchange the data between two rows.
    /// Will panic if either row index is out of bounds.
    fn swap_rows(&mut self, r1: usize, r2: usize) {
        assert!(r1 < self.num_rows());
        assert!(r2 < self.num_rows());
        match r1.cmp(&r2) {
            Ordering::Less => {
                let mut iter = self.rows_mut();
                let tmp = iter.nth(r1).unwrap();
                tmp.swap_with_slice(iter.nth(r2-r1-1).unwrap());
            },
            Ordering::Greater => {
                let mut iter = self.rows_mut();
                let tmp = iter.nth(r2).unwrap();
                tmp.swap_with_slice(iter.nth(r1-r2-1).unwrap());
            },
            Ordering::Equal => {},
        }
    }
    
    /// Return the specified rows as mutable slices.
    /// Will panic if r1 and r2 are equal, or if either row index is out of bounds.
    fn row_pair_mut(&mut self, r1: usize, r2: usize) -> (&mut [T], &mut [T]) {
        assert!(r1 < self.num_rows());
        assert!(r2 < self.num_rows());
        assert!(r1 != r2);
        match r1.cmp(&r2) {
            Ordering::Less => {
                let mut iter = self.rows_mut();
                let tmp = iter.nth(r1).unwrap();
                (tmp, iter.nth(r2-r1-1).unwrap())
            },
            Ordering::Greater => {
                let mut iter = self.rows_mut();
                let tmp = iter.nth(r2).unwrap();
                (iter.nth(r1-r2-1).unwrap(), tmp)
            },
            Ordering::Equal => {
                // unreachable, in theory
                unimplemented!("r1 != r2");
            },
        }
    }
    
}

