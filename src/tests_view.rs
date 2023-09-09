#[cfg(test)]
mod toodee_tests_view {

    extern crate alloc;
    use alloc::boxed::Box;
    use alloc::vec;
    use alloc::vec::Vec;

    use crate::*;

    struct IteratorWithWrongLength();

    impl Iterator for IteratorWithWrongLength {
        type Item = Box<u8>;

        fn next(&mut self) -> Option<Self::Item> { None }
    }

    impl ExactSizeIterator for IteratorWithWrongLength {
        fn len(&self) -> usize { 1 }
    }

    #[test]
    fn new_view_direct() {
        let v = vec![1u32; 32];
        let view = TooDeeView::new(4, 8, &v);
        assert_eq!((4, 8), view.size());
        assert_eq!(view.num_cols(), 4);
        assert_eq!(view.num_rows(), 8);
        assert_eq!(view.cells().sum::<u32>(), 32);
    }

    #[test]
    fn new_view_mut_direct() {
        let mut v = vec![1u32; 32];
        let view = TooDeeViewMut::new(8, 4, &mut v);
        assert_eq!((8, 4), view.size());
        assert_eq!(view.num_cols(), 8);
        assert_eq!(view.num_rows(), 4);
        assert_eq!(view.cells().sum::<u32>(), 32);
    }

    #[test]
    fn view_from_into_toodee() {
        let toodee = TooDee::from_vec(10, 10, (0u32..100).collect());
        let view = toodee.view((2, 2), (4, 4));
        let mut subdee: TooDee<u32> = view.into();
        assert_eq!(subdee.data().iter().sum::<u32>(), 22+23+32+33);
        subdee = TooDee::from(view);
        assert_eq!(subdee.data().iter().sum::<u32>(), 22+23+32+33);
    }

    #[test]
    fn view_mut_into_toodee() {
        let mut toodee = TooDee::from_vec(10, 10, (0u32..100).collect());
        let view = toodee.view_mut((2, 2), (4, 4));
        let subdee: TooDee<u32> = view.into();
        assert_eq!(subdee.data().iter().sum::<u32>(), 22+23+32+33);
    }

    #[test]
    fn view_mut_from_toodee() {
        let mut toodee = TooDee::from_vec(10, 10, (0u32..100).collect());
        let view = toodee.view_mut((2, 2), (4, 4));
        let subdee = TooDee::from(view);
        assert_eq!(subdee.data().iter().sum::<u32>(), 22+23+32+33);
    }

    #[test]
    fn view_mut_to_toodee() {
        let mut toodee = TooDee::from_vec(10, 10, (0u32..100).collect());
        let view : TooDeeView<'_, u32> = toodee.view_mut((2, 2), (4, 4)).into();
        let subdee: TooDee<u32> = view.into();
        assert_eq!(subdee.data().iter().sum::<u32>(), 22+23+32+33);
    }

    #[test]
    fn view_mut_into_iter() {
        let mut toodee = TooDee::from_vec(10, 10, (0u32..100).collect());
        let view = toodee.view_mut((2, 2), (4, 4));
        assert_eq!(view.into_iter().copied().sum::<u32>(), 22+23+32+33);
    }

    #[test]
    fn view_mut_into_iter_2() {
        let mut toodee = TooDee::from_vec(10, 10, (0u32..100).collect());
        let view = toodee.view_mut((2, 2), (4, 4));
        assert_eq!(view.into_iter().copied().sum::<u32>(), 22+23+32+33);
    }

    #[test]
    fn view_into_iter() {
        let toodee = TooDee::from_vec(10, 10, (0u32..100).collect());
        let view = toodee.view((2, 2), (4, 4));
        assert_eq!(view.into_iter().copied().sum::<u32>(), 22+23+32+33);
    }

    #[test]
    fn zero_size_view() {
        let mut toodee = TooDee::init(10, 10, 0u32);
        let mut view = toodee.view_mut((5, 5), (5, 5));
        assert!(view.is_empty());
        assert_eq!(view.rows_mut().next(), None);
        assert_eq!(view.rows().next(), None);
        assert_eq!(view.cells().next(), None);
        assert_eq!(view.cells_mut().next(), None);
        view = toodee.view_mut((5, 5), (6, 5));
        assert_eq!(view.rows_mut().next(), None);
        assert_eq!(view.rows().next(), None);
        assert_eq!(view.cells().next(), None);
        assert_eq!(view.cells_mut().next(), None);
        view = toodee.view_mut((5, 5), (5, 6));
        assert_eq!(view.rows_mut().next(), None);
        assert_eq!(view.rows().next(), None);
        assert_eq!(view.cells().next(), None);
        assert_eq!(view.cells_mut().next(), None);
    }

    #[test]
    fn zero_size_view_of_zero() {
        let mut toodee = TooDee::init(0, 0, 0u32);
        let mut view = toodee.view_mut((0, 0), (0, 0));
        assert_eq!(view.rows_mut().next(), None);
        assert_eq!(view.rows().next(), None);
        assert_eq!(view.cells().next(), None);
        assert_eq!(view.cells_mut().next(), None);
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn zero_size_view_col() {
        let toodee = TooDee::init(0, 0, 0u32);
        let view = toodee.view((0, 0), (0, 0));
        view.col(0);
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn zero_size_view_mut_col_mut() {
        let mut toodee = TooDee::init(0, 0, 0u32);
        let mut view = toodee.view_mut((0, 0), (0, 0));
        view.col_mut(0);
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn zero_size_view_mut_col() {
        let mut toodee = TooDee::init(0, 0, 0u32);
        let view = toodee.view_mut((0, 0), (0, 0));
        view.col(0);
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn toodee_view_new_overflow() {
        // const orig: TooDee<u32> = TooDee::new(1, 1);
        TooDeeView::<u32>::new(usize::MAX, usize::MAX, &[0u32] );
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn toodee_view_mut_new_overflow() {
        // const orig: TooDee<u32> = TooDee::new(1, 1);
        TooDeeViewMut::<u32>::new(usize::MAX, usize::MAX, &mut [0u32]);
    }

    #[test]
    fn swap() {
        let mut toodee = TooDee::from_vec(5, 5, (0u32..25).collect());
        let mut view = toodee.view_mut((1, 1), (4, 4));
        assert_eq!(&view.cells().map(|arg| { arg.clone() }).collect::<Vec<u32>>(), &[6, 7, 8, 11, 12, 13, 16, 17, 18]);
        view.swap((0,0),(2, 2));
        assert_eq!(&view.cells().map(|arg| { arg.clone() }).collect::<Vec<u32>>(), &[18, 7, 8, 11, 12, 13, 16, 17, 6]);
        view.swap((2,2),(0, 0));
        assert_eq!(&view.cells().map(|arg| { arg.clone() }).collect::<Vec<u32>>(), &[6, 7, 8, 11, 12, 13, 16, 17, 18]);
        view.swap((0,2),(1, 1));
        assert_eq!(&view.cells().map(|arg| { arg.clone() }).collect::<Vec<u32>>(), &[6, 7, 8, 11, 16, 13, 12, 17, 18]);
        view.swap((1,1),(1, 1));
        assert_eq!(&view.cells().map(|arg| { arg.clone() }).collect::<Vec<u32>>(), &[6, 7, 8, 11, 16, 13, 12, 17, 18]);
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn swap_out_of_bounds() {
        let mut toodee = TooDee::from_vec(5, 5, (0u32..25).collect());
        let mut view = toodee.view_mut((1, 1), (4, 4));
        view.swap((0,0), (1,3));
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn swap_out_of_bounds_2() {
        let mut toodee = TooDee::from_vec(3, 3, (0u32..9).collect());
        let mut view = toodee.view_mut((1, 1), (4, 4));
        view.swap((3,0), (1,1));
    }

}
