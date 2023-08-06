#[cfg(test)]
mod toodee_tests_transpose {

    use crate::*;

    fn new_2_by_4() -> TooDee<u32>
    {
        TooDee::from_vec(2, 4, (0u32..8).collect())
    }

    #[test]
    fn transpose() {
        let mut toodee = new_2_by_4();
        toodee.transpose();
        assert_eq!(toodee.num_cols(), 4);
        assert_eq!(toodee.num_rows(), 2);
        assert_eq!(toodee.data(), &[0, 2, 4, 6, 1, 3, 5, 7])
    }
}
