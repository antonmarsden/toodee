#![forbid(unsafe_code)]

use core::fmt;
use core::fmt::{ Formatter, Debug };
use core::ops::{Index, IndexMut};

use crate::*;

/// Provides a read-only view (or subset) of a TooDee array.
#[derive(Copy,Clone)]
pub struct TooDeeView<'a, T : 'a> {
    pub(super) col_start: usize,
    pub(super) row_start: usize,
    pub(super) num_cols: usize,
    pub(super) num_rows: usize,
    pub(super) main_cols: usize,
    pub(super) main_rows: usize,
    pub(super) data: &'a [T],
}

impl<'a, T> TooDeeView<'a, T> {

    /// Create a new TooDeeViewMut using the provided slice reference.
    /// 
    /// Will panic if the slice's length is not sufficient to represent
    /// the desired array dimensions.
    pub fn new(num_cols: usize, num_rows: usize, data: &'a [T]) -> TooDeeView<'a, T> {
        let size = num_cols * num_rows;
        assert!(size <= data.len());
        TooDeeView {
            col_start: 0,
            row_start: 0,
            num_cols,
            num_rows,
            main_cols : num_cols,
            main_rows : num_rows,
            data : &data[..size],
        }
    }
}

impl<'a, T> TooDeeOps<T> for TooDeeView<'a, T>
{
    
    fn num_cols(&self) -> usize {
        self.num_cols
    }

    fn num_rows(&self) -> usize {
        self.num_rows
    }
    
    fn bounds(&self) -> (usize, usize, usize, usize) {
        (self.col_start, self.row_start, self.col_start + self.num_cols, self.row_start + self.num_rows)
    }
    
    fn view(&self, col_start: usize, row_start: usize, col_end: usize, row_end: usize) -> TooDeeView<'_, T> {
        assert!(col_end >= col_start);
        assert!(row_end >= row_start);
        assert!(col_end <= self.num_cols);
        assert!(row_end <= self.num_rows);
        TooDeeView {
            col_start : self.col_start + col_start,
            row_start : self.row_start + row_start,
            num_cols: col_end - col_start,
            num_rows: row_end - row_start,
            main_cols : self.main_cols,
            main_rows : self.main_rows,
            data: self.data,
        }
    }

    fn rows(&self) -> Rows<'_, T> {
        let start = self.row_start * self.main_cols + self.col_start;
        let end = start + (self.num_rows - 1) * self.main_cols + self.num_cols;
        Rows {
            cols : self.num_cols,
            skip_cols : self.main_cols - self.num_cols,
            v : &self.data[start..end],
        }
    }
    
    fn col(&self, col: usize) -> Col<'_, T> {
        let start = self.row_start * self.main_cols + self.col_start + col;
        let end = start + (self.num_rows - 1) * self.main_cols + 1; 
        Col {
            skip : self.main_cols - 1,
            v : &self.data[start..end],
        }
    }

}

impl<'a, T> Index<usize> for TooDeeView<'a, T> {
    type Output = [T];
    fn index(&self, row: usize) -> &Self::Output {
        assert!(row < self.num_rows);
        let start = (self.row_start + row) * self.main_cols + self.col_start;
        &self.data[start..start + self.num_cols]
    }
}

/// Provides a mutable view (or subset), of a TooDee array.
pub struct TooDeeViewMut<'a, T : 'a> {
    pub(super) col_start: usize,
    pub(super) row_start: usize,
    pub(super) num_cols: usize,
    pub(super) num_rows: usize,
    pub(super) main_cols: usize,
    pub(super) main_rows: usize,
    pub(super) data: &'a mut [T],
}


impl<'a, T> TooDeeViewMut<'a, T> {

    /// Create a new TooDeeViewMut using the provided mutable slice reference.
    /// 
    /// Will panic if the slice's length is not sufficient to represent
    /// the desired array dimensions.
    pub fn new(num_cols: usize, num_rows: usize, data: &'a mut [T]) -> TooDeeViewMut<'a, T> {
        let size = num_cols * num_rows;
        assert!(size <= data.len());
        TooDeeViewMut {
            col_start: 0,
            row_start: 0,
            num_cols,
            num_rows,
            main_cols : num_cols,
            main_rows : num_rows,
            data : &mut data[..size],
        }
    }
}


impl<'a, T> TooDeeOps<T> for TooDeeViewMut<'a,T> {

    fn num_rows(&self) -> usize {
        self.num_rows
    }

    fn num_cols(&self) -> usize {
        self.num_cols
    }

    fn bounds(&self) -> (usize, usize, usize, usize) {
        (self.col_start, self.row_start, self.col_start + self.num_cols, self.row_start + self.num_rows)
    }
    
    fn view(&self, col_start: usize, row_start: usize, col_end: usize, row_end: usize) -> TooDeeView<'_, T> {
        assert!(col_end >= col_start);
        assert!(row_end >= row_start);
        assert!(col_end <= self.num_cols);
        assert!(row_end <= self.num_rows);
        TooDeeView {
            col_start : self.col_start + col_start,
            row_start : self.row_start + row_start,
            num_cols: col_end - col_start,
            num_rows: row_end - row_start,
            main_cols : self.main_cols,
            main_rows : self.main_rows,
            data: self.data,
        }
    }

    fn rows(&self) -> Rows<'_, T> {
        let start = self.row_start * self.main_cols + self.col_start;
        let end = start + (self.num_rows - 1) * self.main_cols + self.num_cols;
        Rows {
            cols : self.num_cols,
            skip_cols : self.main_cols - self.num_cols,
            v : &self.data[start..end],
        }
    }

    fn col(&self, col: usize) -> Col<'_, T> {
        let start = self.row_start * self.main_cols + self.col_start + col;
        let end = start + (self.num_rows - 1) * self.main_cols + 1; 
        Col {
            skip : self.main_cols - 1,
            v : &self.data[start..end],
        }
    }

}

impl<'a, T> TooDeeOpsMut<T> for TooDeeViewMut<'a,T> {

    fn view_mut(&mut self, col_start: usize, row_start: usize, col_end: usize, row_end: usize) -> TooDeeViewMut<'_, T> {
        assert!(col_end >= col_start);
        assert!(row_end >= row_start);
        assert!(col_end <= self.num_cols);
        assert!(row_end <= self.num_rows);
        TooDeeViewMut {
            col_start : self.col_start + col_start,
            row_start : self.row_start + row_start,
            num_cols: col_end - col_start,
            num_rows: row_end - row_start,
            main_cols : self.main_cols,
            main_rows : self.main_rows,
            data: self.data,
        }
    }
    
    fn rows_mut(&mut self) -> RowsMut<'_, T> {
        let start = self.row_start * self.main_cols + self.col_start;
        let end = start + (self.num_rows - 1) * self.main_cols + self.num_cols;
        RowsMut {
            cols : self.num_cols,
            skip_cols : self.main_cols - self.num_cols,
            v : &mut self.data[start..end],
        }
    }

    fn col_mut(&mut self, col: usize) -> ColMut<'_, T> {
        let start = self.row_start * self.main_cols + self.col_start + col;
        let end = start + (self.num_rows - 1) * self.main_cols + 1; 
        ColMut {
            skip : self.main_cols - 1,
            v : &mut self.data[start..end],
        }
    }

}

impl<'a, T> Index<usize> for TooDeeViewMut<'a, T> {
    type Output = [T];
    fn index(&self, row: usize) -> &Self::Output {
        assert!(row < self.num_rows);
        let start = (self.row_start + row) * self.main_cols + self.col_start;
        &self.data[start..start + self.num_cols]
    }
}

impl<'a, T> IndexMut<usize> for TooDeeViewMut<'a, T> {
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        assert!(row < self.num_rows);
        let start = (self.row_start + row) * self.main_cols + self.col_start;
        &mut self.data[start..start + self.num_cols]
    }
}

impl<'a, T> Into<TooDeeView<'a, T>> for TooDeeViewMut<'a, T> {
    fn into(self) -> TooDeeView<'a, T> {
        TooDeeView {
            col_start: self.col_start,
            row_start: self.row_start,
            num_cols:  self.num_cols,
            num_rows:  self.num_rows,
            main_cols: self.main_cols,
            main_rows: self.main_rows,
            data:      self.data,
        }
    }
}

impl<T> Debug for TooDeeView<'_, T> where T : Debug {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut dl = f.debug_list();
        for r in self.rows() {
            dl.entry(&r);
        }
        dl.finish()
    }
}

impl<T> Debug for TooDeeViewMut<'_, T> where T : Debug {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut dl = f.debug_list();
        for r in self.rows() {
            dl.entry(&r);
        }
        dl.finish()
    }
}

