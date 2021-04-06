use convert_case::{Case, Casing};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{BufReader, BufWriter, Write};
use std::path::Path;

// =====================================
// Define common code block;

static CODE_HEADER: &str = "
#[allow(unused_imports)]
use chrono::prelude::*;
use std::fmt;

use crate::prelude::*;

";

// =====================================
// Basic types

/// Basic wrapper to convert FIX required field to boolean
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum Required {
    Y,
    N,
}

impl Into<bool> for Required {
    fn into(self) -> bool {
        match self {
            Self::Y => true,
            Self::N => false,
        }
    }
}

impl Default for Required {
    fn default() -> Self {
        Self::N
    }
}

/// Allowed message receiver category
#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MessageCategory {
    Admin,
    App,
}

/// Know type (so much types defined in FIX protocol)
#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum FieldType {
    Boolean,
    Char,
    Int,
    Float,
    String,
    Seqnum,
    Length,
    UtcTimestamp,
    MonthYear,
    DayOfMonth,
    UtcDate,
    UtcDateOnly,
    Date,
    UtcTimeOnly,
    Time,
    Data,
    NumInGroup,
    Price,
    #[serde(rename = "AMT")]
    Amount,
    #[serde(rename = "QTY")]
    Quantity,
    Currency,
    MultipleValueString,
    Exchange,
    #[serde(rename = "LOCALMKTDATE")]
    LocalMarketDate,
    PriceOffset,
    Percentage,
    Country,
}

impl FieldType {
    fn as_rust_type(&self) -> &'static str {
        // Some type may be improved
        match *self {
            Self::Boolean => "bool",
            Self::Char => "char",
            Self::Int => "i32",
            Self::Float => "f64",
            Self::String => "String",
            Self::Seqnum => "usize",
            Self::Length => "usize",
            Self::UtcTimestamp => "f64",
            Self::MonthYear => "String",
            Self::DayOfMonth => "u8",
            Self::UtcDate => "chrono::NaiveDate",
            Self::UtcDateOnly => "String",
            Self::Date => "String",
            Self::UtcTimeOnly => "String",
            Self::Time => "chrono::NaiveTime",
            Self::Data => "String",
            Self::NumInGroup => "String",
            Self::Price => "f64",
            Self::Amount => "f64",
            Self::Quantity => "f64",
            Self::Currency => "String",
            Self::MultipleValueString => "String",
            Self::Exchange => "String",
            Self::LocalMarketDate => "String",
            Self::PriceOffset => "f64",
            Self::Percentage => "f64",
            Self::Country => "String",
        }
    }
}

// =====================================
/// Reference to FieldDef

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct FieldRef {
    name: String,
    required: Required,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ComponentRef {
    name: String,
    #[serde(default)]
    required: Required,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct GroupRef {
    name: String,
    required: Required,

    #[serde(rename = "$value")]
    refs: Vec<Reference>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum Reference {
    #[serde(rename = "field")]
    FieldRef(FieldRef),

    #[serde(rename = "component")]
    ComponentRef(ComponentRef),

    #[serde(rename = "group")]
    GroupRef(GroupRef),
}

// =====================================
// Message spec

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct HeaderSpec {
    #[serde(rename = "$value")]
    refs: Vec<Reference>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct TrailerSpec {
    #[serde(rename = "$value")]
    refs: Vec<Reference>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct MessagesSpec {
    #[serde(rename = "$value")]
    items: Vec<Message>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Message {
    msgcat: MessageCategory,
    msgtype: String,
    name: String,
    #[serde(rename = "$value", default)]
    refs: Vec<Reference>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Component {
    name: String,
    #[serde(rename = "$value", default)]
    refs: Vec<Reference>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ComponentSpec {
    #[serde(rename = "$value", default)]
    items: Vec<Component>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct FieldValue {
    #[serde(rename = "enum")]
    value: String,
    description: String,
}

impl FieldValue {
    fn as_rust_desc(&self) -> String {
        assert!(!self.description.is_empty());
        if !char::is_ascii_alphabetic(&self.description.chars().nth(0).unwrap()) {
            format!("Value{}", self.description.to_case(Case::UpperCamel))
        } else {
            self.description.to_case(Case::UpperCamel)
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct FieldDef {
    name: String,
    number: u32,
    #[serde(rename = "type")]
    field_type: FieldType,
    #[serde(rename = "value", default)]
    values: Vec<FieldValue>,
}

impl FieldDef {
    fn as_code(&self) -> String {
        if self.values.is_empty() {
            format!(
                "
#[derive(Debug, PartialEq)]
pub struct {field_name} {{
    pub value: {content_type}
}}

impl {field_name} {{
    pub fn new(value: {content_type}) -> Self {{
        Self {{ value }}
    }}
}}

impl FixID for {field_name} {{
    const FIELD_ID: u32 = {field_id};
}}

impl fmt::Display for {field_name} {{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {{
        write!(f, \"{field_name_upper}({{}})\", self.value)
    }}
}}

impl AsFixMessage for {field_name} {{
    fn as_fix_value(&self) -> String {{
        format!(\"{{}}\", self.value)
    }}
}}

impl FromFixMessage for {field_name} {{
    fn from_fix_value(value: &str) -> Result<Self, FixParseError> {{
        let value = value.parse().map_err(|_e| FixParseError::InvalidData)?;
        Ok(Self {{ value }})
    }}
}}

",
                field_name = self.name,
                field_name_upper = self.name.to_case(Case::UpperSnake),
                field_id = self.number,
                content_type = self.field_type.as_rust_type(),
            )
        } else {
            assert!(matches!(
                self.field_type,
                FieldType::String
                    | FieldType::Char
                    | FieldType::Int
                    | FieldType::MultipleValueString
                    | FieldType::Boolean
                    | FieldType::NumInGroup
            ));

            let field_names = self
                .values
                .iter()
                .map(|x| format!("\t{},", x.as_rust_desc()))
                .collect::<Vec<_>>()
                .join("\n");

            let as_field_descriptions = self
                .values
                .iter()
                .map(|x| format!("\t\t\tSelf::{} => \"{}\",", x.as_rust_desc(), x.description))
                .collect::<Vec<_>>()
                .join("\n");

            let as_field_values = self
                .values
                .iter()
                .map(|x| format!("\t\t\tSelf::{} => \"{}\",", x.as_rust_desc(), x.value))
                .collect::<Vec<_>>()
                .join("\n");

            let from_field_values = self
                .values
                .iter()
                .map(|x| format!("\t\t\t\"{}\" => Ok(Self::{}),", x.value, x.as_rust_desc()))
                .collect::<Vec<_>>()
                .join("\n");

            format!(
                "
#[derive(Debug, PartialEq)]
pub enum {field_name} {{
{field_names}
}}

impl FixID for {field_name} {{
    const FIELD_ID: u32 = {field_id};
}}

impl fmt::Display for {field_name} {{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {{
        write!(f, \"{{}}\", match *self {{
{as_field_descriptions}
        }})
    }}
}}

impl AsFixMessage for {field_name} {{
    fn as_fix_value(&self) -> String {{
        match *self {{
{as_field_values}
        }}.to_string()
    }}
}}

impl FromFixMessage for {field_name} {{
    fn from_fix_value(value: &str) -> Result<Self, FixParseError> {{
        match value {{
{from_field_values}
            _ => Err(FixParseError::InvalidData),
        }}
    }}
}}

",
                field_name = self.name,
                field_id = self.number,
                field_names = field_names,
                as_field_descriptions = as_field_descriptions,
                as_field_values = as_field_values,
                from_field_values = from_field_values,
            )
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct FieldSpec {
    #[serde(rename = "field")]
    items: Vec<FieldDef>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct FixSpec {
    /// Fix major revision
    major: u8,
    /// Fix minor revision
    minor: u8,
    /// Fix servicepack revision
    servicepack: u8,

    /// Message header allowed fields references
    header: HeaderSpec,
    /// Message trailer allowed fields references
    trailer: TrailerSpec,

    /// Message components (common group like)
    #[serde(rename = "components")]
    component: ComponentSpec,

    /// Know networks and standardized messages
    #[serde(rename = "messages")]
    message: MessagesSpec,

    /// Message known fields
    #[serde(rename = "fields")]
    field: FieldSpec,
}

impl FixSpec {
    pub fn generate_specfile<P>(&self, out_dir: P, src_filename: P) -> anyhow::Result<()>
    where
        P: AsRef<Path>,
    {
        let stem = src_filename.as_ref().file_stem().unwrap().to_str().unwrap();

        macro_rules! open_file_writer {
            ($($arg:tt)*)  => {
                BufWriter::new(fs::File::create(out_dir.as_ref().join(format!($($arg)*)))?)
            };
        }

        // Dump parsed XML
        let f_parsed = open_file_writer!("{}.parsed.json", stem);
        serde_json::to_writer_pretty(f_parsed, self)?;

        // Generate code
        let mut f_code = open_file_writer!("{}_fields.rs", stem);

        // Generate fields
        write!(f_code, "{}", CODE_HEADER)?;
        for field in &self.field.items {
            write!(f_code, "{}", field.as_code())?;
        }

        Ok(())
    }
}

pub fn parse<P: AsRef<Path>>(path: P) -> anyhow::Result<FixSpec> {
    let file = fs::File::open(path)?;
    let buf = BufReader::new(file);
    let spec = quick_xml::de::from_reader(buf)?;
    Ok(spec)
}

#[derive(Debug, Default)]
pub struct Builder<P>
where
    P: AsRef<Path>,
{
    paths: Vec<P>,
}

impl<P> Builder<P>
where
    P: AsRef<Path>,
{
    pub fn new() -> Self {
        Self { paths: vec![] }
    }

    pub fn add_path(mut self, path: P) -> Self {
        self.paths.push(path);
        self
    }

    pub fn build(&self, out_dir: P) -> anyhow::Result<()> {
        self.paths.iter().try_for_each(|file| {
            let spec = parse(file.as_ref())?;
            spec.generate_specfile(out_dir.as_ref(), file.as_ref())?;
            Ok(())
        })
    }
}
