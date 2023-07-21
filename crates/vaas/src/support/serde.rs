#![cfg(feature = "serde")]
use serde::{de, de::SeqAccess, Deserializer, Serialize, Serializer};

pub(crate) mod fixed_bytes_as_array {
    use super::*;
    use alloy_primitives::FixedBytes;

    pub fn serialize<S, const N: usize>(
        bytes: &FixedBytes<N>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        bytes.to_vec().serialize(serializer)
    }

    pub fn deserialize<'de, D, const N: usize>(deserializer: D) -> Result<FixedBytes<N>, D::Error>
    where
        D: Deserializer<'de>,
    {
        pub struct FbVisitor<const N: usize>;

        impl<'de, const N: usize> de::Visitor<'de> for FbVisitor<N> {
            type Value = FixedBytes<N>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str(
                    "a fixed length byte array represented as an array of numbers, or as hex",
                )
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut bytes = [0u8; N];
                for (i, b) in bytes.iter_mut().enumerate() {
                    *b = seq
                        .next_element()?
                        .ok_or_else(|| de::Error::invalid_length(i, &self))?;
                }

                if let Ok(elem) = seq.next_element::<u8>() {
                    if elem.is_some() {
                        return Err(de::Error::invalid_length(
                            N + 1,
                            &"a fixed length byte array represented as an array of numbers",
                        ));
                    }
                }

                Ok(FixedBytes(bytes))
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                s.parse()
                    .map_err(|_| de::Error::invalid_value(de::Unexpected::Str(s), &"a hex string"))
            }
        }
        deserializer.deserialize_any(FbVisitor::<N>)
    }
}

#[cfg(test)]
mod test {
    use crate::PayloadKind;
    use crate::Vaa;

    #[test]
    fn basic_deser() {
        let json = serde_json::json!({
                "version": 1,
                "guardianSetIndex": 0,
                "signatures": [
                  {
                    "guardianSetIndex": 0,
                    "signature": "7d204ad9447c4dfd6be62406e7f5a05eec96300da4048e70ff530cfb52aec44807e98194990710ff166eb1b2eac942d38bc1cd6018f93662a6578d985e87c8d001"
                  }
                ],
                "timestamp": 1646343275,
                "nonce": 47293,
                "emitterChain": 1,
                "emitterAddress": "0xc69a1b1a65dd336bf1df6a77afb501fc25db7fc0938cb08595a9ef473265cb4f",
                "sequence": "3",
                "consistencyLevel": 32,
                "payload": [0, 1, 2]
        });
        let vaa: Vaa = serde_json::from_value(json).unwrap();

        assert_eq!(vaa.header.version, 1);
        assert_eq!(vaa.body.payload, PayloadKind::Binary(vec![0, 1, 2]))
    }
}
