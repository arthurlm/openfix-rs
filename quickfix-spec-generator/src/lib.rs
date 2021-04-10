use convert_case::{Case, Casing};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{BufReader, BufWriter, Write};
use std::path::Path;
use std::process::Command;

// =====================================
// Define common code block;

static CODE_HEADER: &str = "
#[allow(unused_imports)]
use chrono::prelude::*;

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

macro_rules! sanitize_field_name {
    // Check if field is not a keyword (like "yield" for example)
    ($x:expr) => {
        match syn::parse_str::<syn::Ident>($x) {
            Err(_) => format!("r#{}", $x),
            Ok(_) => $x.into(),
        }
    };
}

fn format_optional_struct_field(required: &Required, field_name: &str, type_name: &str) -> String {
    let field_name = sanitize_field_name!(field_name);
    match required {
        Required::Y => format!("{}: {}", field_name, type_name),
        Required::N => format!("{}: Option<{}>", field_name, type_name),
    }
}

fn format_optional_function_call(required: &Required, field_name: &str, call_name: &str) -> String {
    let field_name = sanitize_field_name!(field_name);
    match required {
        Required::Y => format!("Some(self.{}.{})", field_name, call_name),
        Required::N => format!("self.{}.as_ref().map(|x| x.{})", field_name, call_name),
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct FieldRef {
    name: String,
    required: Required,
}

impl FieldRef {
    fn as_struct_field_item(&self) -> String {
        format_optional_struct_field(&self.required, &self.name.to_case(Case::Snake), &self.name)
    }

    fn as_function_call(&self, call_name: &str) -> String {
        format_optional_function_call(&self.required, &self.name.to_case(Case::Snake), call_name)
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ComponentRef {
    name: String,
    #[serde(default)]
    required: Required,
}

impl ComponentRef {
    fn as_struct_field_item(&self) -> String {
        format_optional_struct_field(&self.required, &self.name.to_case(Case::Snake), &self.name)
    }

    fn as_function_call(&self, call_name: &str) -> String {
        format_optional_function_call(&self.required, &self.name.to_case(Case::Snake), call_name)
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct GroupRef {
    name: String,
    required: Required,

    #[serde(rename = "$value")]
    refs: Vec<Reference>,
}

impl GroupRef {
    fn cls_name(&self, cls_prefix: &str) -> String {
        format!("{}{}", cls_prefix, self.name.to_case(Case::UpperCamel))
    }

    fn as_group_struct(&self, cls_prefix: &str) -> String {
        let group_elements: Vec<_> = self
            .refs
            .iter()
            .map(|x| format!("\t{0}({0}),", x.as_type_value()))
            .collect();

        let group_encode: Vec<_> = self
            .refs
            .iter()
            .map(|x| {
                format!(
                    "\t\t\tSelf::{}(ref x) => x.encode_message(),",
                    x.as_type_value()
                )
            })
            .collect();

        format!(
            "
#[derive(Debug, PartialEq)]
pub enum {cls_name} {{
{group_elements}
}}

impl AsFixMessage for {cls_name} {{
    fn encode_message(&self) -> Vec<u8> {{
        match *self {{
{group_encode}
        }}
    }}
}}
",
            cls_name = self.cls_name(cls_prefix),
            group_elements = group_elements.join("\n"),
            group_encode = group_encode.join("\n"),
        )
    }

    fn as_struct_field_item(&self, cls_prefix: &str) -> String {
        format_optional_struct_field(
            &self.required,
            &self.name.to_case(Case::Snake),
            &self.cls_name(cls_prefix),
        )
    }

    fn as_function_call(&self, call_name: &str, _cls_prefix: &str) -> String {
        format_optional_function_call(&self.required, &self.name.to_case(Case::Snake), call_name)
    }
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

impl Reference {
    fn as_type_value<'a>(&'a self) -> &'a str {
        match self {
            Self::FieldRef(x) => &x.name,
            Self::ComponentRef(x) => &x.name,
            Self::GroupRef(x) => &x.name,
        }
    }

    fn as_group_struct(&self, cls_prefix: &str) -> Option<String> {
        match self {
            Self::FieldRef(_) => None,
            Self::ComponentRef(_) => None,
            Self::GroupRef(x) => Some(x.as_group_struct(cls_prefix)),
        }
    }

    fn as_struct_field_item(&self, cls_prefix: &str) -> String {
        match self {
            Self::FieldRef(x) => x.as_struct_field_item(),
            Self::ComponentRef(x) => x.as_struct_field_item(),
            Self::GroupRef(x) => x.as_struct_field_item(cls_prefix),
        }
    }

    fn as_function_call(&self, cls_prefix: &str, call_name: &str) -> String {
        match self {
            Self::FieldRef(x) => x.as_function_call(call_name),
            Self::ComponentRef(x) => x.as_function_call(call_name),
            Self::GroupRef(x) => x.as_function_call(call_name, cls_prefix),
        }
    }
}

// =====================================
// Shared message elements

#[derive(Debug)]
struct RefGeneratedCode {
    classes: Vec<String>,
    fields: Vec<String>,
    fields_encode: Vec<String>,
}

fn generate_ref_code(refs: &Vec<Reference>, cls_prefix: &str) -> RefGeneratedCode {
    let classes = refs
        .iter()
        .filter_map(|x| x.as_group_struct(cls_prefix))
        .collect();

    let fields = refs
        .iter()
        .map(|x| format!("\t{},", x.as_struct_field_item(cls_prefix)))
        .collect();

    let fields_encode = refs
        .iter()
        .map(|x| {
            format!(
                "\t\t\t{},",
                x.as_function_call(cls_prefix, "encode_message()")
            )
        })
        .collect();

    RefGeneratedCode {
        classes,
        fields,
        fields_encode,
    }
}

fn spec_as_code(cls_name: &str, refs: &Vec<Reference>) -> String {
    let gen = generate_ref_code(refs, cls_name);

    format!(
        "
{classes}

#[derive(Debug, PartialEq)]
pub struct {cls_name} {{
{fields}
}}

impl AsFixMessage for {cls_name} {{
    fn encode_message(&self) -> Vec<u8> {{
        let fields: Vec<Option<_>> = vec![
{fields_encode}
        ];

        let mut result = vec![];
        for field in fields {{
            if let Some(field) = field {{
                result.push(field);
                result.push(b\"\\x01\".to_vec());
            }}
        }}

        result.concat()
    }}
}}

",
        cls_name = cls_name,
        fields = gen.fields.join("\n"),
        classes = gen.classes.join("\n"),
        fields_encode = gen.fields_encode.join("\n"),
    )
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct HeaderSpec {
    #[serde(rename = "$value", default)]
    refs: Vec<Reference>,
}

impl HeaderSpec {
    fn as_code(&self) -> String {
        spec_as_code("MessageHeader", &self.refs)
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct TrailerSpec {
    #[serde(rename = "$value", default)]
    refs: Vec<Reference>,
}

impl TrailerSpec {
    fn as_code(&self) -> String {
        spec_as_code("MessageTrailer", &self.refs)
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Component {
    name: String,
    #[serde(rename = "$value", default)]
    refs: Vec<Reference>,
}

impl Component {
    fn as_code(&self) -> String {
        spec_as_code(&self.name, &self.refs)
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ComponentSpec {
    #[serde(rename = "$value", default)]
    items: Vec<Component>,
}

// =====================================
// Message spec

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

impl Message {
    fn message_cls_name(&self) -> String {
        format!("Message{}", self.name)
    }

    fn message_dest(&self) -> &'static str {
        match &self.msgcat {
            MessageCategory::Admin => "MessageDest::Admin",
            MessageCategory::App => "MessageDest::App",
        }
    }

    fn as_code(&self) -> String {
        let gen = generate_ref_code(&self.refs, &self.name);

        format!(
            "
{classes}

#[derive(Debug, PartialEq)]
pub struct {message_cls_name} {{
    // Common fields
    header: MessageHeader,
    trailer: MessageTrailer,

    // Custom fields
{fields}
}}

impl {message_cls_name} {{
    pub const MESSAGE_DEST: MessageDest = {message_dest};
    pub const MESSAGE_TYPE: &'static str = \"{msg_type}\";
}}

impl AsFixMessage for {message_cls_name} {{
    fn encode_message(&self) -> Vec<u8> {{
        let fields: Vec<Option<_>> = vec![
            Some(self.header.encode_message()),
{fields_encode}
            Some(self.trailer.encode_message()),
        ];

        let mut result = vec![];
        for field in fields {{
            if let Some(field) = field {{
                result.push(field);
                result.push(b\"\\x01\".to_vec());
            }}
        }}

        result.concat()
    }}
}}
",
            message_cls_name = self.message_cls_name(),
            message_dest = self.message_dest(),
            msg_type = self.msgtype,
            classes = gen.classes.join("\n"),
            fields = gen.fields.join("\n"),
            fields_encode = gen.fields_encode.join("\n"),
        )
    }
}

// =====================================
// Field spec

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

impl fmt::Display for {field_name} {{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {{
        write!(f, \"{field_name_upper}({{}})\", self.value)
    }}
}}

impl AsFixMessageField for {field_name} {{
    fn as_fix_value(&self) -> String {{
        format!(\"{{}}\", self.value)
    }}

    fn as_fix_key(&self) -> u32 {{
        {field_id}
    }}
}}

impl FromFixMessageField for {field_name} {{
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

impl fmt::Display for {field_name} {{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {{
        write!(f, \"{{}}\", match *self {{
{as_field_descriptions}
        }})
    }}
}}

impl AsFixMessageField for {field_name} {{
    fn as_fix_value(&self) -> String {{
        match *self {{
{as_field_values}
        }}.to_string()
    }}

    fn as_fix_key(&self) -> u32 {{
        {field_id}
    }}
}}

impl FromFixMessageField for {field_name} {{
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

// =====================================
// Fix spec

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct FixSpec {
    /// Fix major revision
    major: u8,
    /// Fix minor revision
    minor: u8,
    /// Fix servicepack revision
    servicepack: u8,

    /// Message header allowed fields references
    header: Option<HeaderSpec>,
    /// Message trailer allowed fields references
    trailer: Option<TrailerSpec>,

    /// Message components (common group like)
    #[serde(rename = "components")]
    component: Option<ComponentSpec>,

    /// Know networks and standardized messages
    #[serde(rename = "messages")]
    message: MessagesSpec,

    /// Message known fields
    #[serde(rename = "fields")]
    field: FieldSpec,
}

impl FixSpec {
    pub fn generate_specfile<P>(
        &self,
        out_dir: P,
        src_filename: P,
        enable_rustfmt: bool,
    ) -> anyhow::Result<()>
    where
        P: AsRef<Path>,
    {
        let stem = src_filename.as_ref().file_stem().unwrap().to_str().unwrap();

        macro_rules! out_file_path {
            ($($arg:tt)*)  => {
                out_dir.as_ref().join(format!($($arg)*))
            };
        }

        macro_rules! open_file_writer {
            ($($arg:tt)*)  => {
                BufWriter::new(fs::File::create(out_file_path!($($arg)*))?)
            };
        }

        // Dump parsed XML
        let f_parsed = open_file_writer!("{}.parsed.json", stem);
        serde_json::to_writer_pretty(f_parsed, self)?;

        // Generate code
        let mut f_fields = open_file_writer!("{}_fields.rs", stem);
        let mut f_messages = open_file_writer!("{}_messages.rs", stem);

        write!(f_fields, "{}", CODE_HEADER)?;
        write!(
            f_fields,
            "
#[allow(unused_imports)]
use std::fmt;

#[allow(unused_imports)]
use crate::prelude::*;

",
        )?;
        write!(f_messages, "{}", CODE_HEADER)?;
        write!(
            f_messages,
            "
#[allow(unused_imports)]
use super::fields::*;

#[allow(unused_imports)]
use crate::prelude::*;

",
        )?;

        // Generate fields
        for field in &self.field.items {
            write!(f_fields, "{}", field.as_code())?;
        }

        // Generate headers
        if let Some(header) = &self.header {
            write!(f_messages, "{}", header.as_code())?;
        }

        // Generate trailers
        if let Some(trailer) = &self.trailer {
            write!(f_messages, "{}", trailer.as_code())?;
        }

        // Generate components
        if let Some(component) = &self.component {
            for item in &component.items {
                write!(f_messages, "{}", item.as_code())?;
            }
        }

        // Generate messages
        for message in &self.message.items {
            write!(f_messages, "{}", message.as_code())?;
        }

        drop(f_fields);
        drop(f_messages);

        if enable_rustfmt {
            Command::new("rustfmt")
                .arg(out_file_path!("{}_fields.rs", stem))
                .arg(out_file_path!("{}_messages.rs", stem))
                .status()
                .expect("rustmft failed");
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
    enable_rustfmt: bool,
}

impl<P> Builder<P>
where
    P: AsRef<Path>,
{
    pub fn new() -> Self {
        Self {
            paths: vec![],
            enable_rustfmt: false,
        }
    }

    pub fn add_path(mut self, path: P) -> Self {
        self.paths.push(path);
        self
    }

    pub fn enable_rustfmt(mut self, value: bool) -> Self {
        self.enable_rustfmt = value;
        self
    }

    pub fn build(&self, out_dir: P) -> anyhow::Result<()> {
        self.paths.iter().try_for_each(|file| {
            let spec = parse(file)?;
            spec.generate_specfile(&out_dir, &file, self.enable_rustfmt)?;
            Ok(())
        })
    }
}
