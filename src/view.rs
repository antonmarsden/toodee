use core::fmt;
use core::fmt::{ Formatter, Debug };
use core::ops::{Index, IndexMut};

use crate::toodee::*;
use crate::ops::*;
use crate::iter::*;

/// Checks the proposed view dimensions, and returns the correct cols and rows for view construction.
fn calculate_view_dimensions<T>(start: Coordinate, end: Coordinate, toodee: &impl TooDeeOps<T>) -> (usize, usize) {
    assert!(end.0 >= start.0);
    assert!(end.1 >= start.1);
    assert!(end.0 <= toodee.num_cols());
    assert!(end.1 <= toodee.num_rows());
    let mut num_cols = end.0 - start.0;
    let mut num_rows = end.1 - start.1;
    // zero out dimensions for empty arrays
    if num_cols == 0 || num_rows == 0 {
        num_cols = 0;
        num_rows = 0;
    }
    (num_cols, num_rows)
}

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
    /// # Panics
    /// 
    /// Panics if one of the dimensions is zero but the other is non-zero. This
    /// is to enforce the rule that empty arrays have no dimensions.
    /// 
    /// Panics if the slice's length is not sufficient to represent
    /// the desired array dimensions.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use toodee::TooDeeView;
    /// let data = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
    /// let view = TooDeeView::new(4, 3, &data);
    /// ```
    pub fn new(num_cols: usize, num_rows: usize, data: &'a [T]) -> TooDeeView<'a, T> {
        if num_cols == 0 || num_rows == 0 {
            assert_eq!(num_rows, num_cols);
        }
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
        let (num_cols, num_rows) = calculate_view_dimensions(start, end, toodee);
        let main_cols = toodee.num_cols();
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
    #[inline]
    fn num_cols(&self) -> usize {
        self.num_cols
    }

    #[inline]
    fn num_rows(&self) -> usize {
        self.num_rows
    }
    
    #[inline]
    fn bounds(&self) -> (Coordinate, Coordinate) {
        (self.start, (self.start.0 + self.num_cols, self.start.1 + self.num_rows))
    }
    
    fn view(&self, start: Coordinate, end: Coordinate) -> TooDeeView<'_, T> {
        let (num_cols, num_rows) = calculate_view_dimensions(start, end, self);
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

impl<'a, T> Index<Coordinate> for TooDeeView<'a, T> {

    type Output = T;

    fn index(&self, coord: Coordinate) -> &Self::Output {
        assert!(coord.1 < self.num_rows);
        assert!(coord.0 < self.num_cols);
        // can access the element unchecked because the above assertions hold
        unsafe {
            self.data.get_unchecked(coord.1 * self.main_cols + coord.0)
        }
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
    /// # Panics
    /// 
    /// Panics if one of the dimensions is zero but the other is non-zero. This
    /// is to enforce the rule that empty arrays have no dimensions.
    /// 
    /// Panics if the slice's length is not sufficient to represent
    /// the desired array dimensions.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use toodee::TooDeeViewMut;
    /// let mut data = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
    /// let view_mut = TooDeeViewMut::new(4, 3, &mut data);
    /// ```
    pub fn new(num_cols: usize, num_rows: usize, data: &'a mut [T]) -> TooDeeViewMut<'a, T> {
        if num_cols == 0 || num_rows == 0 {
            assert_eq!(num_rows, num_cols);
        }
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
        let (num_cols, num_rows) = calculate_view_dimensions(start, end, toodee);
        let main_cols = toodee.num_cols();
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
            data: &mut toodee.data_mut()[data_start..data_start + data_len],
        }
    }

}


impl<'a, T> TooDeeOps<T> for TooDeeViewMut<'a,T> {

    #[inline]
    fn num_rows(&self) -> usize {
        self.num_rows
    }

    #[inline]
    fn num_cols(&self) -> usize {
        self.num_cols
    }

    #[inline]
    fn bounds(&self) -> (Coordinate, Coordinate) {
        (self.start, (self.start.0 + self.num_cols, self.start.1 + self.num_rows))
    }
    
    fn view(&self, start: Coordinate, end: Coordinate) -> TooDeeView<'_, T> {
        let (num_cols, num_rows) = calculate_view_dimensions(start, end, self);
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
        let (num_cols, num_rows) = calculate_view_dimensions(start, end, self);

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

impl<'a, T> Index<Coordinate> for TooDeeViewMut<'a, T> {
    type Output = T;
    fn index(&self, coord: Coordinate) -> &Self::Output {
        assert!(coord.1 < self.num_rows);
        assert!(coord.0 < self.num_cols);
        // can access the element unchecked because the above assertions hold
        unsafe {
            self.data.get_unchecked(coord.1 * self.main_cols + coord.0)
        }
    }
}

impl<'a, T> IndexMut<usize> for TooDeeViewMut<'a, T> {
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        assert!(row < self.num_rows);
        let start = row * self.main_cols;
        &mut self.data[start..start + self.num_cols]
    }
}

impl<'a, T> IndexMut<Coordinate> for TooDeeViewMut<'a, T> {
    fn index_mut(&mut self, coord: Coordinate) -> &mut Self::Output {
        assert!(coord.1 < self.num_rows);
        assert!(coord.0 < self.num_cols);
        // can access the element unchecked because the above assertions hold
        unsafe {
            self.data.get_unchecked_mut(coord.1 * self.main_cols + coord.0)
        }
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

impl<'a, T> IntoIterator for &'a TooDeeView<'a, T> {
    type Item = &'a T;
    type IntoIter = Cells<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.cells()
    }
}

impl<'a, T> IntoIterator for &'a TooDeeViewMut<'a, T> {
    type Item = &'a T;
    type IntoIter = Cells<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.cells()
    }
}

impl<'a, T> IntoIterator for &'a mut TooDeeViewMut<'a, T> {
    type Item = &'a mut T;
    type IntoIter = CellsMut<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.cells_mut()
    }
}

impl<T> Debug for TooDeeView<'_, T> where T : Debug {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("TooDeeView");
        d.field("start", &self.start);
        d.field("num_cols", &self.num_cols);
        d.field("num_rows", &self.num_rows);
        d.field("main_cols", &self.main_cols);
        d.finish()
    }
}

impl<T> Debug for TooDeeViewMut<'_, T> where T : Debug {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("TooDeeViewMut");
        d.field("start", &self.start);
        d.field("num_cols", &self.num_cols);
        d.field("num_rows", &self.num_rows);
        d.field("main_cols", &self.main_cols);
        d.finish()
    }
}

