use core::fmt;
use core::fmt::{Formatter, Debug};
use core::ops::{Index, IndexMut, Range};
use core::ptr;
use core::mem;
use core::hash::{Hash, Hasher};

use crate::toodee::*;
use crate::ops::*;
use crate::iter::*;
use crate::flattenexact::FlattenExact;

/// Checks the proposed view dimensions, and returns the correct cols, rows and slice data range
/// for view construction.
fn calculate_view_dimensions<T>(start: Coordinate, end: Coordinate, toodee: &impl TooDeeOps<T>, stride: usize) -> (usize, usize, Range<usize>) {
    assert!(end.0 >= start.0);
    assert!(end.1 >= start.1);
    assert!(end.0 <= toodee.num_cols());
    assert!(end.1 <= toodee.num_rows());
    assert!(stride >= toodee.num_cols());
    let num_cols = end.0 - start.0;
    let num_rows = end.1 - start.1;
    // Empty views must use a valid range even when their coordinates would
    // otherwise produce an offset beyond the backing slice (for example, a
    // zero-sized view at the end of the final row).
    if num_cols == 0 || num_rows == 0 {
        return (0, 0, 0..0);
    }
    let data_start = start.1 * stride + start.0;
    let data_len = (num_rows - 1) * stride + num_cols;
    (num_cols, num_rows, data_start..data_start + data_len)
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn calc_dims_view() {
        let v = vec![1u32; 32];
        let view = TooDeeView::new(4, 4, &v);
        let (num_cols, num_rows, range) = calculate_view_dimensions((0, 1), (2,3), &view, 4);
        assert_eq!(num_cols, 2);
        assert_eq!(num_rows, 2);
        assert_eq!(range, 4..10);
    }

}

/// *Internal only* functions for calculating vector ranges.
trait TooDeeViewCommon<T>: TooDeeOps<T> {

    fn stride(&self) -> usize;

    fn get_col_params(&self, col: usize) -> (Range<usize>, usize){
        assert!(col < self.num_cols());
        let stride = self.stride();
        let start = col;
        let end = {
            let num_rows = self.num_rows();
            if num_rows == 0 {
                start
            } else {
                start + (num_rows - 1) * stride + 1
            }
        };
        (start..end, stride - 1)
    }


}

impl<T> TooDeeViewCommon<T> for TooDeeView<'_, T> {
    #[inline]
    fn stride(&self) -> usize {
        self.stride
    }
}

impl<T> TooDeeViewCommon<T> for TooDeeViewMut<'_, T> {
    #[inline]
    fn stride(&self) -> usize {
        self.stride
    }
}


/// Provides a read-only view (or subset) of a `TooDee` array.
///
/// Equality and hashing consider only the view's dimensions and the cells it
/// exposes, so views of different parent arrays compare equal when their visible
/// cells match.
pub struct TooDeeView<'a, T> {
    data: &'a [T],
    num_cols: usize,
    num_rows: usize,
    stride: usize,
}

// Implemented manually (rather than derived) so that views are `Copy` regardless of `T`,
// like `&[T]`.
impl<T> Copy for TooDeeView<'_, T> {}

impl<T> Clone for TooDeeView<'_, T> {
    fn clone(&self) -> Self {
        *self
    }
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
    /// Panics if `num_cols * num_rows` overflows.
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
        let size = num_cols.checked_mul(num_rows).unwrap();
        assert!(size <= data.len());
        TooDeeView {
            data: &data[..size],
            num_cols,
            num_rows,
            stride: num_cols,
        }
    }

    /// Used internally by `TooDee` to create a `TooDeeView`.
    pub(super) fn from_toodee(start: Coordinate, end: Coordinate, toodee: &'a TooDee<T>) -> TooDeeView<'a, T> {
        let stride = toodee.num_cols();
        let (num_cols, num_rows, data_range) = calculate_view_dimensions(start, end, toodee, stride);
        unsafe {
            TooDeeView {
                data: toodee.data().get_unchecked(data_range),
                num_cols,
                num_rows,
                stride,
            }
        }
    }

    /// Consumes the view, returning an iterator of slices, where each slice represents an entire row.
    ///
    /// Unlike [`TooDeeOps::rows()`], the returned iterator borrows the underlying data for `'a`
    /// rather than borrowing the view, so it can outlive the view.
    ///
    /// # Examples
    ///
    /// ```
    /// use toodee::{TooDee, TooDeeOps, Rows};
    /// fn top_rows(toodee: &TooDee<u32>) -> Rows<'_, u32> {
    ///     toodee.view((0, 0), (10, 2)).into_rows()
    /// }
    /// let toodee : TooDee<u32> = TooDee::init(10, 5, 42u32);
    /// assert_eq!(top_rows(&toodee).map(|r| r.iter().sum::<u32>()).sum::<u32>(), 42*20);
    /// ```
    pub fn into_rows(self) -> Rows<'a, T> {
        Rows {
            v: self.data,
            cols: self.num_cols,
            skip_cols: self.stride - self.num_cols,
        }
    }

    /// Consumes the view, returning an iterator over a single column.
    ///
    /// Unlike [`TooDeeOps::col()`], the returned iterator borrows the underlying data for `'a`
    /// rather than borrowing the view, so it can outlive the view.
    ///
    /// # Panics
    ///
    /// Panics if `col` is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use toodee::{TooDee, TooDeeOps, Col};
    /// fn second_col(toodee: &TooDee<u32>) -> Col<'_, u32> {
    ///     toodee.view((0, 0), (10, 2)).into_col(1)
    /// }
    /// let toodee : TooDee<u32> = TooDee::init(10, 5, 42u32);
    /// assert_eq!(second_col(&toodee).sum::<u32>(), 42*2);
    /// ```
    pub fn into_col(self, col: usize) -> Col<'a, T> {
        let (data_range, skip) = self.get_col_params(col);
        unsafe {
            Col {
                v: self.data.get_unchecked(data_range),
                skip,
            }
        }
    }

    /// Consumes the view, returning an iterator that traverses all cells within the area.
    /// Equivalent to `view.into_iter()`.
    ///
    /// # Examples
    ///
    /// ```
    /// use toodee::{TooDee, TooDeeOps, Cells};
    /// fn top_cells(toodee: &TooDee<u32>) -> Cells<'_, u32> {
    ///     toodee.view((0, 0), (10, 2)).into_cells()
    /// }
    /// let toodee : TooDee<u32> = TooDee::init(10, 5, 42u32);
    /// assert_eq!(top_cells(&toodee).sum::<u32>(), 42*20);
    /// ```
    pub fn into_cells(self) -> Cells<'a, T> {
        FlattenExact::new(self.into_rows())
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

    fn view(&self, start: Coordinate, end: Coordinate) -> TooDeeView<'_, T> {
        let (num_cols, num_rows, data_range) = calculate_view_dimensions(start, end, self, self.stride);
        unsafe {
            TooDeeView {
                data: self.data.get_unchecked(data_range),
                num_cols,
                num_rows,
                stride: self.stride,
            }
        }
    }

    fn rows(&self) -> Rows<'_, T> {
        (*self).into_rows()
    }

    fn col(&self, col: usize) -> Col<'_, T> {
        (*self).into_col(col)
    }

    /// # Examples
    ///
    /// ```
    /// use toodee::{TooDee,TooDeeOps,TooDeeOpsMut};
    /// unsafe {
    ///     let toodee : TooDee<u32> = TooDee::new(10, 5);
    ///     let view = toodee.view((0,0), (10,5));
    ///     let row = view.get_unchecked_row(3);
    ///     assert_eq!(row.len(), 10);
    /// }
    /// ```
    unsafe fn get_unchecked_row(&self, row: usize) -> &[T] {
        let start = row * self.stride;
        self.data.get_unchecked(start..start + self.num_cols)
    }

    /// # Examples
    ///
    /// ```
    /// use toodee::{TooDee,TooDeeOps,TooDeeOpsMut};
    /// let toodee : TooDee<u32> = TooDee::new(10, 5);
    /// let view = toodee.view((0,0), (10,5));
    /// unsafe {
    ///     assert_eq!(*view.get_unchecked((1,3)), 0);
    /// }
    /// ```
    unsafe fn get_unchecked(&self, coord: Coordinate) -> &T {
        self.data.get_unchecked(coord.1 * self.stride + coord.0)
    }
}

impl<'a, T> Index<usize> for TooDeeView<'a, T> {
    type Output = [T];

    fn index(&self, row: usize) -> &Self::Output {
        assert!(row < self.num_rows);
        let start = row * self.stride;
        unsafe {
            self.data.get_unchecked(start..start + self.num_cols)
        }
    }
}

impl<'a, T> Index<Coordinate> for TooDeeView<'a, T> {
    type Output = T;

    fn index(&self, coord: Coordinate) -> &Self::Output {
        assert!(coord.1 < self.num_rows);
        assert!(coord.0 < self.num_cols);
        // can access the element unchecked because the above assertions hold
        unsafe {
            self.data.get_unchecked(coord.1 * self.stride + coord.0)
        }
    }
}


/// Provides a mutable view (or subset), of a `TooDee` array.
///
/// Equality and hashing consider only the view's dimensions and the cells it
/// exposes (see [`TooDeeView`]).
pub struct TooDeeViewMut<'a, T> {
    data: &'a mut [T],
    num_cols: usize,
    num_rows: usize,
    stride: usize,
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
    /// Panics if `num_cols * num_rows` overflows.
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
        let size = num_cols.checked_mul(num_rows).unwrap();
        assert!(size <= data.len());
        unsafe {
            TooDeeViewMut {
                data: data.get_unchecked_mut(..size),
                num_cols,
                num_rows,
                stride: num_cols,
            }
        }
    }

    /// Used internally by `TooDee` to create a `TooDeeViewMut`.
    pub(super) fn from_toodee(start: Coordinate, end: Coordinate, toodee: &'a mut TooDee<T>) -> TooDeeViewMut<'a, T> {
        let stride = toodee.num_cols();
        let (num_cols, num_rows, data_range) = calculate_view_dimensions(start, end, toodee, stride);
        unsafe {
            TooDeeViewMut {
                data: toodee.data_mut().get_unchecked_mut(data_range),
                num_cols,
                num_rows,
                stride,
            }
        }
    }

    /// Consumes the view, returning a mutable iterator of slices, where each slice represents an entire row.
    ///
    /// Unlike [`TooDeeOpsMut::rows_mut()`], the returned iterator borrows the underlying data for `'a`
    /// rather than borrowing the view, so it can outlive the view.
    ///
    /// # Examples
    ///
    /// ```
    /// use toodee::{TooDee, TooDeeOps, TooDeeOpsMut, RowsMut};
    /// fn left_rows(toodee: &mut TooDee<u32>) -> RowsMut<'_, u32> {
    ///     toodee.view_mut((0, 0), (5, 5)).into_rows()
    /// }
    /// let mut toodee : TooDee<u32> = TooDee::init(10, 5, 42u32);
    /// for r in left_rows(&mut toodee) {
    ///     r.iter_mut().for_each(|c| *c = 0);
    /// }
    /// assert_eq!(toodee.cells().sum::<u32>(), 42*50 - 42*25);
    /// ```
    pub fn into_rows(self) -> RowsMut<'a, T> {
        RowsMut {
            v: self.data,
            cols: self.num_cols,
            skip_cols: self.stride - self.num_cols,
        }
    }

    /// Consumes the view, returning a mutable iterator over a single column.
    ///
    /// Unlike [`TooDeeOpsMut::col_mut()`], the returned iterator borrows the underlying data for `'a`
    /// rather than borrowing the view, so it can outlive the view.
    ///
    /// # Panics
    ///
    /// Panics if `col` is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use toodee::{TooDee, TooDeeOps, TooDeeOpsMut, ColMut};
    /// fn second_col(toodee: &mut TooDee<u32>) -> ColMut<'_, u32> {
    ///     toodee.view_mut((0, 0), (5, 5)).into_col(1)
    /// }
    /// let mut toodee : TooDee<u32> = TooDee::init(10, 5, 42u32);
    /// for c in second_col(&mut toodee) {
    ///     *c = 0;
    /// }
    /// assert_eq!(toodee.cells().sum::<u32>(), 42*50 - 42*5);
    /// ```
    pub fn into_col(self, col: usize) -> ColMut<'a, T> {
        let (data_range, skip) = self.get_col_params(col);
        unsafe {
            ColMut {
                v: self.data.get_unchecked_mut(data_range),
                skip,
            }
        }
    }

    /// Consumes the view, returning a mutable iterator that traverses all cells within the area.
    /// Equivalent to `view.into_iter()`.
    ///
    /// # Examples
    ///
    /// ```
    /// use toodee::{TooDee, TooDeeOps, TooDeeOpsMut, CellsMut};
    /// fn left_cells(toodee: &mut TooDee<u32>) -> CellsMut<'_, u32> {
    ///     toodee.view_mut((0, 0), (5, 5)).into_cells()
    /// }
    /// let mut toodee : TooDee<u32> = TooDee::init(10, 5, 42u32);
    /// for c in left_cells(&mut toodee) {
    ///     *c -= 1;
    /// }
    /// assert_eq!(toodee.cells().sum::<u32>(), 42*50 - 25);
    /// ```
    pub fn into_cells(self) -> CellsMut<'a, T> {
        FlattenExact::new(self.into_rows())
    }

    /// Returns a read-only view of the same area, borrowing this view.
    ///
    /// Equivalent to `self.view((0, 0), self.size())`, without recomputing the dimensions.
    /// To convert permanently, consuming this view, use `TooDeeView::from()`.
    ///
    /// # Examples
    ///
    /// ```
    /// use toodee::{TooDee, TooDeeOps, TooDeeOpsMut, TooDeeView};
    /// fn total(view: TooDeeView<'_, u32>) -> u32 {
    ///     view.into_cells().sum()
    /// }
    /// let mut toodee : TooDee<u32> = TooDee::init(10, 5, 1u32);
    /// let mut view = toodee.view_mut((0, 0), (5, 5));
    /// assert_eq!(total(view.as_view()), 25);
    /// view[0][0] = 10; // `view` is still usable
    /// assert_eq!(total(view.as_view()), 34);
    /// ```
    #[must_use]
    pub fn as_view(&self) -> TooDeeView<'_, T> {
        TooDeeView {
            data: self.data,
            num_cols: self.num_cols,
            num_rows: self.num_rows,
            stride: self.stride,
        }
    }

    /// Returns a mutable view of the same area with a shorter lifetime, borrowing this view,
    /// like `&mut *slice` does for `&mut [T]`.
    ///
    /// `TooDeeViewMut` can't be `Clone` or `Copy`, because two mutable views of the same cells
    /// would alias. Reborrowing is the alternative: use it to pass the view by value, or to call a
    /// consuming method such as [`into_rows()`](Self::into_rows), and then use the original view
    /// again once the reborrow has ended.
    ///
    /// Equivalent to `self.view_mut((0, 0), self.size())`, without recomputing the dimensions.
    ///
    /// # Examples
    ///
    /// ```
    /// use toodee::{TooDee, TooDeeOps, TooDeeOpsMut, TooDeeViewMut};
    /// fn clear(view: TooDeeViewMut<'_, u32>) {
    ///     view.into_iter().for_each(|c| *c = 0);
    /// }
    /// let mut toodee : TooDee<u32> = TooDee::init(10, 5, 1u32);
    /// let mut view = toodee.view_mut((0, 0), (5, 5));
    /// clear(view.reborrow());
    /// for r in view.reborrow().into_rows() {
    ///     r[0] = 7;
    /// }
    /// view[0][1] = 9; // `view` is still usable
    /// assert_eq!(toodee.cells().sum::<u32>(), 25 + 7*5 + 9);
    /// ```
    ///
    /// Only one reborrow can be alive at a time, so the views can never alias:
    ///
    /// ```compile_fail,E0499
    /// use toodee::{TooDee, TooDeeOps, TooDeeOpsMut};
    /// let mut toodee : TooDee<u32> = TooDee::init(10, 5, 1u32);
    /// let mut view = toodee.view_mut((0, 0), (5, 5));
    /// let mut a = view.reborrow();
    /// let mut b = view.reborrow();
    /// a[0][0] = 1;
    /// b[0][0] = 2;
    /// ```
    #[must_use]
    pub fn reborrow(&mut self) -> TooDeeViewMut<'_, T> {
        TooDeeViewMut {
            data: &mut *self.data,
            num_cols: self.num_cols,
            num_rows: self.num_rows,
            stride: self.stride,
        }
    }
}


impl<'a, T> TooDeeOps<T> for TooDeeViewMut<'a, T> {
    #[inline]
    fn num_rows(&self) -> usize {
        self.num_rows
    }

    #[inline]
    fn num_cols(&self) -> usize {
        self.num_cols
    }

    fn view(&self, start: Coordinate, end: Coordinate) -> TooDeeView<'_, T> {
        let (num_cols, num_rows, data_range) = calculate_view_dimensions(start, end, self, self.stride);
        TooDeeView {
            data: &self.data[data_range],
            num_cols,
            num_rows,
            stride: self.stride,
        }
    }

    fn rows(&self) -> Rows<'_, T> {
        self.as_view().into_rows()
    }

    fn col(&self, col: usize) -> Col<'_, T> {
        self.as_view().into_col(col)
    }

    /// # Examples
    ///
    /// ```
    /// use toodee::{TooDee,TooDeeOps,TooDeeOpsMut};
    /// unsafe {
    ///     let mut toodee : TooDee<u32> = TooDee::new(10, 5);
    ///     let mut view = toodee.view_mut((0,0), (10,5));
    ///     let row = view.get_unchecked_row(3);
    ///     assert_eq!(row.len(), 10);
    /// }
    /// ```
    unsafe fn get_unchecked_row(&self, row: usize) -> &[T] {
        let start = row * self.stride;
        self.data.get_unchecked(start..start + self.num_cols)
    }

    /// # Examples
    ///
    /// ```
    /// use toodee::{TooDee,TooDeeOps,TooDeeOpsMut};
    /// let mut toodee : TooDee<u32> = TooDee::new(10, 5);
    /// let mut view = toodee.view_mut((0,0), (10,5));
    /// unsafe {
    ///     assert_eq!(*view.get_unchecked((1,3)), 0);
    /// }
    /// ```
    unsafe fn get_unchecked(&self, coord: Coordinate) -> &T {
        self.data.get_unchecked(coord.1 * self.stride + coord.0)
    }
}

impl<'a, T> TooDeeOpsMut<T> for TooDeeViewMut<'a, T> {
    fn view_mut(&mut self, start: Coordinate, end: Coordinate) -> TooDeeViewMut<'_, T> {
        let (num_cols, num_rows, data_range) = calculate_view_dimensions(start, end, self, self.stride);
        unsafe {
            TooDeeViewMut {
                data: self.data.get_unchecked_mut(data_range),
                num_cols,
                num_rows,
                stride: self.stride,
            }
        }
    }

    fn rows_mut(&mut self) -> RowsMut<'_, T> {
        self.reborrow().into_rows()
    }

    fn col_mut(&mut self, col: usize) -> ColMut<'_, T> {
        self.reborrow().into_col(col)
    }

    /// Swap/exchange the data between two rows.
    ///
    /// # Panics
    ///
    /// Panics if either row index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use toodee::{TooDee,TooDeeOps,TooDeeOpsMut};
    /// let mut toodee : TooDee<u32> = TooDee::init(10, 5, 42u32);
    /// toodee[0].iter_mut().for_each(|v| *v = 1);
    /// assert_eq!(toodee[(0, 2)], 42);
    /// toodee.swap_rows(0, 2);
    /// assert_eq!(toodee[(0, 2)], 1);
    /// ```
    fn swap_rows(&mut self, mut r1: usize, mut r2: usize) {
        if r1 == r2 {
            return;
        }
        if r2 < r1 {
            mem::swap(&mut r1, &mut r2);
        }
        assert!(r2 < self.num_rows);
        let num_cols = self.num_cols;
        unsafe {
            let (first, rest) = self.data.get_unchecked_mut(r1 * self.stride..).split_at_mut(num_cols);
            let snd_idx = (r2 - r1) * self.stride - num_cols;
            let second = rest.get_unchecked_mut(snd_idx..snd_idx + num_cols);
            // Both slices are guaranteed to have the same length
            debug_assert_eq!(first.len(), num_cols);
            debug_assert_eq!(second.len(), num_cols);
            // We know that the two slices will not overlap because r1 != r2, and we used split_at_mut()
            ptr::swap_nonoverlapping(first.as_mut_ptr(), second.as_mut_ptr(), num_cols);
        }
    }

    /// # Examples
    ///
    /// ```
    /// use toodee::{TooDee,TooDeeOps,TooDeeOpsMut};
    /// unsafe {
    ///     let mut toodee : TooDee<u32> = TooDee::new(10, 5);
    ///     let mut view = toodee.view_mut((0,0), (10,5));
    ///     let row = view.get_unchecked_row_mut(3);
    ///     assert_eq!(row.len(), 10);
    /// }
    /// ```
    unsafe fn get_unchecked_row_mut(&mut self, row: usize) -> &mut [T] {
        let start = row * self.stride;
        self.data.get_unchecked_mut(start..start + self.num_cols)
    }


    /// # Examples
    ///
    /// ```
    /// use toodee::{TooDee,TooDeeOps,TooDeeOpsMut};
    /// let mut toodee : TooDee<u32> = TooDee::new(10, 5);
    /// let mut view = toodee.view_mut((0,0), (10,5));
    /// unsafe {
    ///     assert_eq!(*view.get_unchecked_mut((1,3)), 0);
    /// }
    /// ```
    unsafe fn get_unchecked_mut(&mut self, coord: Coordinate) -> &mut T {
        self.data.get_unchecked_mut(coord.1 * self.stride + coord.0)
    }
}

impl<'a, T> Index<usize> for TooDeeViewMut<'a, T> {
    type Output = [T];
    fn index(&self, row: usize) -> &Self::Output {
        assert!(row < self.num_rows);
        let start = row * self.stride;
        unsafe {
            self.data.get_unchecked(start..start + self.num_cols)
        }
    }
}

impl<'a, T> Index<Coordinate> for TooDeeViewMut<'a, T> {
    type Output = T;
    fn index(&self, coord: Coordinate) -> &Self::Output {
        assert!(coord.1 < self.num_rows);
        assert!(coord.0 < self.num_cols);
        // can access the element unchecked because the above assertions hold
        unsafe {
            self.data.get_unchecked(coord.1 * self.stride + coord.0)
        }
    }
}

impl<'a, T> IndexMut<usize> for TooDeeViewMut<'a, T> {
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        assert!(row < self.num_rows);
        let start = row * self.stride;
        unsafe {
            self.data.get_unchecked_mut(start..start + self.num_cols)
        }
    }
}

impl<'a, T> IndexMut<Coordinate> for TooDeeViewMut<'a, T> {
    fn index_mut(&mut self, coord: Coordinate) -> &mut Self::Output {
        assert!(coord.1 < self.num_rows);
        assert!(coord.0 < self.num_cols);
        // can access the element unchecked because the above assertions hold
        unsafe {
            self.data.get_unchecked_mut(coord.1 * self.stride + coord.0)
        }
    }
}

impl<'a, T> From<TooDeeViewMut<'a, T>> for TooDeeView<'a, T> {
    fn from(v: TooDeeViewMut<'a, T>) -> TooDeeView<'a, T> {
        TooDeeView {
            data: v.data,
            num_cols: v.num_cols,
            num_rows: v.num_rows,
            stride: v.stride,
        }
    }
}

impl<'a, T> IntoIterator for TooDeeView<'a, T> {
    type Item = &'a T;
    type IntoIter = Cells<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.into_cells()
    }
}

/// Consumes the view, yielding mutable references to its cells, like `IntoIterator` for `&mut [T]`.
///
/// Note: before 0.7.0, `view_mut.into_iter()` resolved to `(&view_mut).into_iter()` and yielded
/// `&T` without consuming the view. Use `view_mut.cells()` for that behaviour.
///
/// # Examples
///
/// ```
/// use toodee::{TooDee, TooDeeOps, TooDeeOpsMut};
/// let mut toodee : TooDee<u32> = TooDee::init(10, 5, 42u32);
/// for c in toodee.view_mut((0, 0), (5, 5)) {
///     *c = 0;
/// }
/// assert_eq!(toodee.cells().sum::<u32>(), 42*25);
/// ```
impl<'a, T> IntoIterator for TooDeeViewMut<'a, T> {
    type Item = &'a mut T;
    type IntoIter = CellsMut<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.into_cells()
    }
}

/// Yields references with the view's data lifetime `'a`, not the borrow's lifetime `'b`.
/// A `TooDeeView` is `Copy` and only shares its data, so borrowing it need not shorten
/// the lifetime of the items.
impl<'a, T> IntoIterator for &TooDeeView<'a, T> {
    type Item = &'a T;
    type IntoIter = Cells<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        (*self).into_cells()
    }
}

impl<'a, 'b, T> IntoIterator for &'b TooDeeViewMut<'a, T> {
    type Item = &'b T;
    type IntoIter = Cells<'b, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.cells()
    }
}

impl<'a, 'b, T> IntoIterator for &'b mut TooDeeViewMut<'a, T> {
    type Item = &'b mut T;
    type IntoIter = CellsMut<'b, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.cells_mut()
    }
}

impl<T> Debug for TooDeeView<'_, T> where T: Debug {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.rows()).finish()
    }
}

impl<T> Debug for TooDeeViewMut<'_, T> where T: Debug {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.rows()).finish()
    }
}

/// Compares two arrays/views by their dimensions and visible cells only.
#[inline]
pub(crate) fn cells_eq<T, U, A, B>(a: &A, b: &B) -> bool
where
    T: PartialEq<U>,
    A: TooDeeOps<T> + ?Sized,
    B: TooDeeOps<U> + ?Sized,
{
    a.num_cols() == b.num_cols()
        && a.num_rows() == b.num_rows()
        && a.rows().zip(b.rows()).all(|(x, y)| x == y)
}

/// Hashes an array/view by its dimensions and visible cells only, so that a
/// `TooDee` and any view with the same dimensions and cells hash identically.
#[inline]
pub(crate) fn cells_hash<T, A, H>(a: &A, state: &mut H)
where
    T: Hash,
    A: TooDeeOps<T> + ?Sized,
    H: Hasher,
{
    a.num_cols().hash(state);
    a.num_rows().hash(state);
    for row in a.rows() {
        row.hash(state);
    }
}

macro_rules! impl_cells_eq {
    ($(($lhs:ty, $rhs:ty)),* $(,)?) => {
        $(
            impl<T, U> PartialEq<$rhs> for $lhs where T: PartialEq<U> {
                #[inline]
                fn eq(&self, other: &$rhs) -> bool {
                    cells_eq(self, other)
                }
            }
        )*
    };
}

impl_cells_eq!(
    (TooDeeView<'_, T>, TooDeeView<'_, U>),
    (TooDeeView<'_, T>, TooDeeViewMut<'_, U>),
    (TooDeeViewMut<'_, T>, TooDeeView<'_, U>),
    (TooDeeViewMut<'_, T>, TooDeeViewMut<'_, U>),
    (TooDee<T>, TooDeeView<'_, U>),
    (TooDee<T>, TooDeeViewMut<'_, U>),
    (TooDeeView<'_, T>, TooDee<U>),
    (TooDeeViewMut<'_, T>, TooDee<U>),
);

impl<T: Eq> Eq for TooDeeView<'_, T> {}
impl<T: Eq> Eq for TooDeeViewMut<'_, T> {}

impl<T: Hash> Hash for TooDeeView<'_, T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        cells_hash(self, state);
    }
}

impl<T: Hash> Hash for TooDeeViewMut<'_, T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        cells_hash(self, state);
    }
}
