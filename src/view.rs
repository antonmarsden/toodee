use core::fmt;
use core::fmt::{ Formatter, Debug };
use core::ops::{Index, IndexMut};

use crate::*;

/// Provides a read-only view (or subset) of a `TooDee` array.
#[derive(Copy,Clone)]
pub struct TooDeeView<'a, T> {
    start: Coordinate,
    num_cols: usize,
    num_rows: usize,
    main_cols: usize,
    data: &'a [T],
}

impl<'a, T> TooDeeView<'a, T> {

    /// Create a new `TooDeeViewMut` using the provided slice reference.
    /// 
    /// Will panic if the slice's length is not sufficient to represent
    /// the desired array dimensions.
    pub fn new(num_cols: usize, num_rows: usize, data: &'a [T]) -> TooDeeView<'a, T> {
        let size = num_cols * num_rows;
        assert!(size <= data.len());
        TooDeeView {
            start: (0, 0),
            num_cols,
            num_rows,
            main_cols : num_cols,
            data : &data[..size],
        }
    }
    
    /// Used internally by `TooDee` to create a `TooDeeView`.
    pub(super) fn from_toodee(start: Coordinate, end: Coordinate, toodee: &'a TooDee<T>) -> TooDeeView<'a, T> {
        assert!(end.0 >= start.0);
        assert!(end.1 >= start.1);
        let main_cols = toodee.num_cols();
        let main_rows = toodee.num_rows();
        assert!(end.0 <= main_cols);
        assert!(end.1 <= main_rows);
        let num_cols = end.0 - start.0;
        let num_rows = end.1 - start.1;
        let data_start = start.1 * main_cols + start.0;
        let data_len = {
            if num_rows == 0 {
                0
            } else {
                (num_rows - 1) * main_cols + num_cols
            }
        };
        TooDeeView {
            start,
            num_cols,
            num_rows,
            main_cols,
            data: &toodee.data()[data_start..data_start + data_len],
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
    
    fn bounds(&self) -> (Coordinate, Coordinate) {
        (self.start, (self.start.0 + self.num_cols, self.start.1 + self.num_rows))
    }
    
    fn view(&self, start: Coordinate, end: Coordinate) -> TooDeeView<'_, T> {
        assert!(end.0 >= start.0);
        assert!(end.1 >= start.1);
        assert!(end.0 <= self.num_cols);
        assert!(end.1 <= self.num_rows);
        
        let num_cols = end.0 - start.0;
        let num_rows = end.1 - start.1;

        let data_start = start.1 * self.main_cols + start.0;
        let data_len = {
            if num_rows == 0 {
                0
            } else {
                (num_rows - 1) * self.main_cols + num_cols
            }
        };

        TooDeeView {
            start: (self.start.0 + start.0, self.start.1 + start.1),
            num_cols,
            num_rows,
            main_cols : self.main_cols,
            data: &self.data[data_start..data_start + data_len],
        }
    }

    fn rows(&self) -> Rows<'_, T> {
        Rows {
            cols : self.num_cols,
            skip_cols : self.main_cols - self.num_cols,
            v : &self.data,
        }
    }
    
    fn col(&self, col: usize) -> Col<'_, T> {
        assert!(col < self.num_cols);
        let start = col;
        let end = {
            if self.num_rows == 0 {
                start
            } else {
                start + (self.num_rows - 1) * self.main_cols + 1
            }
        };
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
        let start = row * self.main_cols;
        &self.data[start..start + self.num_cols]
    }
}

/// Provides a mutable view (or subset), of a `TooDee` array.
pub struct TooDeeViewMut<'a, T> {
    start: Coordinate,
    num_cols: usize,
    num_rows: usize,
    main_cols: usize,
    data: &'a mut [T],
}


impl<'a, T> TooDeeViewMut<'a, T> {

    /// Create a new `TooDeeViewMut` using the provided mutable slice reference.
    /// 
    /// Will panic if the slice's length is not sufficient to represent
    /// the desired array dimensions.
    pub fn new(num_cols: usize, num_rows: usize, data: &'a mut [T]) -> TooDeeViewMut<'a, T> {
        let size = num_cols * num_rows;
        assert!(size <= data.len());
        TooDeeViewMut {
            start: (0, 0),
            num_cols,
            num_rows,
            main_cols : num_cols,
            data : &mut data[..size],
        }
    }

    /// Used internally by `TooDee` to create a `TooDeeViewMut`.
    pub(super) fn from_toodee(start: Coordinate, end: Coordinate, toodee: &'a mut TooDee<T>) -> TooDeeViewMut<'a, T> {
        assert!(end.0 >= start.0);
        assert!(end.1 >= start.1);
        let main_cols = toodee.num_cols();
        let main_rows = toodee.num_rows();
        assert!(end.0 <= main_cols);
        assert!(end.1 <= main_rows);
        let num_cols = end.0 - start.0;
        let num_rows = end.1 - start.1;
        let data_start = start.1 * main_cols + start.0;
        let data_len = {
            if num_rows == 0 {
                0
            } else {
                (num_rows - 1) * main_cols + num_cols
            }
        };
        TooDeeViewMut {
            start,
            num_cols,
            num_rows,
            main_cols,
//            main_rows,
            data: &mut toodee.data_mut()[data_start..data_start + data_len],
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

    fn bounds(&self) -> (Coordinate, Coordinate) {
        (self.start, (self.start.0 + self.num_cols, self.start.1 + self.num_rows))
    }
    
    fn view(&self, start: Coordinate, end: Coordinate) -> TooDeeView<'_, T> {
        assert!(end.0 >= start.0);
        assert!(end.1 >= start.1);
        assert!(end.0 <= self.num_cols);
        assert!(end.0 <= self.num_rows);
        let num_cols = end.0 - start.0;
        let num_rows = end.1 - start.1;

        let data_start = start.1 * self.main_cols + start.0;
        let data_len = {
            if num_rows == 0 {
                0
            } else {
                (num_rows - 1) * self.main_cols + num_cols
            }
        };
        
        TooDeeView {
            start: (self.start.0 + start.0, self.start.1 + start.1),
            num_cols,
            num_rows,
            main_cols : self.main_cols,
            data: &self.data[data_start..data_start + data_len],
        }
    }

    fn rows(&self) -> Rows<'_, T> {
        Rows {
            cols : self.num_cols,
            skip_cols : self.main_cols - self.num_cols,
            v : &self.data,
        }
    }

    fn col(&self, col: usize) -> Col<'_, T> {
        assert!(col < self.num_cols);
        let start = col;
        let end = {
            if self.num_rows == 0 {
                start
            } else {
                start + (self.num_rows - 1) * self.main_cols + 1
            }
        };
        Col {
            skip : self.main_cols - 1,
            v : &self.data[start..end],
        }
    }

}

impl<'a, T> TooDeeOpsMut<T> for TooDeeViewMut<'a,T> {

    fn view_mut(&mut self, start: Coordinate, end: Coordinate) -> TooDeeViewMut<'_, T> {
        assert!(end.0 >= start.0);
        assert!(end.1 >= start.1);
        assert!(end.0 <= self.num_cols);
        assert!(end.1 <= self.num_rows);
        let num_cols = end.0 - start.0;
        let num_rows = end.1 - start.1;

        let data_start = start.1 * self.main_cols + start.0;
        let data_len = {
            if num_rows == 0 {
                0
            } else {
                (num_rows - 1) * self.main_cols + num_cols
            }
        };

        TooDeeViewMut {
            start: (self.start.0 + start.0, self.start.1 + start.1),
            num_cols,
            num_rows,
            main_cols : self.main_cols,
            data: &mut self.data[data_start..data_start + data_len],
        }
    }
    
    fn rows_mut(&mut self) -> RowsMut<'_, T> {
        RowsMut {
            cols : self.num_cols,
            skip_cols : self.main_cols - self.num_cols,
            v : &mut self.data,
        }
    }

    fn col_mut(&mut self, col: usize) -> ColMut<'_, T> {
        assert!(col < self.num_cols);
        let start = col;
        let end = {
            if self.num_rows == 0 {
                start
            } else {
                start + (self.num_rows - 1) * self.main_cols + 1
            }
        };
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
        let start = row * self.main_cols;
        &self.data[start..start + self.num_cols]
    }
}

impl<'a, T> IndexMut<usize> for TooDeeViewMut<'a, T> {
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        assert!(row < self.num_rows);
        let start = row * self.main_cols;
        &mut self.data[start..start + self.num_cols]
    }
}

impl<'a, T> Into<TooDeeView<'a, T>> for TooDeeViewMut<'a, T> {
    fn into(self) -> TooDeeView<'a, T> {
        TooDeeView {
            start:     self.start,
            num_cols:  self.num_cols,
            num_rows:  self.num_rows,
            main_cols: self.main_cols,
            data:      self.data,
        }
    }
}

impl<T> Debug for TooDeeView<'_, T> where T : Debug {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut dl = f.debug_list();
        for r in self.rows() {
            dl.entry(&r);
        }
        dl.finish()
    }
}

impl<T> Debug for TooDeeViewMut<'_, T> where T : Debug {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut dl = f.debug_list();
        for r in self.rows() {
            dl.entry(&r);
        }
        dl.finish()
    }
}

