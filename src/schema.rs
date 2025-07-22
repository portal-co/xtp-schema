#![allow(clippy::redundant_closure_call)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::clone_on_copy)]

#[doc = r" Error types."]
pub mod error {
    #[doc = r" Error from a `TryFrom` or `FromStr` implementation."]
    pub struct ConversionError(::std::borrow::Cow<'static, str>);
    impl ::std::error::Error for ConversionError {}
    impl ::std::fmt::Display for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl ::std::fmt::Debug for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
#[doc = "`ArrayItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/ValueArrayItem\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/RefArrayItem\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/MapArrayItem\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum ArrayItem {
    ValueArrayItem(ValueArrayItem),
    RefArrayItem(RefArrayItem),
    MapArrayItem(MapArrayItem),
}
impl ::std::convert::From<&Self> for ArrayItem {
    fn from(value: &ArrayItem) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<ValueArrayItem> for ArrayItem {
    fn from(value: ValueArrayItem) -> Self {
        Self::ValueArrayItem(value)
    }
}
impl ::std::convert::From<RefArrayItem> for ArrayItem {
    fn from(value: RefArrayItem) -> Self {
        Self::RefArrayItem(value)
    }
}
impl ::std::convert::From<MapArrayItem> for ArrayItem {
    fn from(value: MapArrayItem) -> Self {
        Self::MapArrayItem(value)
    }
}
#[doc = "`CodeSample`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"lang\","]
#[doc = "    \"source\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"label\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"lang\": {"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"typescript\","]
#[doc = "            \"csharp\","]
#[doc = "            \"zig\","]
#[doc = "            \"rust\","]
#[doc = "            \"go\","]
#[doc = "            \"python\","]
#[doc = "            \"c++\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct CodeSample {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub label: ::std::option::Option<::std::string::String>,
    pub lang: CodeSampleLang,
    pub source: ::std::string::String,
}
impl ::std::convert::From<&CodeSample> for CodeSample {
    fn from(value: &CodeSample) -> Self {
        value.clone()
    }
}
impl CodeSample {
    pub fn builder() -> builder::CodeSample {
        Default::default()
    }
}
#[doc = "`CodeSampleLang`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"typescript\","]
#[doc = "        \"csharp\","]
#[doc = "        \"zig\","]
#[doc = "        \"rust\","]
#[doc = "        \"go\","]
#[doc = "        \"python\","]
#[doc = "        \"c++\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CodeSampleLang {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub subtype_0: ::std::option::Option<CodeSampleLangSubtype0>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub subtype_1: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&CodeSampleLang> for CodeSampleLang {
    fn from(value: &CodeSampleLang) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for CodeSampleLang {
    fn default() -> Self {
        Self {
            subtype_0: Default::default(),
            subtype_1: Default::default(),
        }
    }
}
impl CodeSampleLang {
    pub fn builder() -> builder::CodeSampleLang {
        Default::default()
    }
}
#[doc = "`CodeSampleLangSubtype0`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"typescript\","]
#[doc = "    \"csharp\","]
#[doc = "    \"zig\","]
#[doc = "    \"rust\","]
#[doc = "    \"go\","]
#[doc = "    \"python\","]
#[doc = "    \"c++\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CodeSampleLangSubtype0 {
    #[serde(rename = "typescript")]
    Typescript,
    #[serde(rename = "csharp")]
    Csharp,
    #[serde(rename = "zig")]
    Zig,
    #[serde(rename = "rust")]
    Rust,
    #[serde(rename = "go")]
    Go,
    #[serde(rename = "python")]
    Python,
    #[serde(rename = "c++")]
    C,
}
impl ::std::convert::From<&Self> for CodeSampleLangSubtype0 {
    fn from(value: &CodeSampleLangSubtype0) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for CodeSampleLangSubtype0 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Typescript => write!(f, "typescript"),
            Self::Csharp => write!(f, "csharp"),
            Self::Zig => write!(f, "zig"),
            Self::Rust => write!(f, "rust"),
            Self::Go => write!(f, "go"),
            Self::Python => write!(f, "python"),
            Self::C => write!(f, "c++"),
        }
    }
}
impl ::std::str::FromStr for CodeSampleLangSubtype0 {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "typescript" => Ok(Self::Typescript),
            "csharp" => Ok(Self::Csharp),
            "zig" => Ok(Self::Zig),
            "rust" => Ok(Self::Rust),
            "go" => Ok(Self::Go),
            "python" => Ok(Self::Python),
            "c++" => Ok(Self::C),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CodeSampleLangSubtype0 {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CodeSampleLangSubtype0 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CodeSampleLangSubtype0 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`ContentType`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"application/json\","]
#[doc = "    \"application/x-binary\","]
#[doc = "    \"text/plain; charset=utf-8\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ContentType {
    #[serde(rename = "application/json")]
    ApplicationJson,
    #[serde(rename = "application/x-binary")]
    ApplicationXBinary,
    #[serde(rename = "text/plain; charset=utf-8")]
    TextPlainCharsetUtf8,
}
impl ::std::convert::From<&Self> for ContentType {
    fn from(value: &ContentType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for ContentType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ApplicationJson => write!(f, "application/json"),
            Self::ApplicationXBinary => write!(f, "application/x-binary"),
            Self::TextPlainCharsetUtf8 => write!(f, "text/plain; charset=utf-8"),
        }
    }
}
impl ::std::str::FromStr for ContentType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "application/json" => Ok(Self::ApplicationJson),
            "application/x-binary" => Ok(Self::ApplicationXBinary),
            "text/plain; charset=utf-8" => Ok(Self::TextPlainCharsetUtf8),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ContentType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ContentType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ContentType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`EnumSchema`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"enum\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"description\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"enum\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\","]
#[doc = "        \"pattern\": \"^[a-zA-Z_$][a-zA-Z0-9_$]*$\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"string\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct EnumSchema {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "enum")]
    pub enum_: ::std::vec::Vec<EnumSchemaEnumItem>,
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<EnumSchemaType>,
}
impl ::std::convert::From<&EnumSchema> for EnumSchema {
    fn from(value: &EnumSchema) -> Self {
        value.clone()
    }
}
impl EnumSchema {
    pub fn builder() -> builder::EnumSchema {
        Default::default()
    }
}
#[doc = "`EnumSchemaEnumItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^[a-zA-Z_$][a-zA-Z0-9_$]*$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct EnumSchemaEnumItem(::std::string::String);
impl ::std::ops::Deref for EnumSchemaEnumItem {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<EnumSchemaEnumItem> for ::std::string::String {
    fn from(value: EnumSchemaEnumItem) -> Self {
        value.0
    }
}
impl ::std::convert::From<&EnumSchemaEnumItem> for EnumSchemaEnumItem {
    fn from(value: &EnumSchemaEnumItem) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for EnumSchemaEnumItem {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| {
                ::regress::Regex::new("^[a-zA-Z_$][a-zA-Z0-9_$]*$").unwrap()
            });
        if (&*PATTERN).find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z_$][a-zA-Z0-9_$]*$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for EnumSchemaEnumItem {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for EnumSchemaEnumItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EnumSchemaEnumItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for EnumSchemaEnumItem {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "`EnumSchemaType`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"string\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum EnumSchemaType {
    #[serde(rename = "string")]
    String,
}
impl ::std::convert::From<&Self> for EnumSchemaType {
    fn from(value: &EnumSchemaType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for EnumSchemaType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::String => write!(f, "string"),
        }
    }
}
impl ::std::str::FromStr for EnumSchemaType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "string" => Ok(Self::String),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EnumSchemaType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for EnumSchemaType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EnumSchemaType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`Export`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"codeSamples\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/CodeSample\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"input\": {"]
#[doc = "      \"$ref\": \"#/$defs/Parameter\""]
#[doc = "    },"]
#[doc = "    \"output\": {"]
#[doc = "      \"$ref\": \"#/$defs/Parameter\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Export {
    #[serde(
        rename = "codeSamples",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub code_samples: ::std::vec::Vec<CodeSample>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub input: ::std::option::Option<Parameter>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub output: ::std::option::Option<Parameter>,
}
impl ::std::convert::From<&Export> for Export {
    fn from(value: &Export) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for Export {
    fn default() -> Self {
        Self {
            code_samples: Default::default(),
            description: Default::default(),
            input: Default::default(),
            output: Default::default(),
        }
    }
}
impl Export {
    pub fn builder() -> builder::Export {
        Default::default()
    }
}
#[doc = "`Import`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"description\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"input\": {"]
#[doc = "      \"$ref\": \"#/$defs/Parameter\""]
#[doc = "    },"]
#[doc = "    \"output\": {"]
#[doc = "      \"$ref\": \"#/$defs/Parameter\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Import {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub input: ::std::option::Option<Parameter>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub output: ::std::option::Option<Parameter>,
}
impl ::std::convert::From<&Import> for Import {
    fn from(value: &Import) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for Import {
    fn default() -> Self {
        Self {
            description: Default::default(),
            input: Default::default(),
            output: Default::default(),
        }
    }
}
impl Import {
    pub fn builder() -> builder::Import {
        Default::default()
    }
}
#[doc = "`MapArrayItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"additionalProperties\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"additionalProperties\": {"]
#[doc = "      \"allOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/NonMapProperty\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"not\": {"]
#[doc = "            \"type\": \"object\","]
#[doc = "            \"required\": ["]
#[doc = "              \"description\""]
#[doc = "            ]"]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"const\": \"object\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct MapArrayItem {
    #[serde(rename = "additionalProperties")]
    pub additional_properties: NonMapProperty,
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<::serde_json::Value>,
}
impl ::std::convert::From<&MapArrayItem> for MapArrayItem {
    fn from(value: &MapArrayItem) -> Self {
        value.clone()
    }
}
impl MapArrayItem {
    pub fn builder() -> builder::MapArrayItem {
        Default::default()
    }
}
#[doc = "`MapParameter`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"additionalProperties\","]
#[doc = "    \"contentType\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"additionalProperties\": {"]
#[doc = "      \"allOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/NonMapProperty\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"properties\": {"]
#[doc = "            \"description\": false"]
#[doc = "          },"]
#[doc = "          \"additionalProperties\": false"]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"contentType\": {"]
#[doc = "      \"$ref\": \"#/$defs/ContentType\""]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"nullable\": {"]
#[doc = "      \"default\": false,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"const\": \"object\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct MapParameter {
    #[serde(rename = "additionalProperties")]
    pub additional_properties: MapParameterAdditionalProperties,
    #[serde(rename = "contentType")]
    pub content_type: ContentType,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(default)]
    pub nullable: bool,
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<::serde_json::Value>,
}
impl ::std::convert::From<&MapParameter> for MapParameter {
    fn from(value: &MapParameter) -> Self {
        value.clone()
    }
}
impl MapParameter {
    pub fn builder() -> builder::MapParameter {
        Default::default()
    }
}
#[doc = "`MapParameterAdditionalProperties`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"allOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/NonMapProperty\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"description\": false"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
#[serde(deny_unknown_fields)]
pub enum MapParameterAdditionalProperties {}
impl ::std::convert::From<&Self> for MapParameterAdditionalProperties {
    fn from(value: &MapParameterAdditionalProperties) -> Self {
        value.clone()
    }
}
#[doc = "`MapProperty`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"additionalProperties\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"additionalProperties\": {"]
#[doc = "      \"allOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/NonMapProperty\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"not\": {"]
#[doc = "            \"type\": \"object\","]
#[doc = "            \"required\": ["]
#[doc = "              \"description\""]
#[doc = "            ]"]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"nullable\": {"]
#[doc = "      \"default\": false,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"const\": \"object\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct MapProperty {
    #[serde(rename = "additionalProperties")]
    pub additional_properties: NonMapProperty,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(default)]
    pub nullable: bool,
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<::serde_json::Value>,
}
impl ::std::convert::From<&MapProperty> for MapProperty {
    fn from(value: &MapProperty) -> Self {
        value.clone()
    }
}
impl MapProperty {
    pub fn builder() -> builder::MapProperty {
        Default::default()
    }
}
#[doc = "`NonMapProperty`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/ValueProperty\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/RefProperty\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum NonMapProperty {
    ValueProperty(ValueProperty),
    RefProperty(RefProperty),
}
impl ::std::convert::From<&Self> for NonMapProperty {
    fn from(value: &NonMapProperty) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<ValueProperty> for NonMapProperty {
    fn from(value: ValueProperty) -> Self {
        Self::ValueProperty(value)
    }
}
impl ::std::convert::From<RefProperty> for NonMapProperty {
    fn from(value: RefProperty) -> Self {
        Self::RefProperty(value)
    }
}
#[doc = "`ObjectSchema`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"properties\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"description\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"properties\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"patternProperties\": {"]
#[doc = "        \"^[a-zA-Z_$][a-zA-Z0-9_$]*$\": {"]
#[doc = "          \"$ref\": \"#/$defs/Property\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    \"required\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ObjectSchema {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    pub properties: ::std::collections::HashMap<ObjectSchemaPropertiesKey, Property>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub required: ::std::vec::Vec<::std::string::String>,
}
impl ::std::convert::From<&ObjectSchema> for ObjectSchema {
    fn from(value: &ObjectSchema) -> Self {
        value.clone()
    }
}
impl ObjectSchema {
    pub fn builder() -> builder::ObjectSchema {
        Default::default()
    }
}
#[doc = "`ObjectSchemaPropertiesKey`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^[a-zA-Z_$][a-zA-Z0-9_$]*$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ObjectSchemaPropertiesKey(::std::string::String);
impl ::std::ops::Deref for ObjectSchemaPropertiesKey {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ObjectSchemaPropertiesKey> for ::std::string::String {
    fn from(value: ObjectSchemaPropertiesKey) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ObjectSchemaPropertiesKey> for ObjectSchemaPropertiesKey {
    fn from(value: &ObjectSchemaPropertiesKey) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for ObjectSchemaPropertiesKey {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| {
                ::regress::Regex::new("^[a-zA-Z_$][a-zA-Z0-9_$]*$").unwrap()
            });
        if (&*PATTERN).find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z_$][a-zA-Z0-9_$]*$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ObjectSchemaPropertiesKey {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ObjectSchemaPropertiesKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ObjectSchemaPropertiesKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ObjectSchemaPropertiesKey {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "`Parameter`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/ValueParameter\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/RefParameter\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/MapParameter\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum Parameter {
    ValueParameter(ValueParameter),
    RefParameter(RefParameter),
    MapParameter(MapParameter),
}
impl ::std::convert::From<&Self> for Parameter {
    fn from(value: &Parameter) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<ValueParameter> for Parameter {
    fn from(value: ValueParameter) -> Self {
        Self::ValueParameter(value)
    }
}
impl ::std::convert::From<RefParameter> for Parameter {
    fn from(value: RefParameter) -> Self {
        Self::RefParameter(value)
    }
}
impl ::std::convert::From<MapParameter> for Parameter {
    fn from(value: MapParameter) -> Self {
        Self::MapParameter(value)
    }
}
#[doc = "`Property`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/ValueProperty\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/RefProperty\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/MapProperty\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum Property {
    ValueProperty(ValueProperty),
    RefProperty(RefProperty),
    MapProperty(MapProperty),
}
impl ::std::convert::From<&Self> for Property {
    fn from(value: &Property) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<ValueProperty> for Property {
    fn from(value: ValueProperty) -> Self {
        Self::ValueProperty(value)
    }
}
impl ::std::convert::From<RefProperty> for Property {
    fn from(value: RefProperty) -> Self {
        Self::RefProperty(value)
    }
}
impl ::std::convert::From<MapProperty> for Property {
    fn from(value: MapProperty) -> Self {
        Self::MapProperty(value)
    }
}
#[doc = "`RefArrayItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"$ref\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"$ref\": {"]
#[doc = "      \"$ref\": \"#/$defs/SchemaReference\""]
#[doc = "    },"]
#[doc = "    \"nullable\": {"]
#[doc = "      \"default\": false,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct RefArrayItem {
    #[serde(default)]
    pub nullable: bool,
    #[serde(rename = "$ref")]
    pub ref_: SchemaReference,
}
impl ::std::convert::From<&RefArrayItem> for RefArrayItem {
    fn from(value: &RefArrayItem) -> Self {
        value.clone()
    }
}
impl RefArrayItem {
    pub fn builder() -> builder::RefArrayItem {
        Default::default()
    }
}
#[doc = "`RefParameter`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"$ref\","]
#[doc = "    \"contentType\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"$ref\": {"]
#[doc = "      \"$ref\": \"#/$defs/SchemaReference\""]
#[doc = "    },"]
#[doc = "    \"contentType\": {"]
#[doc = "      \"$ref\": \"#/$defs/ContentType\""]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"nullable\": {"]
#[doc = "      \"default\": false,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct RefParameter {
    #[serde(rename = "contentType")]
    pub content_type: ContentType,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(default)]
    pub nullable: bool,
    #[serde(rename = "$ref")]
    pub ref_: SchemaReference,
}
impl ::std::convert::From<&RefParameter> for RefParameter {
    fn from(value: &RefParameter) -> Self {
        value.clone()
    }
}
impl RefParameter {
    pub fn builder() -> builder::RefParameter {
        Default::default()
    }
}
#[doc = "`RefProperty`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"$ref\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"$ref\": {"]
#[doc = "      \"$ref\": \"#/$defs/SchemaReference\""]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"nullable\": {"]
#[doc = "      \"default\": false,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct RefProperty {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(default)]
    pub nullable: bool,
    #[serde(rename = "$ref")]
    pub ref_: SchemaReference,
}
impl ::std::convert::From<&RefProperty> for RefProperty {
    fn from(value: &RefProperty) -> Self {
        value.clone()
    }
}
impl RefProperty {
    pub fn builder() -> builder::RefProperty {
        Default::default()
    }
}
#[doc = "`Schema`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/ObjectSchema\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/EnumSchema\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum Schema {
    ObjectSchema(ObjectSchema),
    EnumSchema(EnumSchema),
}
impl ::std::convert::From<&Self> for Schema {
    fn from(value: &Schema) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<ObjectSchema> for Schema {
    fn from(value: ObjectSchema) -> Self {
        Self::ObjectSchema(value)
    }
}
impl ::std::convert::From<EnumSchema> for Schema {
    fn from(value: EnumSchema) -> Self {
        Self::EnumSchema(value)
    }
}
#[doc = "`SchemaReference`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^#/components/schemas/[^/]+$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct SchemaReference(::std::string::String);
impl ::std::ops::Deref for SchemaReference {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<SchemaReference> for ::std::string::String {
    fn from(value: SchemaReference) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SchemaReference> for SchemaReference {
    fn from(value: &SchemaReference) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for SchemaReference {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| {
                ::regress::Regex::new("^#/components/schemas/[^/]+$").unwrap()
            });
        if (&*PATTERN).find(value).is_none() {
            return Err("doesn't match pattern \"^#/components/schemas/[^/]+$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for SchemaReference {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for SchemaReference {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SchemaReference {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for SchemaReference {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "`ValueArrayItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"format\": {"]
#[doc = "      \"$ref\": \"#/$defs/XtpFormat\""]
#[doc = "    },"]
#[doc = "    \"nullable\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"$ref\": \"#/$defs/XtpType\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ValueArrayItem {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub format: ::std::option::Option<XtpFormat>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub nullable: ::std::option::Option<bool>,
    #[serde(rename = "type")]
    pub type_: XtpType,
}
impl ::std::convert::From<&ValueArrayItem> for ValueArrayItem {
    fn from(value: &ValueArrayItem) -> Self {
        value.clone()
    }
}
impl ValueArrayItem {
    pub fn builder() -> builder::ValueArrayItem {
        Default::default()
    }
}
#[doc = "`ValueParameter`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"contentType\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"contentType\": {"]
#[doc = "      \"$ref\": \"#/$defs/ContentType\""]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"format\": {"]
#[doc = "      \"$ref\": \"#/$defs/XtpFormat\""]
#[doc = "    },"]
#[doc = "    \"items\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"$ref\": \"#/$defs/ArrayItem\""]
#[doc = "    },"]
#[doc = "    \"nullable\": {"]
#[doc = "      \"default\": false,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"$ref\": \"#/$defs/XtpType\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ValueParameter {
    #[serde(rename = "contentType")]
    pub content_type: ContentType,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub format: ::std::option::Option<XtpFormat>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub items: ::std::option::Option<::std::boxed::Box<ArrayItem>>,
    #[serde(default)]
    pub nullable: bool,
    #[serde(rename = "type")]
    pub type_: XtpType,
}
impl ::std::convert::From<&ValueParameter> for ValueParameter {
    fn from(value: &ValueParameter) -> Self {
        value.clone()
    }
}
impl ValueParameter {
    pub fn builder() -> builder::ValueParameter {
        Default::default()
    }
}
#[doc = "`ValueProperty`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"description\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"format\": {"]
#[doc = "      \"$ref\": \"#/$defs/XtpFormat\""]
#[doc = "    },"]
#[doc = "    \"items\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"$ref\": \"#/$defs/ArrayItem\""]
#[doc = "    },"]
#[doc = "    \"nullable\": {"]
#[doc = "      \"default\": false,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"$ref\": \"#/$defs/XtpType\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ValueProperty {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub format: ::std::option::Option<XtpFormat>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub items: ::std::option::Option<::std::boxed::Box<ArrayItem>>,
    #[serde(default)]
    pub nullable: bool,
    #[serde(rename = "type")]
    pub type_: XtpType,
}
impl ::std::convert::From<&ValueProperty> for ValueProperty {
    fn from(value: &ValueProperty) -> Self {
        value.clone()
    }
}
impl ValueProperty {
    pub fn builder() -> builder::ValueProperty {
        Default::default()
    }
}
#[doc = "`XtpFormat`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"int32\","]
#[doc = "    \"int64\","]
#[doc = "    \"float\","]
#[doc = "    \"double\","]
#[doc = "    \"date-time\","]
#[doc = "    \"byte\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum XtpFormat {
    #[serde(rename = "int32")]
    Int32,
    #[serde(rename = "int64")]
    Int64,
    #[serde(rename = "float")]
    Float,
    #[serde(rename = "double")]
    Double,
    #[serde(rename = "date-time")]
    DateTime,
    #[serde(rename = "byte")]
    Byte,
}
impl ::std::convert::From<&Self> for XtpFormat {
    fn from(value: &XtpFormat) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for XtpFormat {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Int32 => write!(f, "int32"),
            Self::Int64 => write!(f, "int64"),
            Self::Float => write!(f, "float"),
            Self::Double => write!(f, "double"),
            Self::DateTime => write!(f, "date-time"),
            Self::Byte => write!(f, "byte"),
        }
    }
}
impl ::std::str::FromStr for XtpFormat {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "int32" => Ok(Self::Int32),
            "int64" => Ok(Self::Int64),
            "float" => Ok(Self::Float),
            "double" => Ok(Self::Double),
            "date-time" => Ok(Self::DateTime),
            "byte" => Ok(Self::Byte),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for XtpFormat {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for XtpFormat {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for XtpFormat {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`XtpType`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"integer\","]
#[doc = "    \"string\","]
#[doc = "    \"number\","]
#[doc = "    \"boolean\","]
#[doc = "    \"object\","]
#[doc = "    \"array\","]
#[doc = "    \"buffer\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum XtpType {
    #[serde(rename = "integer")]
    Integer,
    #[serde(rename = "string")]
    String,
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "object")]
    Object,
    #[serde(rename = "array")]
    Array,
    #[serde(rename = "buffer")]
    Buffer,
}
impl ::std::convert::From<&Self> for XtpType {
    fn from(value: &XtpType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for XtpType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Integer => write!(f, "integer"),
            Self::String => write!(f, "string"),
            Self::Number => write!(f, "number"),
            Self::Boolean => write!(f, "boolean"),
            Self::Object => write!(f, "object"),
            Self::Array => write!(f, "array"),
            Self::Buffer => write!(f, "buffer"),
        }
    }
}
impl ::std::str::FromStr for XtpType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "integer" => Ok(Self::Integer),
            "string" => Ok(Self::String),
            "number" => Ok(Self::Number),
            "boolean" => Ok(Self::Boolean),
            "object" => Ok(Self::Object),
            "array" => Ok(Self::Array),
            "buffer" => Ok(Self::Buffer),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for XtpType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for XtpType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for XtpType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`XtpVersion`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"v0\","]
#[doc = "    \"v1-draft\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum XtpVersion {
    #[serde(rename = "v0")]
    V0,
    #[serde(rename = "v1-draft")]
    V1Draft,
}
impl ::std::convert::From<&Self> for XtpVersion {
    fn from(value: &XtpVersion) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for XtpVersion {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::V0 => write!(f, "v0"),
            Self::V1Draft => write!(f, "v1-draft"),
        }
    }
}
impl ::std::str::FromStr for XtpVersion {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "v0" => Ok(Self::V0),
            "v1-draft" => Ok(Self::V1Draft),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for XtpVersion {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for XtpVersion {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for XtpVersion {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct CodeSample {
        label: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        lang: ::std::result::Result<super::CodeSampleLang, ::std::string::String>,
        source: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for CodeSample {
        fn default() -> Self {
            Self {
                label: Ok(Default::default()),
                lang: Err("no value supplied for lang".to_string()),
                source: Err("no value supplied for source".to_string()),
            }
        }
    }
    impl CodeSample {
        pub fn label<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.label = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for label: {}", e));
            self
        }
        pub fn lang<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CodeSampleLang>,
            T::Error: ::std::fmt::Display,
        {
            self.lang = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for lang: {}", e));
            self
        }
        pub fn source<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<CodeSample> for super::CodeSample {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CodeSample,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                label: value.label?,
                lang: value.lang?,
                source: value.source?,
            })
        }
    }
    impl ::std::convert::From<super::CodeSample> for CodeSample {
        fn from(value: super::CodeSample) -> Self {
            Self {
                label: Ok(value.label),
                lang: Ok(value.lang),
                source: Ok(value.source),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CodeSampleLang {
        subtype_0: ::std::result::Result<
            ::std::option::Option<super::CodeSampleLangSubtype0>,
            ::std::string::String,
        >,
        subtype_1: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CodeSampleLang {
        fn default() -> Self {
            Self {
                subtype_0: Ok(Default::default()),
                subtype_1: Ok(Default::default()),
            }
        }
    }
    impl CodeSampleLang {
        pub fn subtype_0<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CodeSampleLangSubtype0>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_0 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_0: {}", e));
            self
        }
        pub fn subtype_1<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_1 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_1: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<CodeSampleLang> for super::CodeSampleLang {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CodeSampleLang,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                subtype_0: value.subtype_0?,
                subtype_1: value.subtype_1?,
            })
        }
    }
    impl ::std::convert::From<super::CodeSampleLang> for CodeSampleLang {
        fn from(value: super::CodeSampleLang) -> Self {
            Self {
                subtype_0: Ok(value.subtype_0),
                subtype_1: Ok(value.subtype_1),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EnumSchema {
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        enum_: ::std::result::Result<
            ::std::vec::Vec<super::EnumSchemaEnumItem>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<
            ::std::option::Option<super::EnumSchemaType>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for EnumSchema {
        fn default() -> Self {
            Self {
                description: Ok(Default::default()),
                enum_: Err("no value supplied for enum_".to_string()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl EnumSchema {
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn enum_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::EnumSchemaEnumItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.enum_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for enum_: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::EnumSchemaType>>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<EnumSchema> for super::EnumSchema {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EnumSchema,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                enum_: value.enum_?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::EnumSchema> for EnumSchema {
        fn from(value: super::EnumSchema) -> Self {
            Self {
                description: Ok(value.description),
                enum_: Ok(value.enum_),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Export {
        code_samples:
            ::std::result::Result<::std::vec::Vec<super::CodeSample>, ::std::string::String>,
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        input:
            ::std::result::Result<::std::option::Option<super::Parameter>, ::std::string::String>,
        output:
            ::std::result::Result<::std::option::Option<super::Parameter>, ::std::string::String>,
    }
    impl ::std::default::Default for Export {
        fn default() -> Self {
            Self {
                code_samples: Ok(Default::default()),
                description: Ok(Default::default()),
                input: Ok(Default::default()),
                output: Ok(Default::default()),
            }
        }
    }
    impl Export {
        pub fn code_samples<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::CodeSample>>,
            T::Error: ::std::fmt::Display,
        {
            self.code_samples = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for code_samples: {}", e));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn input<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Parameter>>,
            T::Error: ::std::fmt::Display,
        {
            self.input = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for input: {}", e));
            self
        }
        pub fn output<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Parameter>>,
            T::Error: ::std::fmt::Display,
        {
            self.output = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for output: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<Export> for super::Export {
        type Error = super::error::ConversionError;
        fn try_from(value: Export) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                code_samples: value.code_samples?,
                description: value.description?,
                input: value.input?,
                output: value.output?,
            })
        }
    }
    impl ::std::convert::From<super::Export> for Export {
        fn from(value: super::Export) -> Self {
            Self {
                code_samples: Ok(value.code_samples),
                description: Ok(value.description),
                input: Ok(value.input),
                output: Ok(value.output),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Import {
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        input:
            ::std::result::Result<::std::option::Option<super::Parameter>, ::std::string::String>,
        output:
            ::std::result::Result<::std::option::Option<super::Parameter>, ::std::string::String>,
    }
    impl ::std::default::Default for Import {
        fn default() -> Self {
            Self {
                description: Ok(Default::default()),
                input: Ok(Default::default()),
                output: Ok(Default::default()),
            }
        }
    }
    impl Import {
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn input<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Parameter>>,
            T::Error: ::std::fmt::Display,
        {
            self.input = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for input: {}", e));
            self
        }
        pub fn output<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Parameter>>,
            T::Error: ::std::fmt::Display,
        {
            self.output = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for output: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<Import> for super::Import {
        type Error = super::error::ConversionError;
        fn try_from(value: Import) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                input: value.input?,
                output: value.output?,
            })
        }
    }
    impl ::std::convert::From<super::Import> for Import {
        fn from(value: super::Import) -> Self {
            Self {
                description: Ok(value.description),
                input: Ok(value.input),
                output: Ok(value.output),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MapArrayItem {
        additional_properties: ::std::result::Result<super::NonMapProperty, ::std::string::String>,
        type_: ::std::result::Result<
            ::std::option::Option<::serde_json::Value>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for MapArrayItem {
        fn default() -> Self {
            Self {
                additional_properties: Err(
                    "no value supplied for additional_properties".to_string()
                ),
                type_: Ok(Default::default()),
            }
        }
    }
    impl MapArrayItem {
        pub fn additional_properties<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::NonMapProperty>,
            T::Error: ::std::fmt::Display,
        {
            self.additional_properties = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for additional_properties: {}",
                    e
                )
            });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::serde_json::Value>>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<MapArrayItem> for super::MapArrayItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MapArrayItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                additional_properties: value.additional_properties?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::MapArrayItem> for MapArrayItem {
        fn from(value: super::MapArrayItem) -> Self {
            Self {
                additional_properties: Ok(value.additional_properties),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MapParameter {
        additional_properties:
            ::std::result::Result<super::MapParameterAdditionalProperties, ::std::string::String>,
        content_type: ::std::result::Result<super::ContentType, ::std::string::String>,
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        nullable: ::std::result::Result<bool, ::std::string::String>,
        type_: ::std::result::Result<
            ::std::option::Option<::serde_json::Value>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for MapParameter {
        fn default() -> Self {
            Self {
                additional_properties: Err(
                    "no value supplied for additional_properties".to_string()
                ),
                content_type: Err("no value supplied for content_type".to_string()),
                description: Ok(Default::default()),
                nullable: Ok(Default::default()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl MapParameter {
        pub fn additional_properties<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::MapParameterAdditionalProperties>,
            T::Error: ::std::fmt::Display,
        {
            self.additional_properties = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for additional_properties: {}",
                    e
                )
            });
            self
        }
        pub fn content_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ContentType>,
            T::Error: ::std::fmt::Display,
        {
            self.content_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for content_type: {}", e));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn nullable<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.nullable = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for nullable: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::serde_json::Value>>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<MapParameter> for super::MapParameter {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MapParameter,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                additional_properties: value.additional_properties?,
                content_type: value.content_type?,
                description: value.description?,
                nullable: value.nullable?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::MapParameter> for MapParameter {
        fn from(value: super::MapParameter) -> Self {
            Self {
                additional_properties: Ok(value.additional_properties),
                content_type: Ok(value.content_type),
                description: Ok(value.description),
                nullable: Ok(value.nullable),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MapProperty {
        additional_properties: ::std::result::Result<super::NonMapProperty, ::std::string::String>,
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        nullable: ::std::result::Result<bool, ::std::string::String>,
        type_: ::std::result::Result<
            ::std::option::Option<::serde_json::Value>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for MapProperty {
        fn default() -> Self {
            Self {
                additional_properties: Err(
                    "no value supplied for additional_properties".to_string()
                ),
                description: Ok(Default::default()),
                nullable: Ok(Default::default()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl MapProperty {
        pub fn additional_properties<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::NonMapProperty>,
            T::Error: ::std::fmt::Display,
        {
            self.additional_properties = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for additional_properties: {}",
                    e
                )
            });
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn nullable<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.nullable = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for nullable: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::serde_json::Value>>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<MapProperty> for super::MapProperty {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MapProperty,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                additional_properties: value.additional_properties?,
                description: value.description?,
                nullable: value.nullable?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::MapProperty> for MapProperty {
        fn from(value: super::MapProperty) -> Self {
            Self {
                additional_properties: Ok(value.additional_properties),
                description: Ok(value.description),
                nullable: Ok(value.nullable),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ObjectSchema {
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        properties: ::std::result::Result<
            ::std::collections::HashMap<super::ObjectSchemaPropertiesKey, super::Property>,
            ::std::string::String,
        >,
        required:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
    }
    impl ::std::default::Default for ObjectSchema {
        fn default() -> Self {
            Self {
                description: Ok(Default::default()),
                properties: Err("no value supplied for properties".to_string()),
                required: Ok(Default::default()),
            }
        }
    }
    impl ObjectSchema {
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn properties<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<super::ObjectSchemaPropertiesKey, super::Property>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.properties = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for properties: {}", e));
            self
        }
        pub fn required<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.required = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for required: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ObjectSchema> for super::ObjectSchema {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ObjectSchema,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                properties: value.properties?,
                required: value.required?,
            })
        }
    }
    impl ::std::convert::From<super::ObjectSchema> for ObjectSchema {
        fn from(value: super::ObjectSchema) -> Self {
            Self {
                description: Ok(value.description),
                properties: Ok(value.properties),
                required: Ok(value.required),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RefArrayItem {
        nullable: ::std::result::Result<bool, ::std::string::String>,
        ref_: ::std::result::Result<super::SchemaReference, ::std::string::String>,
    }
    impl ::std::default::Default for RefArrayItem {
        fn default() -> Self {
            Self {
                nullable: Ok(Default::default()),
                ref_: Err("no value supplied for ref_".to_string()),
            }
        }
    }
    impl RefArrayItem {
        pub fn nullable<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.nullable = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for nullable: {}", e));
            self
        }
        pub fn ref_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SchemaReference>,
            T::Error: ::std::fmt::Display,
        {
            self.ref_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ref_: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<RefArrayItem> for super::RefArrayItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RefArrayItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                nullable: value.nullable?,
                ref_: value.ref_?,
            })
        }
    }
    impl ::std::convert::From<super::RefArrayItem> for RefArrayItem {
        fn from(value: super::RefArrayItem) -> Self {
            Self {
                nullable: Ok(value.nullable),
                ref_: Ok(value.ref_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RefParameter {
        content_type: ::std::result::Result<super::ContentType, ::std::string::String>,
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        nullable: ::std::result::Result<bool, ::std::string::String>,
        ref_: ::std::result::Result<super::SchemaReference, ::std::string::String>,
    }
    impl ::std::default::Default for RefParameter {
        fn default() -> Self {
            Self {
                content_type: Err("no value supplied for content_type".to_string()),
                description: Ok(Default::default()),
                nullable: Ok(Default::default()),
                ref_: Err("no value supplied for ref_".to_string()),
            }
        }
    }
    impl RefParameter {
        pub fn content_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ContentType>,
            T::Error: ::std::fmt::Display,
        {
            self.content_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for content_type: {}", e));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn nullable<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.nullable = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for nullable: {}", e));
            self
        }
        pub fn ref_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SchemaReference>,
            T::Error: ::std::fmt::Display,
        {
            self.ref_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ref_: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<RefParameter> for super::RefParameter {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RefParameter,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                content_type: value.content_type?,
                description: value.description?,
                nullable: value.nullable?,
                ref_: value.ref_?,
            })
        }
    }
    impl ::std::convert::From<super::RefParameter> for RefParameter {
        fn from(value: super::RefParameter) -> Self {
            Self {
                content_type: Ok(value.content_type),
                description: Ok(value.description),
                nullable: Ok(value.nullable),
                ref_: Ok(value.ref_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RefProperty {
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        nullable: ::std::result::Result<bool, ::std::string::String>,
        ref_: ::std::result::Result<super::SchemaReference, ::std::string::String>,
    }
    impl ::std::default::Default for RefProperty {
        fn default() -> Self {
            Self {
                description: Ok(Default::default()),
                nullable: Ok(Default::default()),
                ref_: Err("no value supplied for ref_".to_string()),
            }
        }
    }
    impl RefProperty {
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn nullable<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.nullable = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for nullable: {}", e));
            self
        }
        pub fn ref_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SchemaReference>,
            T::Error: ::std::fmt::Display,
        {
            self.ref_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ref_: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<RefProperty> for super::RefProperty {
        type Error = super::error::ConversionError;
        fn try_from(
            value: RefProperty,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                nullable: value.nullable?,
                ref_: value.ref_?,
            })
        }
    }
    impl ::std::convert::From<super::RefProperty> for RefProperty {
        fn from(value: super::RefProperty) -> Self {
            Self {
                description: Ok(value.description),
                nullable: Ok(value.nullable),
                ref_: Ok(value.ref_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ValueArrayItem {
        format:
            ::std::result::Result<::std::option::Option<super::XtpFormat>, ::std::string::String>,
        nullable: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        type_: ::std::result::Result<super::XtpType, ::std::string::String>,
    }
    impl ::std::default::Default for ValueArrayItem {
        fn default() -> Self {
            Self {
                format: Ok(Default::default()),
                nullable: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl ValueArrayItem {
        pub fn format<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::XtpFormat>>,
            T::Error: ::std::fmt::Display,
        {
            self.format = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for format: {}", e));
            self
        }
        pub fn nullable<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.nullable = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for nullable: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::XtpType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ValueArrayItem> for super::ValueArrayItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ValueArrayItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                format: value.format?,
                nullable: value.nullable?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::ValueArrayItem> for ValueArrayItem {
        fn from(value: super::ValueArrayItem) -> Self {
            Self {
                format: Ok(value.format),
                nullable: Ok(value.nullable),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ValueParameter {
        content_type: ::std::result::Result<super::ContentType, ::std::string::String>,
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        format:
            ::std::result::Result<::std::option::Option<super::XtpFormat>, ::std::string::String>,
        items: ::std::result::Result<
            ::std::option::Option<::std::boxed::Box<super::ArrayItem>>,
            ::std::string::String,
        >,
        nullable: ::std::result::Result<bool, ::std::string::String>,
        type_: ::std::result::Result<super::XtpType, ::std::string::String>,
    }
    impl ::std::default::Default for ValueParameter {
        fn default() -> Self {
            Self {
                content_type: Err("no value supplied for content_type".to_string()),
                description: Ok(Default::default()),
                format: Ok(Default::default()),
                items: Ok(Default::default()),
                nullable: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl ValueParameter {
        pub fn content_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ContentType>,
            T::Error: ::std::fmt::Display,
        {
            self.content_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for content_type: {}", e));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn format<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::XtpFormat>>,
            T::Error: ::std::fmt::Display,
        {
            self.format = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for format: {}", e));
            self
        }
        pub fn items<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::boxed::Box<super::ArrayItem>>>,
            T::Error: ::std::fmt::Display,
        {
            self.items = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for items: {}", e));
            self
        }
        pub fn nullable<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.nullable = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for nullable: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::XtpType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ValueParameter> for super::ValueParameter {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ValueParameter,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                content_type: value.content_type?,
                description: value.description?,
                format: value.format?,
                items: value.items?,
                nullable: value.nullable?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::ValueParameter> for ValueParameter {
        fn from(value: super::ValueParameter) -> Self {
            Self {
                content_type: Ok(value.content_type),
                description: Ok(value.description),
                format: Ok(value.format),
                items: Ok(value.items),
                nullable: Ok(value.nullable),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ValueProperty {
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        format:
            ::std::result::Result<::std::option::Option<super::XtpFormat>, ::std::string::String>,
        items: ::std::result::Result<
            ::std::option::Option<::std::boxed::Box<super::ArrayItem>>,
            ::std::string::String,
        >,
        nullable: ::std::result::Result<bool, ::std::string::String>,
        type_: ::std::result::Result<super::XtpType, ::std::string::String>,
    }
    impl ::std::default::Default for ValueProperty {
        fn default() -> Self {
            Self {
                description: Ok(Default::default()),
                format: Ok(Default::default()),
                items: Ok(Default::default()),
                nullable: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl ValueProperty {
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn format<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::XtpFormat>>,
            T::Error: ::std::fmt::Display,
        {
            self.format = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for format: {}", e));
            self
        }
        pub fn items<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::boxed::Box<super::ArrayItem>>>,
            T::Error: ::std::fmt::Display,
        {
            self.items = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for items: {}", e));
            self
        }
        pub fn nullable<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.nullable = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for nullable: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::XtpType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ValueProperty> for super::ValueProperty {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ValueProperty,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                format: value.format?,
                items: value.items?,
                nullable: value.nullable?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::ValueProperty> for ValueProperty {
        fn from(value: super::ValueProperty) -> Self {
            Self {
                description: Ok(value.description),
                format: Ok(value.format),
                items: Ok(value.items),
                nullable: Ok(value.nullable),
                type_: Ok(value.type_),
            }
        }
    }
}
