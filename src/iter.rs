use core::mem;

/// An `Iterator` that knows how many columns it emits per row.
pub trait TooDeeIterator : Iterator {
    /// The number of columns the iterator emits per row
    fn num_cols(&self) -> usize;
}

/// An `Iterator` over each row of a `TooDee[View]`, where each row is represented as a slice.
#[derive(Debug)]
pub struct Rows<'a, T> {
    /// This reference contains row data at each end. When iterating in either direction the row will
    /// be pulled off the end then `skip_cols` elements will be skipped in preparation for reading the
    /// next row.
    pub(super) v: &'a [T],
    pub(super) cols: usize,
    pub(super) skip_cols: usize,
}

impl<'a, T> Iterator for Rows<'a, T> {

    type Item = &'a [T];

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.v.is_empty() {
            None
        } else {
            let (fst, snd) = self.v.split_at(self.cols);
            if snd.is_empty() {
                self.v = &[];
            } else {
                // snd must contain at least one row, so no check required
                unsafe {
                    self.v = snd.get_unchecked(self.skip_cols..);
                }
            }
            Some(fst)
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.cols == 0 {
            return (0, Some(0));
        }
        let len = self.v.len();
        let denom = self.cols + self.skip_cols;
        let n = len / denom + (len % denom) / self.cols;
        (n, Some(n))
    }

    #[inline]
    fn count(self) -> usize {
        self.len()
    }
    
    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        
        let (start, overflow) = n.overflowing_mul(self.cols + self.skip_cols);
        if start >= self.v.len() || overflow {
            self.v = &[];
        } else {
            let (_, snd) = self.v.split_at(start);
            self.v = snd;
        }
        self.next()
    }

    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }    
}

impl<'a, T> DoubleEndedIterator for Rows<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.v.is_empty() {
            None
        } else {
            let (fst, snd) = self.v.split_at(self.v.len() - self.cols);
            if fst.is_empty() {
                self.v = &[];
            } else {
                // skip_cols will be <= fst.len(), so no check required
                unsafe {
                    self.v = fst.get_unchecked(..fst.len() - self.skip_cols);
                }
            }
            Some(&snd)
        }
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        let (adj, overflow) = n.overflowing_mul(self.cols + self.skip_cols);
        if adj >= self.v.len() || overflow {
            self.v = &[];
        } else {
            // adj < fst.len(), so no check required
            unsafe {
                self.v = self.v.get_unchecked(..self.v.len() - adj);
            }
        }
        self.next_back()
    }
}

impl<T> ExactSizeIterator for Rows<'_, T> {}

impl<T> TooDeeIterator for Rows<'_, T> {
    fn num_cols(&self) -> usize {
        self.cols
    }
}

/// A mutable Iterator over each row of a `TooDee[ViewMut]`, where each row is represented as a slice.
#[derive(Debug)]
pub struct RowsMut<'a, T> {
    /// This reference contains row data at each end. When iterating in either direction the row will
    /// be pulled off the end then `skip_cols` elements will be skipped in preparation for reading the
    /// next row.
    pub(super) v: &'a mut [T],
    pub(super) cols: usize,
    pub(super) skip_cols: usize,
}

impl<'a, T> Iterator for RowsMut<'a, T> {

    type Item = &'a mut [T];

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.v.is_empty() {
            None
        } else {
            let tmp = mem::replace(&mut self.v, &mut []);
            let (head, tail) = tmp.split_at_mut(self.cols);
            if tail.is_empty() {
                self.v = &mut [];
            } else {
                // tail must contain at least one row, so no check required
                unsafe {
                    self.v = tail.get_unchecked_mut(self.skip_cols..);
                }
            }
            Some(head)
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.cols == 0 {
            return (0, Some(0));
        }
        let len = self.v.len();
        let denom = self.cols + self.skip_cols;
        let n = len / denom + (len % denom) / self.cols;
        (n, Some(n))
    }

    #[inline]
    fn count(self) -> usize {
        self.len()
    }
    
    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let (start, overflow) = n.overflowing_mul(self.cols + self.skip_cols);
        if start >= self.v.len() || overflow {
            self.v = &mut [];
        } else {
            let tmp = mem::replace(&mut self.v, &mut []);
            let (_, snd) = tmp.split_at_mut(start);
            self.v = snd;
        }
        self.next()
    }
    
    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }    
}

impl<'a, T> DoubleEndedIterator for RowsMut<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.v.is_empty() {
            None
        } else {
            let tmp = mem::replace(&mut self.v, &mut []);
            let tmp_len = tmp.len();
            let (fst, snd) = tmp.split_at_mut(tmp_len - self.cols);
            if fst.is_empty() {
                self.v = &mut [];
            } else {
                // fst must contain at least one row, so no check required
                unsafe {
                    self.v = fst.get_unchecked_mut(..tmp_len - self.cols - self.skip_cols);
                }
            }
            Some(snd)
        }
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {

        let (adj, overflow) = n.overflowing_mul(self.cols + self.skip_cols);
        if adj >= self.v.len() || overflow {
            self.v = &mut [];
        } else {
            let tmp = mem::replace(&mut self.v, &mut []);
            // adj < self.v.len(), so no check required
            unsafe {
                self.v = tmp.get_unchecked_mut(..self.v.len() - adj);
            }
        }
        self.next_back()
    }
}

impl<T> ExactSizeIterator for RowsMut<'_, T> {}

impl<T> TooDeeIterator for RowsMut<'_, T> {
    fn num_cols(&self) -> usize {
        self.cols
    }
}

/// An iterator over a single column.
#[derive(Debug)]
pub struct Col<'a, T> {
    pub(super) v: &'a [T],
    pub(super) skip: usize,
}

impl<'a, T> Iterator for Col<'a, T> {

    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if let Some((fst, snd)) = self.v.split_first() {
            if snd.is_empty() {
                self.v = &[];
            } else {
                // snd must contain at least one row, so we don't need a bounds check
                unsafe {
                    self.v = &snd.get_unchecked(self.skip..);
                }
            }
            Some(fst)
        } else {
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.v.len();
        let denom = 1 + self.skip;
        let n = len / denom + (len % denom);
        (n, Some(n))
    }

    #[inline]
    fn count(self) -> usize {
        self.len()
    }
    
    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        
        let (start, overflow) = n.overflowing_mul(1 + self.skip);
        if start >= self.v.len() || overflow {
            self.v = &[];
        } else {
            let (_, snd) = self.v.split_at(start);
            self.v = snd;
        }
        self.next()
    }

    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}

impl<'a, T> DoubleEndedIterator for Col<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if let Some((last, fst)) = self.v.split_last() {
            if fst.is_empty() {
                self.v = &[];
            } else {
                // fst must contain at least one row, so we don't need a bounds check
                unsafe {
                    self.v = &fst.get_unchecked(..fst.len() - self.skip);
                }
            }
            Some(last)
        } else {
            None
        }
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        let (adj, overflow) = n.overflowing_mul(1 + self.skip);
        if adj >= self.v.len() || overflow {
            self.v = &[];
        } else {
            // adj < self.v.len(), so no check required
            unsafe {
                self.v = self.v.get_unchecked(..self.v.len() - adj);
            }
        }
        self.next_back()
    }
}

impl<T> ExactSizeIterator for Col<'_, T> {}


/// A mutable iterator over a single column.
#[derive(Debug)]
pub struct ColMut<'a, T> {
    pub(super) v: &'a mut [T],
    pub(super) skip: usize,
}

impl<'a, T> Iterator for ColMut<'a, T> {

    type Item = &'a mut T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let tmp = mem::replace(&mut self.v, &mut []);
        if let Some((fst, snd)) = tmp.split_first_mut() {
            if snd.is_empty() {
                self.v = &mut [];
            } else {
                // snd must contain at least one row, so no check required
                unsafe {
                    self.v = snd.get_unchecked_mut(self.skip..);
                }
            }
            Some(fst)
        } else {
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.v.len();
        let denom = 1 + self.skip;
        let n = len / denom + (len % denom);
        (n, Some(n))
    }

    #[inline]
    fn count(self) -> usize {
        self.len()
    }
    
    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let (start, overflow) = n.overflowing_mul(1 + self.skip);
        if start >= self.v.len() || overflow {
            self.v = &mut [];
        } else {
            let tmp = mem::replace(&mut self.v, &mut []);
            let (_, snd) = tmp.split_at_mut(start);
            self.v = snd;
        }
        self.next()
    }
    
    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }    
}

impl<'a, T> DoubleEndedIterator for ColMut<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        let tmp = mem::replace(&mut self.v, &mut []);
        if let Some((last, fst)) = tmp.split_last_mut() {
            if fst.is_empty() {
                self.v = &mut [];
            } else {
                let new_len = fst.len() - self.skip;
                // skip <= fst.len(), so no check required
                unsafe {
                    self.v = fst.get_unchecked_mut(..new_len);
                }
            }
            Some(last)
        } else {
            None
        }
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {

        let (adj, overflow) = n.overflowing_mul(1 + self.skip);
        if adj >= self.v.len() || overflow {
            self.v = &mut [];
        } else {
            let tmp = mem::replace(&mut self.v, &mut []);
            // adj <= self.v.len(), so no check required
            unsafe {
                self.v = tmp.get_unchecked_mut(..self.v.len() - adj);
            }
        }
        self.next_back()
    }
}

impl<T> ExactSizeIterator for ColMut<'_, T> {}

