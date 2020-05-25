#[cfg(test)]
mod toodee_tests_iter {
    
    use crate::*;

    #[test]
    fn rows_iter() {
        let toodee = TooDee::init(10, 10, 22u32);
        assert_eq!(toodee.rows().len(), 10);
        assert_eq!(toodee.rows().fold(0, |count, r| count + r.len()), 10 * 10);
    }

    #[test]
    fn rows_mut_iter() {
        let mut toodee = TooDee::init(10, 10, 22u32);
        assert_eq!(toodee.rows_mut().len(), 10);
        assert_eq!(toodee.rows_mut().fold(0, |count, r| count + r.len()), 10 * 10);
    }

    #[test]
    fn view_rows_iter() {
        let toodee = TooDee::init(10, 10, 22u32);
        let v = toodee.view((2, 2), (10, 10));
        assert_eq!(v.rows().len(), 8);
        assert_eq!(v.rows().fold(0, |count, r| count + r.len()), 8 * 8);
    }

    #[test]
    fn view_rows_iter_rev() {
        let toodee = TooDee::init(10, 10, 22u32);
        let v = toodee.view((2, 2), (10, 10));
        assert_eq!(v.rows().rev().len(), 8);
        assert_eq!(v.rows().rev().fold(0, |count, r| count + r.len()), 8 * 8);
    }
    
    #[test]
    fn view_rows_iter_mut() {
        let mut toodee = TooDee::init(10, 10, 22u32);
        let mut v = toodee.view_mut((2, 2), (10, 10));
        assert_eq!(v.rows().len(), 8);
        assert_eq!(v.rows_mut().len(), 8);
        assert_eq!(v.rows().fold(0, |count, r| count + r.len()), 8 * 8);
        assert_eq!(v.rows_mut().fold(0, |count, r| count + r.len()), 8 * 8);
    }

    #[test]
    fn view_rows_iter_mut_rev() {
        let mut toodee = TooDee::init(10, 10, 22u32);
        let mut v = toodee.view_mut((2, 2), (10, 10));
        assert_eq!(v.rows().rev().len(), 8);
        assert_eq!(v.rows_mut().rev().len(), 8);
        assert_eq!(v.rows().rev().fold(0, |count, r| count + r.len()), 8 * 8);
        assert_eq!(v.rows_mut().rev().fold(0, |count, r| count + r.len()), 8 * 8);
    }

    #[test]
    fn col_iter() {
        let toodee = TooDee::from_vec(10, 10, (0u32..100).collect());
        let mut col = toodee.col(2);
        assert_eq!(col.len(), 10);
        assert_eq!(col.next().unwrap(), &2);
        let expected_sum = 2+12+22+32+42+52+62+72+82+92;
        assert_eq!(col.copied().sum::<u32>(), expected_sum-2);
        let mut rev  = toodee.col(2).rev();
        assert_eq!(rev.len(), 10);
        assert_eq!(rev.next().unwrap(), &92);
        assert_eq!(rev.copied().sum::<u32>(), expected_sum-92);
    }

    #[test]
    fn col_mut_iter() {
        let mut toodee = TooDee::from_vec(10, 10, (0u32..100).collect());
        let mut col = toodee.col_mut(2);
        assert_eq!(col.len(), 10);
        assert_eq!(col.next().unwrap(), &2);
        let expected_sum = 2+12+22+32+42+52+62+72+82+92;
        assert_eq!(col.map(|v| *v).sum::<u32>(), expected_sum-2);
        let mut rev  = toodee.col_mut(2).rev();
        assert_eq!(rev.len(), 10);
        assert_eq!(rev.next().unwrap(), &92);
        assert_eq!(rev.map(|v| *v).sum::<u32>(), expected_sum-92);
    }

    #[test]
    fn view_col_iter() {
        let toodee = TooDee::from_vec(10, 10, (0u32..100).collect());
        let view = toodee.view((2, 2), (8, 8));
        let mut col = view.col(2);
        assert_eq!(col.len(), 6);
        assert_eq!(col.next().unwrap(), &24);
        let expected_sum = 24+34+44+54+64+74;
        assert_eq!(col.copied().sum::<u32>(), expected_sum-24);
        let mut rev  = view.col(2).rev();
        assert_eq!(rev.len(), 6);
        assert_eq!(rev.next().unwrap(), &74);
        assert_eq!(rev.copied().sum::<u32>(), expected_sum-74);
    }

    #[test]
    fn view_col_mut_iter() {
        let mut toodee = TooDee::from_vec(10, 10, (0u32..100).collect());
        let mut view = toodee.view_mut((2, 2), (8, 8));
        let mut col = view.col_mut(2);
        assert_eq!(col.len(), 6);
        assert_eq!(col.next().unwrap(), &24);
        let expected_sum = 24+34+44+54+64+74;
        assert_eq!(col.map(|v| *v).sum::<u32>(), expected_sum-24);
        let mut rev  = view.col_mut(2).rev();
        assert_eq!(rev.len(), 6);
        assert_eq!(rev.next().unwrap(), &74);
        assert_eq!(rev.map(|v| *v).sum::<u32>(), expected_sum-74);
    }

    #[test]
    #[allow(clippy::iter_nth_zero)]
    fn cells() {
        let toodee = TooDee::from_vec(10, 10, (0u32..100).collect());
        let mut cells = toodee.cells();
        assert_eq!(cells.next(), Some(&0u32));
        assert_eq!(cells.next(), Some(&1u32));
        assert_eq!(cells.size_hint(), (98, Some(98)));
        assert_eq!(cells.next_back(), Some(&99u32));
        assert_eq!(cells.next_back(), Some(&98u32));
        assert_eq!(cells.size_hint(), (96, Some(96)));
        // tests nth() in FlattenExact
        assert_eq!(cells.nth(18), Some(&20u32));
        assert_eq!(cells.nth(8),  Some(&29u32));
        assert_eq!(cells.nth(63), Some(&93u32));
        assert_eq!(cells.nth(1), Some(&95u32));
        assert_eq!(cells.nth(0), Some(&96u32));
        assert_eq!(cells.nth(0), Some(&97u32));
        assert_eq!(cells.nth(0), None);
    }
    
    #[test]
    fn cells_mut() {
        let mut toodee = TooDee::from_vec(10, 10, (0u32..100).collect());
        let mut cells = toodee.cells_mut();
        assert_eq!(cells.next(), Some(&mut 0u32));
        assert_eq!(cells.next(), Some(&mut 1u32));
        assert_eq!(cells.size_hint(), (98, Some(98)));
        assert_eq!(cells.next_back(), Some(&mut 99u32));
        assert_eq!(cells.next_back(), Some(&mut 98u32));
        assert_eq!(cells.size_hint(), (96, Some(96)));
    }
}
