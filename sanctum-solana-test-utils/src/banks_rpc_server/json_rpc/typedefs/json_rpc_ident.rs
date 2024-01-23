use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};

const VERSION_2_0: &str = "2.0";

#[derive(Clone, Copy, Debug, Default)]
pub struct JsonRpc2Ident;

impl Serialize for JsonRpc2Ident {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(VERSION_2_0)
    }
}

struct JsonRpc2IdentVisitor;

impl Visitor<'_> for JsonRpc2IdentVisitor {
    type Value = JsonRpc2Ident;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str(VERSION_2_0)
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if v == VERSION_2_0 {
            Ok(JsonRpc2Ident)
        } else {
            Err(de::Error::invalid_type(de::Unexpected::Str(v), &Self))
        }
    }
}

impl<'de> Deserialize<'de> for JsonRpc2Ident {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(JsonRpc2IdentVisitor)
    }
}
