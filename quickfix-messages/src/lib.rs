pub mod dec_helpers;
pub mod enc_helpers;

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

#[cfg(feature = "test_spec")]
pub mod test_spec {
    pub mod fields {
        include!(concat!(env!("OUT_DIR"), "/TEST_SPEC_fields.rs"));
    }
    pub mod messages {
        include!(concat!(env!("OUT_DIR"), "/TEST_SPEC_messages.rs"));
    }
}

#[cfg(feature = "test_spec")]
pub mod test_spec_sig {
    pub mod fields {
        include!(concat!(env!("OUT_DIR"), "/TEST_SPEC_SIG_fields.rs"));
    }
    pub mod messages {
        include!(concat!(env!("OUT_DIR"), "/TEST_SPEC_SIG_messages.rs"));
    }
}

pub mod prelude {
    pub use super::{
        AsFixMessage, AsFixMessageField, FixParseError, FromFixMessage, FromFixMessageField,
        MessageDest,
    };
}

use format_bytes::format_bytes;
use std::str::Utf8Error;
use thiserror::Error;

use crate::dec_helpers::FixFieldItems;

pub trait AsFixMessageField {
    /// Fix key representation
    const FIX_KEY: u32;

    /// FIX value representation
    fn as_fix_value(&self) -> String;

    /// Encode field as "Key=Value"
    fn encode_message(&self) -> Vec<u8> {
        format_bytes!(b"{}={}\x01", Self::FIX_KEY, self.as_fix_value().as_bytes()).to_vec()
    }
}

#[derive(Debug, PartialEq, Error)]
pub enum FixParseError {
    #[error("invalid data")]
    InvalidData,

    #[error("invalid string")]
    InvalidString(#[from] Utf8Error),

    #[error("no data for this field ID")]
    NoData,
}

pub trait FromFixMessageField: AsFixMessageField {
    /// FIX value representation
    fn from_fix_value(value: &[u8]) -> Result<Self, FixParseError>
    where
        Self: Sized;

    /// Decode field from map of (key ID => value) data
    fn decode_message(items: &FixFieldItems) -> Result<Self, FixParseError>
    where
        Self: Sized,
    {
        let key_id = Self::FIX_KEY;
        let data = items
            .get(&key_id)
            .ok_or_else(|| FixParseError::InvalidData)?;

        Self::from_fix_value(&data)
    }
}

pub trait AsFixMessage {
    fn encode_message(&self) -> Vec<u8>;
}

pub trait FromFixMessage {
    fn decode_message(items: &FixFieldItems) -> Result<Self, FixParseError>
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

    use crate::dec_helpers::split_message_items;

    #[derive(Debug, PartialEq)]
    struct TestStruct {
        value: String,
    }

    impl AsFixMessageField for TestStruct {
        const FIX_KEY: u32 = 42;

        fn as_fix_value(&self) -> String {
            self.value.clone()
        }
    }

    impl FromFixMessageField for TestStruct {
        fn from_fix_value(value: &[u8]) -> Result<Self, FixParseError> {
            Ok(Self {
                value: std::str::from_utf8(value)?.to_string(),
            })
        }
    }

    #[test]
    fn test_struct_encode() {
        let field = TestStruct {
            value: "foobar".into(),
        };
        assert_eq!(field.encode_message(), b"42=foobar\x01");
    }

    #[test]
    fn test_struct_decode() {
        assert_eq!(
            TestStruct::decode_message(&split_message_items(b"foo")),
            Err(FixParseError::InvalidData)
        );
        assert_eq!(
            TestStruct::decode_message(&split_message_items(b"foo=bar")),
            Err(FixParseError::InvalidData)
        );
        assert_eq!(
            TestStruct::decode_message(&split_message_items(b"12=bar")),
            Err(FixParseError::InvalidData)
        );
        assert_eq!(
            TestStruct::decode_message(&split_message_items(b"42=foobar")),
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
        const FIX_KEY: u32 = 29;

        fn as_fix_value(&self) -> String {
            match *self {
                Self::Opt1 => "opt1",
                Self::Opt2 => "opt2",
            }
            .to_string()
        }
    }

    impl FromFixMessageField for TestEnum {
        fn from_fix_value(value: &[u8]) -> Result<Self, FixParseError> {
            match value {
                b"opt1" => Ok(Self::Opt1),
                b"opt2" => Ok(Self::Opt2),
                _ => Err(FixParseError::InvalidData),
            }
        }
    }

    #[test]
    fn test_enum_encode() {
        let field = TestEnum::Opt1;
        assert_eq!(field.encode_message(), b"29=opt1\x01");
        let field = TestEnum::Opt2;
        assert_eq!(field.encode_message(), b"29=opt2\x01");
    }

    #[test]
    fn test_enum_decode() {
        assert_eq!(
            TestEnum::decode_message(&split_message_items(b"foo")),
            Err(FixParseError::InvalidData)
        );
        assert_eq!(
            TestEnum::decode_message(&split_message_items(b"foo=bar")),
            Err(FixParseError::InvalidData)
        );
        assert_eq!(
            TestEnum::decode_message(&split_message_items(b"12=bar")),
            Err(FixParseError::InvalidData)
        );
        assert_eq!(
            TestEnum::decode_message(&split_message_items(b"29=opt1")),
            Ok(TestEnum::Opt1)
        );
        assert_eq!(
            TestEnum::decode_message(&split_message_items(b"29=opt2")),
            Ok(TestEnum::Opt2)
        );
    }
}
