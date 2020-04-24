#[cfg(test)]
mod toodee_tests_slide {
    
    extern crate alloc;

    use crate::*;
    
    #[test]
    fn slide_with_wrap() {
        let mut toodee = TooDee::new(10, 10, 0u32);
        for i in 0..100 {
            toodee.data_mut()[i] = i as u32;
        }
        toodee.slide_with_wrap(-13, 12);
        let expected = (100 * 100 - 100) / 2;
        assert_eq!(toodee.data.iter().sum::<u32>(), expected);
        assert_eq!(toodee[0][0], 83);
        assert_eq!(toodee[0][9], 82);
        assert_eq!(toodee[9][0], 73);
        assert_eq!(toodee[9][9], 72);
//        println!("{:?}", toodee);
    }

    #[test]
    fn slide_with_fill() {
        let mut toodee = TooDee::new(10, 10, 0u32);
        for i in 0..100 {
            toodee.data_mut()[i] = i as u32;
        }
        toodee.slide_with_fill(1, 2, &0);
//        println!("{:?}", toodee);
        assert_eq!(toodee[0][0], 0);
        assert_eq!(toodee[1][9], 0);
        assert_eq!(toodee[9][0], 0);
        assert_eq!(toodee[9][1], 70);
        assert_eq!(toodee[9][9], 78);
    }


    #[test]
    fn view_slide_with_wrap() {
        let mut toodee = TooDee::new(10, 10, 0u32);
        for i in 0..100 {
            toodee.data_mut()[i] = i as u32;
        }
        toodee.view_mut(0, 0, 10, 10).slide_with_wrap(-13, 12);
        let expected = (100 * 100 - 100) / 2;
//        println!("{:?}", toodee);
        assert_eq!(toodee.data.iter().sum::<u32>(), expected);
        assert_eq!(toodee[0][0], 83);
        assert_eq!(toodee[0][9], 82);
        assert_eq!(toodee[9][0], 73);
        assert_eq!(toodee[9][9], 72);
    }

    #[test]
    fn view_slide_with_fill() {
        let mut toodee = TooDee::new(10, 10, 0u32);
        for i in 0..100 {
            toodee.data_mut()[i] = i as u32;
        }
        toodee.view_mut(0, 0, 10, 10).slide_with_fill(1, 2, &0);
//        println!("{:?}", toodee);
        assert_eq!(toodee[0][0], 0);
        assert_eq!(toodee[1][9], 0);
        assert_eq!(toodee[9][0], 0);
        assert_eq!(toodee[9][1], 70);
        assert_eq!(toodee[9][9], 78);
    }

    #[test]
    fn view_slide_with_wrap_zero() {
        let mut toodee = TooDee::new(10, 10, 0u32);
        for i in 0..100 {
            toodee.data_mut()[i] = i as u32;
        }
        toodee.view_mut(0, 0, 10, 10).slide_with_wrap(0, 0);
        assert_eq!(toodee[0][0], 0);
        assert_eq!(toodee[0][9], 9);
        assert_eq!(toodee[9][0], 90);
        assert_eq!(toodee[9][9], 99);
    }

    #[test]
    fn view_slide_with_wrap_col_only() {
        let mut toodee = TooDee::new(10, 10, 0u32);
        for i in 0..100 {
            toodee.data_mut()[i] = i as u32;
        }
        toodee.view_mut(0, 0, 10, 10).slide_with_wrap(1, 0);
        assert_eq!(toodee[0][0], 9);
        assert_eq!(toodee[0][9], 8);
        assert_eq!(toodee[9][0], 99);
        assert_eq!(toodee[9][9], 98);
        toodee.view_mut(0, 0, 10, 10).slide_with_wrap(-2, 0);
        assert_eq!(toodee[0][0], 1);
        assert_eq!(toodee[0][9], 0);
        assert_eq!(toodee[9][0], 91);
        assert_eq!(toodee[9][9], 90);
    }

    #[test]
    fn view_slide_with_wrap_row_only() {
        let mut toodee = TooDee::new(10, 10, 0u32);
        for i in 0..100 {
            toodee.data_mut()[i] = i as u32;
        }
        toodee.view_mut(0, 0, 10, 10).slide_with_wrap(0, 1);
        assert_eq!(toodee[0][0], 90);
        assert_eq!(toodee[0][9], 99);
        assert_eq!(toodee[9][0], 80);
        assert_eq!(toodee[9][9], 89);
        toodee.view_mut(0, 0, 10, 10).slide_with_wrap(0, -2);
        assert_eq!(toodee[0][0], 10);
        assert_eq!(toodee[0][9], 19);
        assert_eq!(toodee[9][0], 0);
        assert_eq!(toodee[9][9], 9);
    }

}
