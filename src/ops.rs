use core::ops::{Index, IndexMut};
use core::cmp::Ordering;
use core::borrow::Borrow;

use crate::iter::*;
use crate::view::*;
use crate::flattenexact::*;

/// A (col, row) coordinate in 2D space.
pub type Coordinate = (usize, usize);

/// An iterator over each "cell" in a 2D array
pub type Cells<'a, T> = FlattenExact<Rows<'a, T>>;
/// A mutable iterator over each "cell" in a 2D array
pub type CellsMut<'a, T> = FlattenExact<RowsMut<'a, T>>;

/// Defines operations common to both `TooDee` and `TooDeeView`. Default implementations are provided
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

    /// Returns `true` if the array contains no elements.
    fn is_empty(&self) -> bool {
        self.num_cols() == 0 || self.num_rows() == 0
    }

    /// Returns the bounds of the object's area within the original `TooDee` area (views
    /// are not nested for now).
    fn bounds(&self) -> (Coordinate, Coordinate);
    
    /// Returns a view (or subset) of the current area based on the coordinates provided.
    fn view(&self, start: Coordinate, end: Coordinate) -> TooDeeView<'_, T>;
    
    /// Returns an iterator of slices, where each slice represents the entire row of the object's area.
    fn rows(&self) -> Rows<'_, T>;
    
    /// Returns an iterator over a single column
    fn col(&self, col: usize) -> Col<'_, T>;

    /// Returns an iterator that traverses all cells within the area.
    fn cells(&self) -> Cells<'_, T> {
        FlattenExact::new(self.rows())
    }

}

/// Defines operations common to both `TooDee` and `TooDeeViewMut`. Default implementations
/// are provided where possible/practical.
pub trait TooDeeOpsMut<T> : TooDeeOps<T> + IndexMut<usize> {

    /// Returns a mutable view (or subset) of the current area based on the coordinates provided.
    fn view_mut(&mut self, start: Coordinate, end: Coordinate) -> TooDeeViewMut<'_, T>;
    
    /// Returns a mutable iterator of slices, where each slice represents the entire row of the object's area.
    fn rows_mut(&mut self) -> RowsMut<'_, T>;
    
    /// Returns a mutable iterator over a single column
    fn col_mut(&mut self, col: usize) -> ColMut<'_, T>;
    
    /// Returns an iterator that traverses all cells within the area.
    fn cells_mut(&mut self) -> CellsMut<'_, T> {
        FlattenExact::new(self.rows_mut())
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
        let num_cols = self.num_cols();
        assert!(c1 < num_cols);
        assert!(c2 < num_cols);
        for r in self.rows_mut() {
            r.swap(c1, c2);
        }
    }
    
    /// Swap/exchange the data between two rows.
    /// 
    /// # Panics
    /// 
    /// Panics if either row index is out of bounds.
    fn swap_rows(&mut self, r1: usize, r2: usize) {
        let num_rows = self.num_rows();
        assert!(r1 < num_rows);
        assert!(r2 < num_rows);
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
    /// 
    /// # Panics
    /// 
    /// Will panic if `r1` and `r2` are equal, or if either row index is out of bounds.
    fn row_pair_mut(&mut self, r1: usize, r2: usize) -> (&mut [T], &mut [T]) {
        let num_rows = self.num_rows();
        assert!(r1 < num_rows);
        assert!(r2 < num_rows);
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
                unreachable!("r1 != r2");
            },
        }
    }
    
}

