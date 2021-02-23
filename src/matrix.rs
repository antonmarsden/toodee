use core::fmt;
use core::fmt::{ Formatter, Debug };
use core::ops::{Index, IndexMut};
use core::iter::IntoIterator;
use core::ptr;
use core::cmp::Ordering;

extern crate alloc;

use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;
use alloc::vec::IntoIter;
use crate::iter::*;
use crate::view::*;
use crate::ops::*;

pub type IntoIterTooDee<T> = IntoIter<T>;

/// Represents a two-dimensional array.
/// 
/// Empty arrays will always have dimensions of zero.
#[derive(Hash, Eq, PartialEq)]
pub struct Matrix<T, const C : usize, const R : usize> {
    data: Box<[T]>,
}

impl<T, const C : usize, const R : usize> Index<usize> for Matrix<T, C, R> {
    type Output = [T];
    fn index(&self, row: usize) -> &Self::Output {
        assert!(row < R);
        let start = row * C;
        // can access the element unchecked because the above assertion holds
        unsafe {
            self.data.get_unchecked(start..start + C)
        }
    }
}

impl<T, const C : usize, const R : usize> Index<Coordinate> for Matrix<T, C, R> {
    type Output = T;
    fn index(&self, coord: Coordinate) -> &Self::Output {
        assert!(coord.1 < R);
        assert!(coord.0 < C);
        // can access the element unchecked because the above assertions hold
        unsafe {
            self.data.get_unchecked(coord.1 * C + coord.0)
        }
    }
}


impl<T, const C : usize, const R : usize> IndexMut<usize> for Matrix<T, C, R> {

    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        assert!(row < R);
        let start = row * C;
        // can access the element unchecked because the above assertion holds
        unsafe {
            self.data.get_unchecked_mut(start..start + C)
        }
    }
}

impl<T, const C : usize, const R : usize> IndexMut<Coordinate> for Matrix<T, C, R> {

    fn index_mut(&mut self, coord: Coordinate) -> &mut Self::Output {
        assert!(coord.1 < R);
        assert!(coord.0 < C);
        // can access the element unchecked because the above assertions hold
        unsafe {
            self.data.get_unchecked_mut(coord.1 * C + coord.0)
        }
    }
}

impl<T, const C : usize, const R : usize> TooDeeOps<T> for Matrix<T, C, R> {
    
    fn num_cols(&self) -> usize {
        C
    }

    fn num_rows(&self) -> usize {
        R
    }

    fn bounds(&self) -> (Coordinate, Coordinate) {
        ((0, 0), (C, R))
    }
    
    fn view(&self, start: Coordinate, end: Coordinate) -> TooDeeView<'_, T> {
        TooDeeView::from_matrix(start, end, self)
    }
    
    fn rows(&self) -> Rows<'_, T> {
        Rows {
            v : &self.data,
            cols : C,
            skip_cols : 0,
        }
    }
    
    fn col(&self, col: usize) -> Col<'_, T> {
        assert!(col < C);
        unsafe {
            Col {
                v : self.data.get_unchecked(col..self.data.len() - C + col + 1),
                skip : C - 1,
            }
        }
    }

    unsafe fn get_unchecked_row(&self, row: usize) -> &[T] {
        let start = row * C;
        self.data.get_unchecked(start..start + C)
    }

    unsafe fn get_unchecked(&self, coord: Coordinate) -> &T {
        self.data.get_unchecked(coord.1 * C + coord.0)
    }

}

impl<T, const C : usize, const R : usize> TooDeeOpsMut<T> for Matrix<T, C, R> {
    
    fn view_mut(&mut self, start: Coordinate, end: Coordinate) -> TooDeeViewMut<'_, T> {
        TooDeeViewMut::from_matrix(start, end, self)
    }
    
    fn rows_mut(&mut self) -> RowsMut<'_, T> {
        RowsMut {
            v : &mut self.data,
            cols : C,
            skip_cols : 0,
        }
    }
    
    fn col_mut(&mut self, col: usize) -> ColMut<'_, T> {
        assert!(col < C);
        let dlen = self.data.len();
        unsafe {
            ColMut {
                v : self.data.get_unchecked_mut(col..dlen - C + col + 1),
                skip : C - 1,
            }
        }
    }
    
    fn fill(&mut self, fill: T)
    where T: Clone {
        self.data.fill(fill);
    }

    fn swap_rows(&mut self, mut r1: usize, mut r2: usize) {
        match r1.cmp(&r2) {
            Ordering::Less => {},
            Ordering::Greater => {
                // force r1 < r2
                core::mem::swap(&mut r1, &mut r2);
            },
            Ordering::Equal => {
                // swapping a row with itself
                return;
            }
        }
        assert!(r2 < R);
        let num_cols = C;
        unsafe {
            let (first, rest) = self.data.get_unchecked_mut(r1 * num_cols..).split_at_mut(num_cols);
            let snd_idx = (r2 - r1 - 1) * num_cols;
            let second = rest.get_unchecked_mut(snd_idx..snd_idx + num_cols);
            // Both slices are guaranteed to have the same length
            debug_assert_eq!(first.len(), num_cols);
            debug_assert_eq!(second.len(), num_cols);
            // We know that the two slices will not overlap because r1 != r2, and we used split_at_mut()
            ptr::swap_nonoverlapping(first.as_mut_ptr(), second.as_mut_ptr(), num_cols);
        }
    }
    
    unsafe fn get_unchecked_row_mut(&mut self, row: usize) -> &mut [T] {
        let start = row * C;
        self.data.get_unchecked_mut(start..start + C)
    }

    unsafe fn get_unchecked_mut(&mut self, coord: Coordinate) -> &mut T {
        self.data.get_unchecked_mut(coord.1 * C + coord.0)
    }

}

impl<T : Default + Clone, const C : usize, const R : usize> Default for Matrix<T, C, R> {
    /// Create a new `Matrix` and fill it with
    /// the type's default value.
    /// 
    /// # Panics
    /// 
    /// Panics if one of the dimensions is zero but the other is non-zero. This
    /// is to enforce the rule that empty arrays have `size() == (0, 0)`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use toodee::{Matrix,TooDeeOps,TooDeeOpsMut};
    /// let matrix : Matrix<u32, 10, 5> = <Matrix<u32, 10, 5>>::default();
    /// assert_eq!(matrix.num_cols(), 10);
    /// assert_eq!(matrix.num_rows(), 5);
    /// assert_eq!(matrix[0][0], 0);
    /// ```
    fn default() -> Matrix<T, C, R> {
        if C == 0 || R == 0 {
            assert_eq!(R, C);
        }
        Matrix {
            data : Box::from(vec![T::default() ; C*R])
        }
    }
}

impl<T, const C : usize, const R : usize> Matrix<T, C, R> {

    /// Create a new `Matrix` and fill it with `init_value`.
    /// 
    /// # Panics
    /// 
    /// Panics if one of the dimensions is zero but the other is non-zero. This
    /// is to enforce the rule that empty arrays have `size() == (0, 0)`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use toodee::{Matrix,TooDeeOps,TooDeeOpsMut};
    /// let matrix : Matrix<u32, 10, 5> = <Matrix<u32, 10, 5>>::init(42);
    /// assert_eq!(matrix.num_cols(), 10);
    /// assert_eq!(matrix.num_rows(), 5);
    /// assert_eq!(matrix[0][0], 42);
    /// ```
    pub fn init(init_value: T) -> Matrix<T, C, R>
    where T: Clone {
        if C == 0 || R == 0 {
            assert_eq!(R, C);
        }
        Matrix {
            data : Box::from(vec![init_value ; C*R])
        }
    }
    
    pub fn from_vec(v: Vec<T>) -> Matrix<T, C, R> {
        assert_eq!(C * R, v.len());
        Matrix {
            data : v.into_boxed_slice(),
        }
    }
    
    pub fn from_box(b: Box<[T]>) -> Matrix<T, C, R> {
        Matrix::from_vec(b.into_vec())
    }
    
    pub fn data(&self) -> &[T] {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut [T] {
        &mut self.data
    }

}

impl<'a, T, const C : usize, const R : usize> IntoIterator for Matrix<T, C, R> {
    type Item = T;
    type IntoIter = IntoIterTooDee<T>;
    // TODO: avoid slice -> vec -> iter
    fn into_iter(self) -> Self::IntoIter {
        self.data.into_vec().into_iter()
    }
}

impl<'a, T, const C : usize, const R : usize> IntoIterator for &'a Matrix<T, C, R> {
    type Item = &'a T;
    type IntoIter = Cells<'a, T>;
    /// `Cells` is the preferred iterator type here, because it implements `TooDeeIterator`
    fn into_iter(self) -> Self::IntoIter {
        self.cells()
    }
}

impl<'a, T, const C : usize, const R : usize> IntoIterator for &'a mut Matrix<T, C, R> {
    type Item = &'a mut T;
    /// `CellsMut` is the preferred iterator type here, because it implements `TooDeeIterator`
    type IntoIter = CellsMut<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.cells_mut()
    }
}

/// Support conversion into a `Vec`.
impl<T, const C : usize, const R : usize> From<Matrix<T, C, R>> for Vec<T> {
    fn from(toodee: Matrix<T, C, R>) -> Vec<T> {
        Vec::from(toodee.data)
    }
}

/// Support conversion into a boxed slice.
impl<T, const C : usize, const R : usize> From<Matrix<T, C, R>> for Box<[T]> {
    fn from(toodee: Matrix<T, C, R>) -> Box<[T]> {
        toodee.data
    }
}

impl<T, const C : usize, const R : usize> AsRef<[T]> for Matrix<T, C, R> {
    fn as_ref(&self) -> &[T] {
        &self.data
    }
}

impl<'a, T, const C : usize, const R : usize> AsMut<[T]> for Matrix<T, C, R> {
    fn as_mut(&mut self) -> &mut [T] {
        &mut self.data
    }
}

impl<'a, T, const C : usize, const R : usize> Debug for Matrix<T, C, R> where T : Debug {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.rows()).finish()
    }
}

impl<'a, T, const C : usize, const R : usize> From<TooDeeView<'_, T>> for Matrix<T, C, R> where T : Clone {
    fn from(view: TooDeeView<'_, T>) -> Self {
        assert_eq!(C, view.num_cols());
        assert_eq!(R, view.num_rows());
        let mut v = Vec::with_capacity(C * R);
        for r in view.rows() {
            v.extend_from_slice(r);
        }
        Matrix {
            data : v.into_boxed_slice(),
        }
    }
}

impl<'a, T, const C : usize, const R : usize> From<TooDeeViewMut<'_, T>> for Matrix<T, C, R> where T : Clone {
    fn from(view: TooDeeViewMut<'_, T>) -> Self {
        assert_eq!(C, view.num_cols());
        assert_eq!(R, view.num_rows());
        let mut v = Vec::with_capacity(C * R);
        for r in view.rows() {
            v.extend_from_slice(r);
        }
        Matrix {
            data : v.into_boxed_slice(),
        }
    }
}
