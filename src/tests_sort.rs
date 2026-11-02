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

    
}
