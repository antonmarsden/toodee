#[cfg(test)]
mod toodee_tests {
    
    extern crate alloc;
    use alloc::boxed::Box;
    use alloc::vec;
    use alloc::vec::Vec;

    use crate::*;

    #[test]
    fn new() {
        let toodee = TooDee::new(200, 150, 0u32);
        assert_eq!(toodee.data().len(), 200 * 150);
        assert_eq!((200, 150), toodee.size());
        assert_eq!(toodee.num_rows(), 150);
        assert_eq!(toodee.num_cols(), 200);
    }

    #[test]
    fn new_view() {
        let toodee = TooDee::new(200, 150, 0u32);
        let view = toodee.view((50, 50), (150, 100));
        assert_eq!((100, 50), view.size());
        assert_eq!(view.num_rows(), 50);
        assert_eq!(view.num_cols(), 100);
        assert_eq!(view.bounds(), ((50, 50), (150, 100)));
    }

    #[test]
    fn new_view_direct() {
        let v = vec![1u32; 32];
        let view = TooDeeView::new(4, 8, &v);
        assert_eq!((4, 8), view.size());
        assert_eq!(view.num_cols(), 4);
        assert_eq!(view.num_rows(), 8);
        assert_eq!(view.bounds(), ((0, 0), (4, 8)));
        assert_eq!(view.cells().sum::<u32>(), 32);
    }

    #[test]
    fn new_view_mut_direct() {
        let mut v = vec![1u32; 32];
        let view = TooDeeViewMut::new(8, 4, &mut v);
        assert_eq!((8, 4), view.size());
        assert_eq!(view.num_cols(), 8);
        assert_eq!(view.num_rows(), 4);
        assert_eq!(view.bounds(), ((0, 0), (8, 4)));
        assert_eq!(view.cells().sum::<u32>(), 32);
    }

    #[test]
    fn into_vec() {
        let toodee = TooDee::new(10, 10, 22u32);
        let v: Vec<u32> = toodee.into();
        assert_eq!(v.iter().sum::<u32>(), 2200);
    }

    #[test]
    fn fill() {
        let mut toodee = TooDee::new(10, 10, 22u32);
        assert_eq!(toodee.data().iter().sum::<u32>(), 2200);
        toodee.view_mut((0, 0), (10, 10)).fill(11);
        assert_eq!(toodee.data().iter().sum::<u32>(), 1100);
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
    fn from_vec() {
        let v = vec![42u32; 16];
        let toodee = TooDee::from_vec(8, 2, v);
        assert_eq!(42, toodee[1][7]);
        assert_eq!(42, toodee[1][3]);
        assert_eq!((8, 2), toodee.size());
    }

    #[test]
    fn from_box() {
        let v = vec![42u32; 16];
        let toodee = TooDee::from_box(8, 2, Box::from(v));
        assert_eq!(42, toodee[1][7]);
        assert_eq!(42, toodee[1][3]);
        assert_eq!((8, 2), toodee.size());
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn from_vec_bad_size() {
        let v = vec![42u32; 16];
        TooDee::from_vec(8, 3, v);
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn from_vec_bad_size_2() {
        let v = vec![42u32; 16];
        TooDee::from_vec(8, 1, v);
    }

    #[test]
    fn index() {
        let mut toodee = TooDee::new(4, 3, 0u32);
        toodee[0].copy_from_slice(&vec![1u32; 4][..4]);
        toodee[1].copy_from_slice(&vec![2u32; 4][..4]);
        toodee[2].copy_from_slice(&vec![3u32; 4][..4]);
        assert_eq!(toodee.data().iter().sum::<u32>(), 24);
        assert_eq!(toodee[1].iter().sum::<u32>(), 8);
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn index_bad() {
        let mut toodee = TooDee::new(4, 3, 0u32);
        toodee[5][5] = 1;
    }

    #[test]
    #[should_panic(expected = "index out of bounds: the len is 4 but the index is 5")]
    fn index_bad_2() {
        let mut toodee = TooDee::new(4, 3, 0u32);
        toodee[2][5] = 1;
    }

    #[test]
    fn set_value() {
        let mut toodee = TooDee::new(3, 3, 0u32);
        toodee[1][1] = 1;
        toodee[2][2] = 2;
        assert_eq!(toodee.data().iter().sum::<u32>(), 3);
    }

    #[test]
    fn swap_cols() {
        let mut toodee = TooDee::from_vec(10, 10, (0u32..100).collect());
        toodee.swap_cols(1,2);
        assert_eq!(toodee[0][1], 2);
        assert_eq!(toodee[0][2], 1);
        assert_eq!(toodee[9][1], 92);
        assert_eq!(toodee[9][2], 91);
        toodee.swap_cols(9,5);
        assert_eq!(toodee[0][5], 9);
        assert_eq!(toodee[0][9], 5);
        assert_eq!(toodee[9][5], 99);
        assert_eq!(toodee[9][9], 95);
        println!("{:?}", toodee);
        toodee.swap_cols(6,6);
        assert_eq!(toodee[6][0], 60);
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn swap_rows_out_of_bounds() {
        let mut toodee = TooDee::new(10, 10, 0u32);
        toodee.swap_rows(0,10);
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn swap_cols_out_of_bounds() {
        let mut toodee = TooDee::new(10, 10, 0u32);
        toodee.swap_cols(0,10);
    }

    #[test]
    fn view() {
        let toodee = TooDee::from_vec(10, 10, (0u32..100).collect());

        let expected = (100 * 100 - 100) / 2;
        assert_eq!(toodee.data().iter().sum::<u32>(), expected);

        let view = toodee.view((4, 6), (6, 10));
        assert_eq!(2, view.num_cols());
        assert_eq!(4, view.num_rows());
        let mut count = 0u32;
        for r in 0..view.num_rows() {
            for c in 0..view.num_cols() {
                count += view[r][c];
            }
        }
        assert_eq!(64+65+74+75+84+85+94+95, count);
    }

    #[test]
    #[should_panic(expected = "index out of bounds: the len is 0 but the index is 0")]
    fn view_empty() {
        let toodee = TooDee::new(10, 10, 42u32);
        let view = toodee.view((0, 0), (0, 10));
        let tmp = view[0][0];
        assert_eq!(tmp, 42);
    }

    #[test]
    fn view_mut() {
        let mut toodee = TooDee::from_vec(10, 10, (0u32..100).collect());
        let expected = (100 * 100 - 100) / 2;
        assert_eq!(toodee.data().iter().sum::<u32>(), expected);

        let mut view = toodee.view_mut((4, 6), (6, 10));
        assert_eq!(2, view.num_cols());
        assert_eq!(4, view.num_rows());
        for r in 0..view.num_rows() {
            for c in 0..view.num_cols() {
                view[r][c] = (r * view.num_cols() + c) as u32;
            }
        }
        assert_eq!(
            toodee.data().iter().sum::<u32>(),
            4950 - (64+65+74+75+84+85+94+95) + (1 + 2 + 3 + 4 + 5 + 6 + 7)
        );
    }

    #[test]
    fn copy_from_view() {
        let mut toodee = TooDee::new(10, 10, 0u32);
        let tile = TooDee::new(3, 3, 1u32);
        let tile_view = tile.view((0, 0), (3, 3));
        toodee.view_mut((0, 0), (3, 3)).copy_from_toodee(&tile_view);
        toodee.view_mut((6, 6), (9, 9)).copy_from_toodee(&tile_view);
        assert_eq!(toodee.data().iter().sum::<u32>(), 18);
    }
    
    #[test]
    fn zero_size_toodee() {
        let mut toodee = TooDee::new(0, 0, 0u32);
        assert_eq!(toodee.rows_mut().next(), None);
        assert_eq!(toodee.rows().next(), None);
        assert_eq!(toodee.cells().next(), None);
        assert_eq!(toodee.cells_mut().next(), None);
    }

    #[test]
    fn zero_size_view() {
        let mut toodee = TooDee::new(10, 10, 0u32);
        let mut view = toodee.view_mut((5, 5), (5, 5));
        assert_eq!(view.rows_mut().next(), None);
        assert_eq!(view.rows().next(), None);
        assert_eq!(view.cells().next(), None);
        assert_eq!(view.cells_mut().next(), None);
        view = toodee.view_mut((5, 5), (6, 5));
        assert_eq!(view.rows_mut().next(), None);
        assert_eq!(view.rows().next(), None);
        assert_eq!(view.cells().next(), None);
        assert_eq!(view.cells_mut().next(), None);
        assert_eq!(None, view.col(0).next());
        view = toodee.view_mut((5, 5), (5, 6));
        assert_eq!(view.rows_mut().next(), None);
        assert_eq!(view.rows().next(), None);
        assert_eq!(view.cells().next(), None);
        assert_eq!(view.cells_mut().next(), None);
        assert_eq!(view[0].iter().next(), None);
    }
    
    #[test]
    fn zero_size_view_of_zero() {
        let mut toodee = TooDee::new(0, 0, 0u32);
        let mut view = toodee.view_mut((0, 0), (0, 0));
        assert_eq!(view.rows_mut().next(), None);
        assert_eq!(view.rows().next(), None);
        assert_eq!(view.cells().next(), None);
        assert_eq!(view.cells_mut().next(), None);
    }
    
    #[test]
    #[should_panic(expected = "assertion failed")]
    fn zero_size_view_col() {
        let toodee = TooDee::new(0, 0, 0u32);
        let view = toodee.view((0, 0), (0, 0));
        view.col(0);
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn zero_size_view_mut_col_mut() {
        let mut toodee = TooDee::new(0, 0, 0u32);
        let mut view = toodee.view_mut((0, 0), (0, 0));
        view.col_mut(0);
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn zero_size_view_mut_col() {
        let mut toodee = TooDee::new(0, 0, 0u32);
        let view = toodee.view_mut((0, 0), (0, 0));
        view.col(0);
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn zero_size_col() {
        let toodee = TooDee::new(0, 0, 0u32);
        toodee.col(0);
    }
    
    #[test]
    #[should_panic(expected = "assertion failed")]
    fn zero_size_col_mut() {
        let mut toodee = TooDee::new(0, 0, 0u32);
        toodee.col_mut(0);
    }
}
