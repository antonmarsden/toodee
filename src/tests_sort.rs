#[cfg(test)]
mod toodee_tests_sort {
    
    use crate::*;
    use rand::Rng;
    use rand::SeedableRng;
    use rand::rngs::StdRng;
    use rand::distributions::Uniform;
    
    #[test]
    fn sort_by_row() {
        let rng = StdRng::seed_from_u64(42);
        let samples = rng.sample_iter(Uniform::new(0,100));
        let mut toodee = TooDee::from_vec(10, 10, samples.take(100).collect());
        println!("{:?}",&toodee[3]);
        toodee.sort_by_row(3, |a, b| a.cmp(b));
        let r = &toodee[3];
        println!("{:?}",r);
        for i in 0..9 {
            assert!(r[i] <= r[i+1]);
        }
    }
        
    #[test]
    fn sort_ord_helpers() {
        let mut toodee = TooDee::from_vec(3, 3, vec![3u32, 1, 2, 6, 4, 5, 9, 7, 8]);
        toodee.sort_row_ord(0);
        assert_eq!(&toodee[0], &[1, 2, 3]);
        toodee.sort_unstable_row_ord(1);
        assert_eq!(&toodee[1], &[4, 5, 6]);
        toodee.sort_col_ord(0);
        assert_eq!(toodee.col(0).copied().collect::<Vec<u32>>(), vec![1, 4, 7]);
    }


    #[test]
    fn sort_by_col() {
        let rng = StdRng::seed_from_u64(42);
        let samples = rng.sample_iter(Uniform::new(0,100));
        let mut toodee = TooDee::from_vec(10, 10, samples.take(100).collect());
        println!("{:?}", toodee.col(3).copied().collect::<Vec<u32>>());
        toodee.sort_by_col(3, |a, b| a.cmp(b));
        let c : Vec<u32> = toodee.col(3).copied().collect();
        println!("{:?}",c);
        for i in 0..9 {
            assert!(c[i] <= c[i+1]);
        }
    }
    
    #[test]
    fn sort_by_row_view() {
        let rng = StdRng::seed_from_u64(42);
        let samples = rng.sample_iter(Uniform::new(0,100));
        let mut toodee = TooDee::from_vec(10, 10, samples.take(100).collect());
        println!("{:?}",&toodee[3]);
        toodee.view_mut((0, 0), (10, 10)).sort_by_row(3, |a, b| a.cmp(b));
        let r = &toodee[3];
        println!("{:?}",r);
        for i in 0..9 {
            assert!(r[i] <= r[i+1]);
        }
    }

    #[test]
    fn sort_by_col_view() {
        let rng = StdRng::seed_from_u64(42);
        let samples = rng.sample_iter(Uniform::new(0,100));
        let mut toodee = TooDee::from_vec(10, 10, samples.take(100).collect());
        println!("{:?}", toodee.col(3).copied().collect::<Vec<u32>>());
        toodee.view_mut((0, 0), (10, 10)).sort_by_col(3, |a, b| a.cmp(b));
        let c : Vec<u32> = toodee.col(3).copied().collect();
        println!("{:?}",c);
        for i in 0..9 {
            assert!(c[i] <= c[i+1]);
        }
    }

    // Regression tests for issue #55: the *_col_key methods sorted along the wrong axis.

    #[test]
    fn sort_by_col_key_wide_grid() {
        // More columns than rows: column 2 is valid but would be out of range as a row index.
        let mut toodee = TooDee::from_vec(3, 2, vec![10, 20, 3, 40, 50, 1]);
        toodee.sort_by_col_key(2, |v| *v);
        assert_eq!(toodee.data(), &[40, 50, 1, 10, 20, 3]);
    }

    #[test]
    fn sort_unstable_by_col_key_wide_grid() {
        let mut toodee = TooDee::from_vec(3, 2, vec![10, 20, 3, 40, 50, 1]);
        toodee.sort_unstable_by_col_key(2, |v| *v);
        assert_eq!(toodee.data(), &[40, 50, 1, 10, 20, 3]);
    }

    #[test]
    fn sort_by_col_key_square_grid() {
        // Square grid: the old code silently sorted row 0 instead of reordering rows by column 0.
        let mut toodee = TooDee::from_vec(2, 2, vec![3, 2, 1, 4]);
        toodee.sort_by_col_key(0, |v| *v);
        assert_eq!(toodee.data(), &[1, 4, 3, 2]);

        let mut toodee = TooDee::from_vec(2, 2, vec![3, 2, 1, 4]);
        toodee.sort_unstable_by_col_key(0, |v| *v);
        assert_eq!(toodee.data(), &[1, 4, 3, 2]);
    }

    #[test]
    fn sort_by_col_key_uses_key_and_is_stable() {
        // Sort descending via the key; rows with equal keys in column 1 keep their original order.
        let mut toodee = TooDee::from_vec(2, 4, vec![0, 5, 1, 7, 2, 5, 3, 9]);
        toodee.sort_by_col_key(1, |v| core::cmp::Reverse(*v));
        assert_eq!(toodee.data(), &[3, 9, 1, 7, 0, 5, 2, 5]);
    }

    #[test]
    fn sort_by_col_key_view() {
        let mut toodee = TooDee::from_vec(3, 3, vec![1, 9, 0, 2, 8, 0, 3, 7, 0]);
        toodee.view_mut((0, 0), (2, 3)).sort_unstable_by_col_key(1, |v| *v);
        assert_eq!(toodee.data(), &[3, 7, 0, 2, 8, 0, 1, 9, 0]);
    }

}
