pub mod fixt11 {
    include!(concat!(env!("OUT_DIR"), "/FIXT11_gen.rs"));
}

pub mod fix40 {
    include!(concat!(env!("OUT_DIR"), "/FIX40_gen.rs"));
}

pub mod fix41 {
    include!(concat!(env!("OUT_DIR"), "/FIX41_gen.rs"));
}

pub mod fix42 {
    include!(concat!(env!("OUT_DIR"), "/FIX42_gen.rs"));
}

pub mod fix43 {
    include!(concat!(env!("OUT_DIR"), "/FIX43_gen.rs"));
}

pub mod fix44 {
    include!(concat!(env!("OUT_DIR"), "/FIX44_gen.rs"));
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
}

#[derive(Debug)]
pub enum FixParseError {
    InvalidData,
    MissingPayload,
}

pub trait FromFixMessage: FixID {
    /// FIX standardized text representation
    fn from_fix_str(value: &str) -> Result<Self, FixParseError>
    where
        Self: Sized;

    /// FIX value representation
    fn from_fix_value(value: &str) -> Result<Self, FixParseError>
    where
        Self: Sized;
}
