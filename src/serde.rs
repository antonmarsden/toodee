use serde::de::{self, Unexpected, Deserialize, Deserializer, Visitor, MapAccess};
use serde::{Serializer,Serialize};
use crate::toodee::TooDee;
use crate::view::{TooDeeView,TooDeeViewMut};
use core::fmt;
extern crate alloc;
use alloc::vec::Vec;
use core::marker::PhantomData;
use serde::ser::SerializeStruct;
use crate::TooDeeOps;

struct TooDeeVisitor<T> {
    marker: PhantomData<fn() -> TooDee<T>>
}

impl<T> TooDeeVisitor<T> {
    fn new() -> Self {
        TooDeeVisitor {
            marker: PhantomData
        }
    }
}
const FIELDS: &[&str] = &["num_cols", "num_rows", "data"];

impl<'de, T> Visitor<'de> for TooDeeVisitor<T>
    where T: Deserialize<'de>
{
    type Value = TooDee<T>;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a TooDee array (num_cols, num_rows, data)")
    }

    fn visit_map<M>(self, mut visitor: M) -> Result<Self::Value, M::Error>
        where
            M: MapAccess<'de>,
    {
        let mut num_cols = None;
        let mut num_rows = None;
        let mut data = None;
        while let Some(key) = visitor.next_key::<&str>()? {
            match key {
                "num_cols" => {
                    if num_cols.is_some() {
                        return Err(de::Error::duplicate_field("num_cols"));
                    }
                    num_cols = Some(visitor.next_value::<usize>()?)
                },
                "num_rows" => {
                    if num_rows.is_some() {
                        return Err(de::Error::duplicate_field("num_rows"));
                    }
                    num_rows = Some(visitor.next_value::<usize>()?)
                },
                "data" => {
                    data = Some(visitor.next_value::<Vec<T>>()?)
                },
                &_ => return Err(de::Error::unknown_field(key, FIELDS)),
            }
        }
        let num_cols = num_cols.ok_or_else(|| de::Error::missing_field("num_cols"))?;
        let num_rows = num_rows.ok_or_else(|| de::Error::missing_field("num_rows"))?;
        let data = data.ok_or_else(|| de::Error::missing_field("data"))?;
        let (product, overflow) = num_cols.overflowing_mul(num_rows);
        if overflow {
            return Err(de::Error::invalid_value(Unexpected::Other("product"),&"dimensions too big"))
        }
        if product != data.len() {
            return Err(de::Error::invalid_length(product, &"dimensions to match array length"))
        }
        Ok(TooDee::from_vec(num_cols, num_rows, data))
    }
}

impl<'de, T> Deserialize<'de> for TooDee<T>
    where
        T: Deserialize<'de>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>
    {
        deserializer.deserialize_map(TooDeeVisitor::new())
    }
}

impl Serialize for TooDeeView<'_, u32>
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

impl Serialize for TooDeeViewMut<'_, u32>
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