pub mod fixt11 {
    include!(concat!(env!("OUT_DIR"), "/FIXT11_fields.rs"));
}

pub mod fix40 {
    include!(concat!(env!("OUT_DIR"), "/FIX40_fields.rs"));
}

pub mod fix41 {
    include!(concat!(env!("OUT_DIR"), "/FIX41_fields.rs"));
}

pub mod fix42 {
    include!(concat!(env!("OUT_DIR"), "/FIX42_fields.rs"));
}

pub mod fix43 {
    include!(concat!(env!("OUT_DIR"), "/FIX43_fields.rs"));
}

pub mod fix44 {
    include!(concat!(env!("OUT_DIR"), "/FIX44_fields.rs"));
}

pub trait FixID {
    /// FIX field ID
    const FIELD_ID: usize;
}

pub trait AsFixMessage: FixID {
    /// FIX value representation
    fn as_fix_value(&self) -> String;

    /// Encode field as "Key=Value"
    fn encode_field(&self) -> String {
        format!("{}={}", Self::FIELD_ID, self.as_fix_value())
    }
}

#[derive(Debug)]
pub enum FixParseError {
    InvalidData,
}

pub trait FromFixMessage: FixID {
    /// FIX value representation
    fn from_fix_value(value: &str) -> Result<Self, FixParseError>
    where
        Self: Sized;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    struct TestStruct {
        value: String,
    }

    impl FixID for TestStruct {
        const FIELD_ID: usize = 42;
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

    #[derive(Debug, PartialEq)]
    enum TestEnum {
        Opt1,
        Opt2,
    }

    impl FixID for TestEnum {
        const FIELD_ID: usize = 29;
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
}
