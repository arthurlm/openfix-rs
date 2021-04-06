use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{BufReader, BufWriter};
use std::path::Path;

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

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct FieldDef {
    name: String,
    number: usize,
    #[serde(rename = "type")]
    field_type: FieldType,
    #[serde(rename = "value", default)]
    values: Vec<FieldValue>,
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
    components: ComponentSpec,

    /// Know networks and standardized messages
    messages: MessagesSpec,

    /// Message known fields
    fields: FieldSpec,
}

impl FixSpec {
    pub fn generate_specfile<P>(&self, out_dir: P, src_filename: P) -> anyhow::Result<()>
    where
        P: AsRef<Path>,
    {
        let stem = src_filename.as_ref().file_stem().unwrap().to_str().unwrap();

        // Dump parsed XML
        let f_parsed = BufWriter::new(fs::File::create(
            out_dir.as_ref().join(format!("{}.parsed.json", stem)),
        )?);
        serde_json::to_writer_pretty(f_parsed, self)?;

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
