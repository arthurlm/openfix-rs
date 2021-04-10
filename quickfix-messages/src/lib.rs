use format_bytes::format_bytes;
use std::num::ParseIntError;
use thiserror::Error;

pub mod parse_helpers;

#[cfg(feature = "fixt11")]
pub mod fixt11 {
    pub mod fields {
        include!(concat!(env!("OUT_DIR"), "/FIXT11_fields.rs"));
    }
    pub mod messages {
        include!(concat!(env!("OUT_DIR"), "/FIXT11_messages.rs"));
    }
}

#[cfg(feature = "fix40")]
pub mod fix40 {
    pub mod fields {
        include!(concat!(env!("OUT_DIR"), "/FIX40_fields.rs"));
    }
    pub mod messages {
        include!(concat!(env!("OUT_DIR"), "/FIX40_messages.rs"));
    }
}

#[cfg(feature = "fix41")]
pub mod fix41 {
    pub mod fields {
        include!(concat!(env!("OUT_DIR"), "/FIX41_fields.rs"));
    }
    pub mod messages {
        include!(concat!(env!("OUT_DIR"), "/FIX41_messages.rs"));
    }
}

#[cfg(feature = "fix42")]
pub mod fix42 {
    pub mod fields {
        include!(concat!(env!("OUT_DIR"), "/FIX42_fields.rs"));
    }
    pub mod messages {
        include!(concat!(env!("OUT_DIR"), "/FIX42_messages.rs"));
    }
}

#[cfg(feature = "fix43")]
pub mod fix43 {
    pub mod fields {
        include!(concat!(env!("OUT_DIR"), "/FIX43_fields.rs"));
    }
    pub mod messages {
        include!(concat!(env!("OUT_DIR"), "/FIX43_messages.rs"));
    }
}

#[cfg(feature = "fix44")]
pub mod fix44 {
    pub mod fields {
        include!(concat!(env!("OUT_DIR"), "/FIX44_fields.rs"));
    }
    pub mod messages {
        include!(concat!(env!("OUT_DIR"), "/FIX44_messages.rs"));
    }
}

pub mod prelude {
    pub use super::{
        AsFixMessage, AsFixMessageField, FixParseError, FromFixMessage, FromFixMessageField,
        MessageDest,
    };
}

pub trait AsFixMessageField {
    /// FIX value representation
    fn as_fix_value(&self) -> String;

    /// Fix key representation
    fn as_fix_key(&self) -> u32;

    /// Encode field as "Key=Value"
    fn encode_message(&self) -> Vec<u8> {
        format_bytes!(b"{}={}", self.as_fix_key(), self.as_fix_value().as_bytes()).to_vec()
    }
}

#[derive(Debug, PartialEq, Error)]
pub enum FixParseError {
    #[error("invalid key: {0}")]
    InvalidKey(#[from] ParseIntError),

    #[error("invalid key ID: {0}")]
    InvalidKeyId(u32),

    #[error("invalid data")]
    InvalidData,
}

pub trait FromFixMessageField {
    /// FIX value representation
    fn from_fix_value(value: &str) -> Result<Self, FixParseError>
    where
        Self: Sized;

    fn decode_field(value: &str, field_id: u32) -> Result<Self, FixParseError>
    where
        Self: Sized,
    {
        let values: Vec<_> = value.splitn(2, '=').collect();
        match values[..] {
            [key, payload] => {
                let key_id: i64 = key.parse()?;
                if key_id as u32 == field_id {
                    Self::from_fix_value(payload)
                } else {
                    Err(FixParseError::InvalidKeyId(key_id as u32))
                }
            }
            _ => Err(FixParseError::InvalidData),
        }
    }
}

pub trait AsFixMessage {
    fn encode_message(&self) -> Vec<u8>;
}

pub trait FromFixMessage {
    fn decode_message(data: &[u8]) -> Result<Self, FixParseError>
    where
        Self: Sized;
}

#[derive(Debug, PartialEq)]
pub enum MessageDest {
    Admin,
    App,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    struct TestStruct {
        value: String,
    }

    impl AsFixMessageField for TestStruct {
        fn as_fix_value(&self) -> String {
            self.value.clone()
        }

        fn as_fix_key(&self) -> u32 {
            42
        }
    }

    impl FromFixMessageField for TestStruct {
        fn from_fix_value(value: &str) -> Result<Self, FixParseError> {
            Ok(Self {
                value: value.into(),
            })
        }
    }

    #[test]
    fn test_struct_encode() {
        let field = TestStruct {
            value: "foobar".into(),
        };
        assert_eq!(field.encode_message(), b"42=foobar");
    }

    #[test]
    fn test_struct_decode() {
        assert_eq!(
            TestStruct::decode_field("foo", 42),
            Err(FixParseError::InvalidData)
        );
        assert_eq!(
            TestStruct::decode_field("foo=bar", 42),
            Err(FixParseError::InvalidKey("foo".parse::<i32>().unwrap_err()))
        );
        assert_eq!(
            TestStruct::decode_field("12=bar", 42),
            Err(FixParseError::InvalidKeyId(12))
        );
        assert_eq!(
            TestStruct::decode_field("42=foobar", 42),
            Ok(TestStruct {
                value: "foobar".into(),
            })
        );
    }

    #[derive(Debug, PartialEq)]
    enum TestEnum {
        Opt1,
        Opt2,
    }

    impl AsFixMessageField for TestEnum {
        fn as_fix_value(&self) -> String {
            match *self {
                Self::Opt1 => "opt1",
                Self::Opt2 => "opt2",
            }
            .to_string()
        }

        fn as_fix_key(&self) -> u32 {
            29
        }
    }

    impl FromFixMessageField for TestEnum {
        fn from_fix_value(value: &str) -> Result<Self, FixParseError> {
            match value {
                "opt1" => Ok(Self::Opt1),
                "opt2" => Ok(Self::Opt2),
                _ => Err(FixParseError::InvalidData),
            }
        }
    }

    #[test]
    fn test_enum_encode() {
        let field = TestEnum::Opt1;
        assert_eq!(field.encode_message(), b"29=opt1");
        let field = TestEnum::Opt2;
        assert_eq!(field.encode_message(), b"29=opt2");
    }

    #[test]
    fn test_enum_decode() {
        assert_eq!(
            TestEnum::decode_field("foo", 29),
            Err(FixParseError::InvalidData)
        );
        assert_eq!(
            TestEnum::decode_field("foo=bar", 29),
            Err(FixParseError::InvalidKey("foo".parse::<i32>().unwrap_err()))
        );
        assert_eq!(
            TestEnum::decode_field("12=bar", 29),
            Err(FixParseError::InvalidKeyId(12))
        );
        assert_eq!(TestEnum::decode_field("29=opt1", 29), Ok(TestEnum::Opt1));
        assert_eq!(TestEnum::decode_field("29=opt2", 29), Ok(TestEnum::Opt2));
    }
}
