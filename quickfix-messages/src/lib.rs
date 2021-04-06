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
    /// FIX standardized text representation
    fn as_fix_str(&self) -> &'static str;

    /// FIX value representation
    fn as_fix_value(&self) -> String;

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
