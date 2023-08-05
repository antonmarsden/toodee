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

    const JSON_OK: &str = r#"
{
  "num_rows": 2,
  "num_cols": 3,
  "data": [1, 2, 3, 4, 5, 6]
}
"#;

    const JSON_BAD_ARRAY: &str = r#"
{
  "num_rows": 2,
  "num_cols": 3,
  "data": [1, 2, 3, 4]
}
"#;

    #[test]
    fn serde() {
        let tmp = new_5_by_10();
        let serialized = serde_json::to_string(&tmp).unwrap();
        let deser: TooDee<u32> = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deser.num_cols(), 5);
        assert_eq!(deser.num_rows(), 10);
        assert_eq!(deser.data().len(), 50);
    }

    #[test]
    fn deserialize() {
        let deser: TooDee<u32> = serde_json::from_str(&JSON_OK).unwrap();
        assert_eq!(deser.num_cols(), 3);
        assert_eq!(deser.num_rows(), 2);
        assert_eq!(deser.data().len(), 6);
    }

    #[test]
    #[should_panic(expected = "invalid length 6, expected dimensions to match array length")]
    fn deserialize_bad_array() {
        let deser: TooDee<u32> = serde_json::from_str(&JSON_BAD_ARRAY).unwrap();
        assert_eq!(deser.num_cols(), 3);
        assert_eq!(deser.num_rows(), 2);
        assert_eq!(deser.data().len(), 6);
    }
}
