/// Pagination information
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Pagination {
    /// URL of next page (if any). This is sometimes the empty string.
    /// To prevent misuse, we add an accessor.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    next: Option<String>,
}
impl Pagination {
    pub fn next(&self) -> Option<&str> {
        match self.next {
            Some(ref s) if s.is_empty() => None,
            ref s => s.as_deref(),
        }
    }
}

pub(crate) mod base64 {
    use serde::{Deserialize, Serialize};
    use serde::{Deserializer, Serializer};

    use base64::Engine;

    pub fn serialize<S: Serializer>(v: &Vec<u8>, s: S) -> Result<S::Ok, S::Error> {
        let base64 = base64::engine::general_purpose::STANDARD.encode(v);
        String::serialize(&base64, s)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Vec<u8>, D::Error> {
        let base64 = String::deserialize(d)?;
        base64::engine::general_purpose::STANDARD
            .decode(base64.as_bytes())
            .map_err(serde::de::Error::custom)
    }
}

pub(crate) mod one_or_many {
    use std::marker::PhantomData;

    use serde::{de::Visitor, Deserialize, Deserializer};

    #[derive(Default)]
    struct OneOrManyVisitor<T>(PhantomData<T>);

    impl<'de, T> Visitor<'de> for OneOrManyVisitor<T>
    where
        T: Deserialize<'de>,
    {
        type Value = Vec<T>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("One, or several instances of a type")
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::SeqAccess<'de>,
        {
            let mut vec = Vec::new();
            while let Some(elem) = seq.next_element()? {
                vec.push(elem);
            }
            Ok(vec)
        }

        fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::MapAccess<'de>,
        {
            Ok(vec![T::deserialize(
                serde::de::value::MapAccessDeserializer::new(map),
            )?])
        }
    }

    pub fn deserialize<'de, D: Deserializer<'de>, T: Deserialize<'de>>(
        d: D,
    ) -> Result<Vec<T>, D::Error> {
        d.deserialize_any(OneOrManyVisitor(PhantomData))
    }
}
