use serde::de::{self, Unexpected, Deserialize, Deserializer, Visitor, MapAccess};
use crate::toodee::TooDee;
use core::fmt;
extern crate alloc;
use alloc::vec::Vec;

struct MyMapVisitor<T> where T: Default + Clone {
    num_cols: Option<usize>,
    num_rows: Option<usize>,
    data: Option<Vec<T>>
}

impl<T> MyMapVisitor<T> where T: Default + Clone {
    fn new() -> Self {
        MyMapVisitor {
            num_cols: None,
            num_rows: None,
            data: None
        }
    }
    fn data_length(&self) -> Option<usize> {
        if self.data.is_none() {
            return None;
        }
        Some(self.data.as_ref().unwrap().len())
    }
}
const FIELDS: &'static [&'static str] = &["num_cols", "num_rows", "data"];

impl<'de, T> Visitor<'de> for MyMapVisitor<T>
    where T: Deserialize<'de> + Default + Clone
{
    // The type that our Visitor is going to produce.
    type Value = TooDee<T>;

    // Format a message stating what data this Visitor expects to receive.
    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a very special map")
    }

    // Deserialize MyMap from an abstract "map" provided by the
    // Deserializer. The MapAccess input is a callback provided by
    // the Deserializer to let us see each entry in the map.
    fn visit_map<M>(mut self, mut visitor: M) -> Result<Self::Value, M::Error>
        where
            M: MapAccess<'de>,
    {
        // const map = TooDee::new(0, 0);

        // While there are entries remaining in the input, add them
        // into our map.
        while let Some(key) = visitor.next_key::<&str>()? {
            match key {
                "num_cols" => self.num_cols = Some(visitor.next_value::<usize>()?),
                "num_rows" => self.num_rows = Some(visitor.next_value::<usize>()?),
                "data" => self.data = Some(visitor.next_value::<Vec<T>>()?),
                &_ => return Err(de::Error::unknown_field(&key, FIELDS)),
            }
        }
        if self.num_cols.is_none() {
            return Err(de::Error::missing_field("num_cols"))
        }
        if self.num_rows.is_none() {
            return Err(de::Error::missing_field("num_rows"))
        }
        if self.data.is_none() {
            return Err(de::Error::missing_field("data"))
        }
        let (product, overflow) = self.num_cols.unwrap().overflowing_mul(self.num_rows.unwrap());
        if overflow {
            return Err(de::Error::invalid_value(Unexpected::Unsigned(product.try_into().unwrap()),&"overflow"))
        }
        if product != self.data_length().unwrap() {
            return Err(de::Error::invalid_length(product, &"dimensions to match array length"))
        }
        Ok(TooDee::from_vec(self.num_cols.unwrap(), self.num_rows.unwrap(), self.data.unwrap()))
    }
}

impl<'de, T> Deserialize<'de> for TooDee<T>
    where
        T: Deserialize<'de> + Default + Clone
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>
    {
        // Instantiate our Visitor and ask the Deserializer to drive
        // it over the input data, resulting in an instance of MyMap.
        deserializer.deserialize_map(MyMapVisitor::new())
    }
}