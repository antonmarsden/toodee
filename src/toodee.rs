use core::fmt;
use core::fmt::{ Formatter, Debug };
use core::ops::{Index, IndexMut};
use core::slice;
use core::borrow::Borrow;
use core::iter::IntoIterator;

extern crate alloc;

use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;
use alloc::vec::Drain;

pub use crate::iter::*;
pub use crate::view::*;
pub use crate::ops::*;
pub use crate::translate::*;

/// Represents a two-dimensional array.
/// 
/// Empty arrays will always have dimensions of zero.
#[derive(Clone,Default)]
pub struct TooDee<T> {
    num_rows: usize,
    num_cols: usize,
    data: Vec<T>,
}

impl<T> Index<usize> for TooDee<T> {
    type Output = [T];
    
    fn index(&self, row: usize) -> &Self::Output {
        assert!(row < self.num_rows);
        let start = row * self.num_cols;
        &self.data[start..start + self.num_cols]
    }
}

impl<T> IndexMut<usize> for TooDee<T> {
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        assert!(row < self.num_rows);
        let start = row * self.num_cols;
        &mut self.data[start..start + self.num_cols]
    }
}

impl<T> TooDeeOps<T> for TooDee<T> {
    
    fn num_cols(&self) -> usize {
        self.num_cols
    }

    fn num_rows(&self) -> usize {
        self.num_rows
    }

    fn bounds(&self) -> (Coordinate, Coordinate) {
        ((0, 0), (self.num_cols, self.num_rows))
    }
    
    fn view(&self, start: Coordinate, end: Coordinate) -> TooDeeView<'_, T> {
        TooDeeView::from_toodee(start, end, self)
    }
    
    fn rows(&self) -> Rows<'_, T> {
        Rows {
            cols : self.num_cols,
            skip_cols : 0,
            v : &self.data,
        }
    }
    
    fn col(&self, col: usize) -> Col<'_, T> {
        assert!(col < self.num_cols);
        Col {
            skip : self.num_cols - 1,
            v : &self.data[col..self.data.len() - self.num_cols + col + 1],
        }
    }

}

impl<T> TooDeeOpsMut<T> for TooDee<T> {
    
    fn view_mut(&mut self, start: Coordinate, end: Coordinate) -> TooDeeViewMut<'_, T> {
        TooDeeViewMut::from_toodee(start, end, self)
    }
    
    fn copy_from_slice(&mut self, src: &[T]) where T: Copy {
        self.data.copy_from_slice(src);
    }
    
    fn clone_from_slice(&mut self, src: &[T]) where T: Clone {
        self.data.clone_from_slice(src);
    }
    
    fn rows_mut(&mut self) -> RowsMut<'_, T> {
        RowsMut {
            cols : self.num_cols,
            skip_cols : 0,
            v : &mut self.data,
        }
    }
    
    fn col_mut(&mut self, col: usize) -> ColMut<'_, T> {
        assert!(col < self.num_cols);
        let dlen = self.data.len();
        ColMut {
            skip : self.num_cols - 1,
            v : &mut self.data[col..dlen - self.num_cols + col + 1],
        }
    }

    fn fill<V>(&mut self, fill: V)
    where
        V: Borrow<T>,
        T: Clone {
        let value = fill.borrow();
        for v in self.data.iter_mut() {
            v.clone_from(value);
        }
    }
    
}

impl<T> TooDee<T> {

    /// Create a new `TooDee` array of the specified dimensions, and fill it with
    /// the type's default value.
    /// 
    /// Will panic if one of the dimensions is zero but the other is non-zero. This
    /// is to enforce the rule that empty arrays have no dimensions.
    pub fn new(num_cols: usize, num_rows: usize) -> TooDee<T>
    where T: Default + Clone {
        if num_cols == 0 || num_rows == 0 {
            assert_eq!(num_rows, num_cols);
        }
        let len = num_rows * num_cols;
        let v = vec![T::default(); len];
        TooDee {
            num_cols,
            num_rows,
            data : v,
        }
    }

    /// Create a new `TooDee` array of the specified dimensions, and fill it with
    /// an initial value.
    /// 
    /// Will panic if one of the dimensions is zero but the other is non-zero. This
    /// is to enforce the rule that empty arrays have no dimensions.
    pub fn init(num_cols: usize, num_rows: usize, init_value: T) -> TooDee<T>
    where T: Clone {
        if num_cols == 0 || num_rows == 0 {
            assert_eq!(num_rows, num_cols);
        }
        let len = num_rows * num_cols;
        let v = vec![init_value; len];
        TooDee {
            num_cols,
            num_rows,
            data : v,
        }
    }
    
    /// Constructs a new, empty `TooDee<T>` with the specified element capacity.
    /// 
    /// TODO: need a way to set/update num_cols.
    pub fn with_capacity(capacity: usize) -> TooDee<T> {
        TooDee {
            num_cols : 0,
            num_rows : 0,
            data     : Vec::with_capacity(capacity),
        }
    }

    /// Reserves the minimum capacity for at least `additional` more elements to be inserted
    /// into the `TooDee<T>`.
    pub fn reserve_exact(&mut self, capacity: usize) {
        self.data.reserve_exact(capacity);
    }
    
    /// Reserves capacity for at least `additional` more elements to be inserted
    /// in the given `TooDee<T>`.    
    pub fn reserve(&mut self, capacity: usize) {
        self.data.reserve(capacity);
    }

    /// Shrinks the capacity of the underlying vector as much as possible.
    pub fn shrink_to_fit(&mut self) {
        self.data.shrink_to_fit();
    }
    
    /// Create a new `TooDee` array using the provided vector. The vector's length
    /// must match the dimensions of the array.
    /// 
    /// Will panic if one of the dimensions is zero but the other is non-zero. This
    /// is to enforce the rule that empty arrays have no dimensions.
    pub fn from_vec(num_cols: usize, num_rows: usize, v: Vec<T>) -> TooDee<T> {
        if num_cols == 0 || num_rows == 0 {
            assert_eq!(num_rows, num_cols);
        }
        assert_eq!(num_cols * num_rows, v.len());
        TooDee {
            num_cols,
            num_rows,
            data : v,
        }
    }
    
    /// Create a new `TooDee` array using the provided boxed slice. The slice's length
    /// must match the dimensions of the array.
    pub fn from_box(num_cols: usize, num_rows: usize, b: Box<[T]>) -> TooDee<T> {
        TooDee::from_vec(num_cols, num_rows, b.into_vec())
    }
    
    /// Returns a reference to the raw array data
    pub fn data(&self) -> &[T] {
        &self.data
    }

    /// Returns a mutable reference to the raw array data
    pub fn data_mut(&mut self) -> &mut [T] {
        &mut self.data
    }
    
    
    /// Clears the array, removing all values and zeroing the number of columns and rows.
    ///
    /// Note that this method has no effect on the allocated capacity of the array.
    pub fn clear(&mut self) {
        self.num_cols = 0;
        self.num_rows = 0;
        self.data.clear();
    }
    
    /// Removes the last row from the array and returns it as a `Drain`, or `None` if it is empty.
    pub fn pop_row(&mut self) -> Option<Drain<'_, T>> {
        if self.num_rows == 0 {
            None
        } else {
            Some(self.remove_row(self.num_rows - 1))
        }
    }
    
    /// Appends a new row to the array.
    /// 
    /// Panics if the data's length doesn't match the length of existing rows (if any).
    pub fn push_row<I>(&mut self, data: impl IntoIterator<Item=T, IntoIter=I>)
    where I : Iterator<Item=T> + ExactSizeIterator
    {
        self.insert_row(self.num_rows, data);
    }

    /// Inserts new `data` into the array at the specified `row`
    /// 
    /// Panics if the data's length doesn't match the length of existing rows (if any).
    pub fn insert_row<I>(&mut self, index: usize, data: impl IntoIterator<Item=T, IntoIter=I>)
    where I : Iterator<Item=T> + ExactSizeIterator
    {
        assert!(index <= self.num_rows);
        let iter = data.into_iter();
        if self.num_rows == 0 {
            self.num_cols = iter.len();
        } else {
            assert_eq!(self.num_cols, iter.len());
        }
        self.num_rows += 1;
        // append the new row to the end of the vector
        self.data.extend(iter);
        // rotate a subset of the vector
        let start = index * self.num_cols;
        self.data[start..].rotate_right(self.num_cols);
    }

    /// Removes the specified row from the array and returns it as a `Drain`
    /// 
    /// Panics if the specified row index is out of bounds.
    pub fn remove_row(&mut self, index : usize) -> Drain<'_, T>
    {
        assert!(index < self.num_rows);
        let start = index * self.num_cols;
        let drain = self.data.drain(start..start + self.num_cols);
        self.num_rows -= 1;
        if self.num_rows == 0 {
            self.num_cols = 0;
        }
        drain
    }

    /// Removes the last column from the array and returns it as a `Drain`, or `None` if it is empty.
    pub fn pop_col(&mut self) -> Option<Drain<'_, T>> {
        if self.num_cols == 0 {
            None
        } else {
            Some(self.remove_col(self.num_cols - 1))
        }
    }
    
    /// Appends a new column to the array.
    /// 
    /// Panics if the data's length doesn't match the length of existing rows (if any).
    pub fn push_col<I>(&mut self, data: impl IntoIterator<Item=T, IntoIter=I>)
    where I : Iterator<Item=T> + ExactSizeIterator
    {
        self.insert_col(self.num_cols, data);
    }

    /// Removes the specified column from the array and returns it as a `Drain`
    /// 
    /// Panics if the specified column index is out of bounds.
    pub fn remove_col(&mut self, index: usize) -> Drain<'_, T>
    {
        assert!(index < self.num_cols);
        
        let len = self.data.len();

        // TODO: tidy this logic up a bit
        let mut start = index;
        let incr = self.num_cols - 1;
        let mut n = 1;
        while start + self.num_cols + n - 1 < len {
            self.data[start..start + self.num_cols + n - 1].rotate_left(n);
            start += incr;
            n += 1;
        }
        
        self.data[start..].rotate_left(self.num_rows);
        
        let drain = self.data.drain(len - self.num_rows..len);

        self.num_cols -= 1;
        if self.num_cols == 0 {
            self.num_rows = 0;
        }
        
        drain
    }

    /// Inserts new `data` into the array at the specified `col`.
    /// 
    /// Panics if the data's length doesn't match the length of existing columns (if any).
    pub fn insert_col<I>(&mut self, index: usize, data: impl IntoIterator<Item=T, IntoIter=I>)
    where I : Iterator<Item=T> + ExactSizeIterator
    {
        assert!(index <= self.num_cols);
        let iter = data.into_iter();
        if self.num_cols == 0 {
            self.num_rows = iter.len();
        } else {
            assert_eq!(self.num_rows, iter.len());
        }
        
        // This algorithm is basically a reverse of the remove_col() impl
        
        // append new column data to end of array
        self.data.extend(iter);
        
        let incr = self.num_cols;
        
        // update the number of columns
        self.num_cols += 1;

        let mut start = self.data.len() - self.num_rows - (self.num_cols - 1 - index);
        
        self.data[start..].rotate_right(self.num_rows);
        
        let mut n = self.num_rows - 1;
        while start >= incr && n > 0 {
            start -= incr;
            self.data[start..start + self.num_cols + n - 1].rotate_right(n);
            n -= 1;
        }

    }


}

impl<'a, T> IntoIterator for &'a TooDee<T> {
    type Item = &'a T;
    type IntoIter = slice::Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut TooDee<T> {
    type Item = &'a mut T;
    type IntoIter = slice::IterMut<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.iter_mut()
    }
}

/// Support conversion into a `Vec`.
impl<T> Into<Vec<T>> for TooDee<T> {
    fn into(self) -> Vec<T> {
        self.data
    }
}

impl<T> AsRef<[T]> for TooDee<T> {
    fn as_ref(&self) -> &[T] {
        &self.data
    }
}

impl<T> AsMut<[T]> for TooDee<T> {
    fn as_mut(&mut self) -> &mut [T] {
        &mut self.data
    }
}

impl<T> Debug for TooDee<T> where T : Debug {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("TooDee");
        d.field("num_cols", &self.num_cols);
        d.field("num_rows", &self.num_rows);
        d.finish()
    }
}

impl<T> From<TooDeeView<'_, T>> for TooDee<T> where T : Clone {
    fn from(view: TooDeeView<'_, T>) -> Self {
        let num_cols = view.num_cols();
        let num_rows = view.num_rows();
        let mut v = Vec::with_capacity(num_cols * num_rows);
        for r in view.rows() {
            v.extend_from_slice(r);
        }
        TooDee {
            num_cols,
            num_rows,
            data : v,
        }
    }
}

impl<T> From<TooDeeViewMut<'_, T>> for TooDee<T> where T : Clone {
    fn from(view: TooDeeViewMut<'_, T>) -> Self {
        let num_cols = view.num_cols();
        let num_rows = view.num_rows();
        let mut v = Vec::with_capacity(num_cols * num_rows);
        for r in view.rows() {
            v.extend_from_slice(r);
        }
        TooDee {
            num_cols,
            num_rows,
            data : v,
        }
    }
}
