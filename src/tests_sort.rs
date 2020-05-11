#[cfg(test)]
mod toodee_tests_sort {
    
    use crate::*;
    use rand::Rng;
    
    #[test]
    fn sort_by_row() {
        let mut toodee = TooDee::new(10, 10, 0u32);
        let mut rng = rand::thread_rng();
        for i in 0..10 {
            for j in 0..10 {
                toodee[i][j] = rng.gen_range(0, 100);
            }
        }
        println!("{:?}",&toodee[3]);
        toodee.sort_by_row(3, |a, b| a.cmp(b));
        let r = &toodee[3];
        println!("{:?}",r);
        for i in 0..9 {
            assert!(r[i] <= r[i+1]);
        }
    }
        
    #[test]
    fn sort_by_col() {
        let mut toodee = TooDee::new(10, 10, 0u32);
        let mut rng = rand::thread_rng();
        for i in 0..10 {
            for j in 0..10 {
                toodee[i][j] = rng.gen_range(0, 100);
            }
        }
        println!("{:?}", toodee.col(3).copied().collect::<Vec<u32>>());
        toodee.sort_by_col(3, |a, b| a.cmp(b));
        let c : Vec<u32> = toodee.col(3).copied().collect();
        println!("{:?}",c);
        for i in 0..9 {
            assert!(c[i] <= c[i+1]);
        }
    }
}
