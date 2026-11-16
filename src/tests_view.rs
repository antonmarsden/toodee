#[cfg(test)]
mod toodee_tests_view {

    extern crate alloc;
    use alloc::vec;
    use alloc::vec::Vec;

    use crate::*;

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
        view.into_iter().for_each(|c| *c += 1000);
        assert_eq!(toodee.cells().filter(|&&c| c >= 1000).count(), 4);
        assert_eq!(toodee.view((2, 2), (4, 4)).cells().sum::<u32>(), 4000+22+23+32+33);
    }

    #[test]
    fn view_mut_into_iter_by_value_outlives_view() {
        fn cells(toodee: &mut TooDee<u32>) -> impl Iterator<Item = &mut u32> {
            toodee.view_mut((2, 2), (4, 4)).into_iter()
        }
        let mut toodee = TooDee::from_vec(10, 10, (0u32..100).collect());
        cells(&mut toodee).for_each(|c| *c = 0);
        for c in toodee.view_mut((2, 2), (4, 4)) {
            *c += 1;
        }
        assert_eq!(toodee.view((2, 2), (4, 4)).cells().sum::<u32>(), 4);
    }

    #[test]
    fn view_mut_ref_into_iter() {
        let mut toodee = TooDee::from_vec(10, 10, (0u32..100).collect());
        let mut view = toodee.view_mut((2, 2), (4, 4));
        assert_eq!((&view).into_iter().copied().sum::<u32>(), 22+23+32+33);
        for c in &mut view {
            *c += 1;
        }
        assert_eq!(view.cells().sum::<u32>(), 4+22+23+32+33);
    }

    #[test]
    fn view_into_iter() {
        let toodee = TooDee::from_vec(10, 10, (0u32..100).collect());
        let view = toodee.view((2, 2), (4, 4));
        assert_eq!(view.into_iter().copied().sum::<u32>(), 22+23+32+33);
    }

    #[test]
    fn view_into_iter_by_value_outlives_view() {
        fn cells(toodee: &TooDee<u32>) -> impl Iterator<Item = &u32> {
            toodee.view((2, 2), (4, 4)).into_iter()
        }
        let toodee = TooDee::from_vec(10, 10, (0u32..100).collect());
        assert_eq!(cells(&toodee).copied().sum::<u32>(), 22+23+32+33);
        let mut sum = 0;
        for c in toodee.view((2, 2), (4, 4)) {
            sum += *c;
        }
        assert_eq!(sum, 22+23+32+33);
    }

    #[test]
    fn view_into_rows_col_cells() {
        let toodee = TooDee::from_vec(10, 10, (0u32..100).collect());
        let view = toodee.view((2, 2), (5, 4));
        assert!(view.into_rows().eq(view.rows()));
        assert!(view.into_col(1).eq(view.col(1)));
        assert!(view.into_cells().eq(view.cells()));
        assert_eq!(view.into_rows().len(), 2);
        assert_eq!(view.into_col(2).copied().collect::<Vec<_>>(), vec![24, 34]);
        assert_eq!(view.into_cells().rev().copied().collect::<Vec<_>>(), vec![34, 33, 32, 24, 23, 22]);
    }

    #[test]
    fn view_is_copy_for_non_copy_t() {
        let toodee = TooDee::from_vec(2, 2, vec![Vec::<u32>::new(), vec![1], vec![2], vec![3]]);
        let view = toodee.view((0, 0), (2, 2));
        let copy = view;
        assert_eq!(view.into_col(1).map(|v| v.len()).sum::<usize>(), 2);
        assert_eq!(copy.into_cells().count(), 4);
    }

    #[test]
    #[should_panic]
    fn view_into_col_out_of_bounds() {
        let toodee = TooDee::from_vec(10, 10, (0u32..100).collect());
        let _ = toodee.view((2, 2), (5, 4)).into_col(3);
    }

    #[test]
    #[should_panic]
    fn view_mut_into_col_out_of_bounds() {
        let mut toodee = TooDee::from_vec(10, 10, (0u32..100).collect());
        let _ = toodee.view_mut((2, 2), (5, 4)).into_col(3);
    }

    #[test]
    #[should_panic]
    fn empty_view_into_col_panics() {
        let toodee = TooDee::init(4, 4, 1u32);
        let _ = toodee.view((1, 1), (1, 1)).into_col(0);
    }

    #[test]
    #[should_panic]
    fn empty_view_mut_into_col_panics() {
        let mut toodee = TooDee::init(4, 4, 1u32);
        let _ = toodee.view_mut((1, 1), (1, 1)).into_col(0);
    }

    #[test]
    fn view_into_methods_through_shared_ref() {
        let toodee = TooDee::init(4, 4, 1u32);
        let view = toodee.view((0, 0), (2, 2));
        let r = &view;
        assert_eq!(r.into_cells().count(), 4);
        assert_eq!(r.into_col(1).count(), 2);
        assert_eq!(r.into_rows().count(), 2);
    }

    #[test]
    fn view_ref_into_iter_yields_data_lifetime() {
        fn cells(toodee: &TooDee<u32>) -> impl Iterator<Item = &u32> {
            let view = toodee.view((2, 2), (4, 4));
            (&view).into_iter()
        }
        let toodee = TooDee::from_vec(10, 10, (0u32..100).collect());
        assert_eq!(cells(&toodee).copied().sum::<u32>(), 22+23+32+33);
    }

    #[test]
    fn view_mut_into_col_double_ended() {
        let mut toodee = TooDee::from_vec(4, 4, (0u32..16).collect());
        let mut col = toodee.view_mut((1, 1), (3, 4)).into_col(1);
        assert_eq!(col.len(), 3);
        assert_eq!(*col.next_back().unwrap(), 14);
        assert_eq!(*col.nth(1).unwrap(), 10);
        assert!(col.next().is_none());
        assert!(col.next_back().is_none());
    }

    #[test]
    fn view_mut_into_rows_double_ended() {
        let mut toodee = TooDee::from_vec(4, 4, (0u32..16).collect());
        let mut rows = toodee.view_mut((1, 1), (3, 4)).into_rows();
        assert_eq!(rows.len(), 3);
        assert_eq!(rows.next_back().unwrap(), &[13, 14]);
        assert_eq!(rows.next().unwrap(), &[5, 6]);
        assert_eq!(rows.next().unwrap(), &[9, 10]);
        assert!(rows.next().is_none());
    }

    #[test]
    fn view_mut_ops_match_into_methods() {
        let mut toodee = TooDee::from_vec(10, 10, (0u32..100).collect());
        let mut view = toodee.view_mut((2, 2), (5, 4));
        let rows: Vec<Vec<u32>> = view.rows().map(|r| r.to_vec()).collect();
        let col: Vec<u32> = view.col(1).copied().collect();
        assert_eq!(rows, vec![vec![22, 23, 24], vec![32, 33, 34]]);
        assert_eq!(col, vec![23, 33]);
        for r in view.rows_mut() {
            r[0] += 100;
        }
        view.col_mut(2).for_each(|c| *c += 200);
        assert_eq!(view.into_rows().map(|r| r.to_vec()).collect::<Vec<_>>(), vec![vec![122, 23, 224], vec![132, 33, 234]]);
    }

    #[test]
    fn zero_sized_type_into_methods() {
        let mut toodee = TooDee::init(5, 3, ());
        assert_eq!(toodee.view((1, 1), (4, 3)).into_cells().count(), 6);
        assert_eq!(toodee.view((1, 1), (4, 3)).into_col(2).count(), 2);
        assert_eq!(toodee.view_mut((1, 1), (4, 3)).into_rows().count(), 2);
        assert_eq!(toodee.view_mut((1, 0), (4, 3)).into_col(2).count(), 3);
        assert_eq!(toodee.view_mut((1, 0), (4, 3)).into_cells().count(), 9);
    }

    #[test]
    fn view_mut_into_rows_col_cells() {
        let mut toodee = TooDee::from_vec(10, 10, (0u32..100).collect());
        for r in toodee.view_mut((2, 2), (5, 4)).into_rows() {
            r[0] = 0;
        }
        for c in toodee.view_mut((2, 2), (5, 4)).into_col(2) {
            *c = 1;
        }
        assert_eq!(toodee.view((2, 2), (5, 4)).into_cells().copied().collect::<Vec<_>>(), vec![0, 23, 1, 0, 33, 1]);
        toodee.view_mut((2, 2), (5, 4)).into_cells().for_each(|c| *c += 100);
        assert_eq!(toodee.view((2, 2), (5, 4)).into_cells().copied().collect::<Vec<_>>(), vec![100, 123, 101, 100, 133, 101]);
        assert_eq!(toodee.cells().filter(|&&c| c >= 100).count(), 6);
    }

    #[test]
    fn empty_view_into_iterators() {
        let mut toodee = TooDee::init(10, 10, 0u32);
        assert_eq!(toodee.view((5, 5), (5, 5)).into_rows().next(), None);
        assert_eq!(toodee.view((5, 5), (5, 5)).into_cells().next(), None);
        assert_eq!(toodee.view_mut((5, 5), (5, 5)).into_rows().next(), None);
        assert_eq!(toodee.view_mut((5, 5), (5, 5)).into_cells().next(), None);
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


    #[test]
    fn empty_views_at_end_use_in_bounds_ranges() {
        let mut toodee = TooDee::from_vec(3, 2, (0u32..6).collect());

        let view = toodee.view((1, 2), (1, 2));
        assert_eq!(view.num_cols(), 0);
        assert_eq!(view.num_rows(), 0);

        let view = toodee.view_mut((3, 2), (3, 2));
        assert_eq!(view.num_cols(), 0);
        assert_eq!(view.num_rows(), 0);
        drop(view);

        let mut view = toodee.view_mut((0, 0), (2, 2));
        let nested = view.view((1, 2), (1, 2));
        assert_eq!(nested.num_cols(), 0);
        assert_eq!(nested.num_rows(), 0);
        let nested = view.view_mut((1, 2), (1, 2));
        assert_eq!(nested.num_cols(), 0);
        assert_eq!(nested.num_rows(), 0);
    }
}
