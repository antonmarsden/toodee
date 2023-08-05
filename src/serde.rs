use crate::toodee::*;
use crate::ops::*;
extern crate alloc;
use alloc::vec::Vec;
use serde::ser::{Serialize, SerializeStruct, Serializer};

impl Serialize for TooDee<u32>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut storage = serializer.serialize_struct("TooDee", 3)?;
        storage.serialize_field("num_cols", &self.num_cols())?;
        storage.serialize_field("num_rows", &self.num_rows())?;
        storage.serialize_field("data", &self.cells().collect::<Vec<_>>())?;
        storage.end()
    }
}

// impl<T> Deserialize for dyn TooDeeOps<T> where T: Deserialize
// {
//
// }
//
// impl<'de> Deserialize<'de> for i32 {
// fn deserialize<D>(deserializer: D) -> Result<i32, D::Error>
//     where
//         D: Deserializer<'de>,
// {
//     deserializer.deserialize_i32(I32Visitor)
// }
// }