use std::num::ParseIntError;
use thiserror::Error;

pub mod fixt11 {
    pub mod fields {
        include!(concat!(env!("OUT_DIR"), "/FIXT11_fields.rs"));
    }
    pub mod messages {
        include!(concat!(env!("OUT_DIR"), "/FIXT11_messages.rs"));
    }
}

pub mod fix40 {
    pub mod fields {
        include!(concat!(env!("OUT_DIR"), "/FIX40_fields.rs"));
    }
    pub mod messages {
        include!(concat!(env!("OUT_DIR"), "/FIX40_messages.rs"));
    }
}

pub mod fix41 {
    pub mod fields {
        include!(concat!(env!("OUT_DIR"), "/FIX41_fields.rs"));
    }
    pub mod messages {
        include!(concat!(env!("OUT_DIR"), "/FIX41_messages.rs"));
    }
}

pub mod fix42 {
    pub mod fields {
        include!(concat!(env!("OUT_DIR"), "/FIX42_fields.rs"));
    }
    pub mod messages {
        include!(concat!(env!("OUT_DIR"), "/FIX42_messages.rs"));
    }
}

pub mod fix43 {
    pub mod fields {
        include!(concat!(env!("OUT_DIR"), "/FIX43_fields.rs"));
    }
    pub mod messages {
        include!(concat!(env!("OUT_DIR"), "/FIX43_messages.rs"));
    }
}

pub mod fix44 {
    pub mod fields {
        include!(concat!(env!("OUT_DIR"), "/FIX44_fields.rs"));
    }
    pub mod messages {
        include!(concat!(env!("OUT_DIR"), "/FIX44_messages.rs"));
    }
}

pub mod prelude {
    pub use super::{AsFixMessage, FixID, FixParseError, FromFixMessage};
}

pub trait FixID {
    /// FIX field ID
    const FIELD_ID: u32;
}

pub trait AsFixMessage: FixID {
    /// FIX value representation
    fn as_fix_value(&self) -> String;

    /// Encode field as "Key=Value"
    fn encode_field(&self) -> String {
        format!("{}={}", Self::FIELD_ID, self.as_fix_value())
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

pub trait FromFixMessage: FixID {
    /// FIX value representation
    fn from_fix_value(value: &str) -> Result<Self, FixParseError>
    where
        Self: Sized;

    fn decode_field(value: &str) -> Result<Self, FixParseError>
    where
        Self: Sized,
    {
        let values: Vec<_> = value.splitn(2, '=').collect();
        match values[..] {
            [key, payload] => {
                let key_id: i64 = key.parse()?;
                if key_id as u32 == Self::FIELD_ID {
                    Self::from_fix_value(payload)
                } else {
                    Err(FixParseError::InvalidKeyId(key_id as u32))
                }
            }
            _ => Err(FixParseError::InvalidData),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    struct TestStruct {
        value: String,
    }

    impl FixID for TestStruct {
        const FIELD_ID: u32 = 42;
    }

    impl AsFixMessage for TestStruct {
        fn as_fix_value(&self) -> String {
            self.value.clone()
        }
    }

    impl FromFixMessage for TestStruct {
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
        assert_eq!(field.encode_field(), "42=foobar".to_string());
    }

    #[test]
    fn test_struct_decode() {
        assert_eq!(
            TestStruct::decode_field("foo"),
            Err(FixParseError::InvalidData)
        );
        assert_eq!(
            TestStruct::decode_field("foo=bar"),
            Err(FixParseError::InvalidKey("foo".parse::<i32>().unwrap_err()))
        );
        assert_eq!(
            TestStruct::decode_field("12=bar"),
            Err(FixParseError::InvalidKeyId(12))
        );
        assert_eq!(
            TestStruct::decode_field("42=foobar"),
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

    impl FixID for TestEnum {
        const FIELD_ID: u32 = 29;
    }

    impl AsFixMessage for TestEnum {
        fn as_fix_value(&self) -> String {
            match *self {
                Self::Opt1 => "opt1",
                Self::Opt2 => "opt2",
            }
            .to_string()
        }
    }

    impl FromFixMessage for TestEnum {
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
        assert_eq!(field.encode_field(), "29=opt1".to_string());
        let field = TestEnum::Opt2;
        assert_eq!(field.encode_field(), "29=opt2".to_string());
    }

    #[test]
    fn test_enum_decode() {
        assert_eq!(
            TestEnum::decode_field("foo"),
            Err(FixParseError::InvalidData)
        );
        assert_eq!(
            TestEnum::decode_field("foo=bar"),
            Err(FixParseError::InvalidKey("foo".parse::<i32>().unwrap_err()))
        );
        assert_eq!(
            TestEnum::decode_field("12=bar"),
            Err(FixParseError::InvalidKeyId(12))
        );
        assert_eq!(TestEnum::decode_field("29=opt1"), Ok(TestEnum::Opt1));
        assert_eq!(TestEnum::decode_field("29=opt2"), Ok(TestEnum::Opt2));
    }
}
