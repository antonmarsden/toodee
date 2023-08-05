#[cfg(test)]
mod toodee_tests_serde {
    use crate::*;
    use serde_json;
    fn new_5_by_10() -> TooDee<u32>
    {
        TooDee::from_vec(5, 10, (0u32..50).collect())
    }

    #[test]
    fn serialize() {
        let tmp = new_5_by_10();
        let serialized: String = serde_json::to_string(&tmp).unwrap();
        assert!(serialized.contains("data"));
    }

    #[test]
    fn deserialize() {
        let tmp = new_5_by_10();
        let serialized = serde_json::to_string(&tmp).unwrap();
        let deser: TooDee<u32> = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deser.num_cols(), 5);
        assert_eq!(deser.num_rows(), 10);
        assert_eq!(deser.data().len(), 50);
    }
}
