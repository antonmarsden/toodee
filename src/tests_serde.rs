#[cfg(test)]
mod toodee_tests_serde {

    use crate::*;
    
    fn new_5_by_10() -> TooDee<u32>
    {
        TooDee::from_vec(5, 10, (0u32..50).collect())
    }

    #[test]
    fn serde() {
        let tmp = new_5_by_10();
        let serialized = serde_json::to_string(&tmp).unwrap();
        println!("serialized = {}", serialized);
    }

}
