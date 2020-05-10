use core::fmt;
use core::fmt::{ Formatter, Debug };
use core::ops::{Index, IndexMut};
use core::slice;
use core::borrow::Borrow;

extern crate alloc;

use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;

pub use crate::iter::*;
pub use crate::view::*;
pub use crate::ops::*;

/// Represents a two-dimensional array
pub struct TooDee<T> {
    pub(super) num_rows: usize,
    pub(super) num_cols: usize,
    pub(super) data: Box<[T]>,
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

    fn bounds(&self) -> (usize, usize, usize, usize) {
        (0, 0, self.num_cols, self.num_rows)
    }
    
    fn view(&self, col_start: usize, row_start: usize, col_end: usize, row_end: usize) -> TooDeeView<'_, T> {
        assert!(col_end >= col_start);
        assert!(row_end >= row_start);
        assert!(col_end <= self.num_cols);
        assert!(row_end <= self.num_rows);
        TooDeeView {
            col_start,
            row_start,
            num_cols: col_end - col_start,
            num_rows: row_end - row_start,
            main_cols : self.num_cols,
            main_rows : self.num_rows,
            data: &self.data,
        }
    }
    
    fn rows(&self) -> Rows<'_, T> {
        Rows {
            cols : self.num_cols,
            skip_cols : 0,
            v : &self.data,
        }
    }
    
    fn col(&self, col: usize) -> Col<'_, T> {
        Col {
            skip : self.num_cols - 1,
            v : &self.data[col..self.data.len() - self.num_cols + col + 1],
        }
    }
    

}

impl<T> TooDeeOpsMut<T> for TooDee<T> {
    
    fn view_mut(&mut self, col_start: usize, row_start: usize, col_end: usize, row_end: usize) -> TooDeeViewMut<'_, T> {
        assert!(col_end >= col_start);
        assert!(row_end >= row_start);
        assert!(col_end <= self.num_cols);
        assert!(row_end <= self.num_rows);
        TooDeeViewMut {
            row_start,
            col_start,
            num_rows: row_end - row_start,
            num_cols: col_end - col_start,
            main_cols : self.num_cols,
            main_rows : self.num_rows,
            data: &mut self.data,
        }
    }
    
    fn copy_from_slice(&mut self, src: &[T]) where T: Copy {
        assert_eq!(self.data.len(), src.len());
        self.data.copy_from_slice(src);
    }
    
    fn rows_mut(&mut self) -> RowsMut<'_, T> {
        RowsMut {
            cols : self.num_cols,
            skip_cols : 0,
            v : &mut self.data,
        }
    }
    
    fn col_mut(&mut self, col: usize) -> ColMut<'_, T> {
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

    /// Create a new TooDee array of the specified dimensions, and fill it with
    /// an initial value.
    pub fn new(num_cols: usize, num_rows: usize, init_value: T) -> TooDee<T>
    where T: Clone {
        let len = num_rows * num_cols;
        // the algorithms won't cope with arrays beyond this isize::MAX
        assert!(len < isize::MAX as usize);
        let v = vec![init_value; len];
        TooDee {
            num_cols,
            num_rows,
            data : v.into_boxed_slice(),
        }
    }

    /// Create a new TooDee array using the provided vector. The vector's length
    /// must match the dimensions of the array.
    pub fn from_vec(num_cols: usize, num_rows: usize, v: Vec<T>) -> TooDee<T> {
        TooDee::from_box(num_cols, num_rows, v.into_boxed_slice())
    }
    
    /// Create a new TooDee array using the provided boxed slice. The slice's length
    /// must match the dimensions of the array.
    pub fn from_box(num_cols: usize, num_rows: usize, b: Box<[T]>) -> TooDee<T> {
        assert_eq!(num_cols * num_rows, b.len());
        TooDee {
            num_cols,
            num_rows,
            data : b,
        }
    }
    
    /// Returns a reference to the raw array data
    pub fn data(&self) -> &[T] {
        &self.data
    }

    /// Returns a mutable reference to the raw array data
    pub fn data_mut(&mut self) -> &mut [T] {
        &mut self.data
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

/// Support conversion into a Vec.
impl<T> Into<Vec<T>> for TooDee<T> {
    fn into(self) -> Vec<T> {
        self.data.into()
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
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut dl = f.debug_list();
        for r in self.rows() {
            dl.entry(&r);
        }
        dl.finish()
    }
}

impl<T> From<TooDeeView<'_, T>> for TooDee<T> where T : Clone {
    fn from(view: TooDeeView<'_, T>) -> Self {
        let mut v = Vec::with_capacity(view.num_rows * view.num_cols);
        for r in view.rows() {
            v.extend_from_slice(r);
        }
        TooDee {
            num_cols : view.num_cols,
            num_rows : view.num_rows,
            data     : v.into_boxed_slice(),
        }
    }
}

impl<T> From<TooDeeViewMut<'_, T>> for TooDee<T> where T : Clone {
    fn from(view: TooDeeViewMut<'_, T>) -> Self {
        let mut v = Vec::with_capacity(view.num_rows * view.num_cols);
        for r in view.rows() {
            v.extend_from_slice(r);
        }
        TooDee {
            num_cols : view.num_cols,
            num_rows : view.num_rows,
            data     : v.into_boxed_slice(),
        }
    }
}
