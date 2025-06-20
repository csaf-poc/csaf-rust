
#![allow(clippy::clone_on_copy)]
#![allow(clippy::derivable_impls)]
#![allow(clippy::len_zero)]
/// Error types.
pub mod error {
    /// Error from a TryFrom or FromStr implementation.
    pub struct ConversionError(::std::borrow::Cow<'static, str>);
    impl ::std::error::Error for ConversionError {}
    impl ::std::fmt::Display for ConversionError {
        fn fmt(
            &self,
            f: &mut ::std::fmt::Formatter<'_>,
        ) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl ::std::fmt::Debug for ConversionError {
        fn fmt(
            &self,
            f: &mut ::std::fmt::Formatter<'_>,
        ) -> Result<(), ::std::fmt::Error> {
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
///Identifier for the vulnerability that was evaluation, such as CVE, CERT/CC VU#, OSV id, Bugtraq, GHSA etc.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Identifier for the vulnerability that was evaluation, such as CVE, CERT/CC VU#, OSV id, Bugtraq, GHSA etc.",
///  "examples": [
///    "CVE-1900-1234",
///    "VU#11111",
///    "GHSA-11a1-22b2-33c3"
///  ],
///  "type": "string",
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct Id(::std::string::String);
impl ::std::ops::Deref for Id {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Id> for ::std::string::String {
    fn from(value: Id) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Id> for Id {
    fn from(value: &Id) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for Id {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for Id {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for Id {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Id {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for Id {
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
///The role of the stakeholder performing the evaluation (e.g., Supplier, Deployer, Coordinator). See SSVC documentation for a currently identified list: https://certcc.github.io/SSVC/topics/enumerating_stakeholders/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "The role of the stakeholder performing the evaluation (e.g., Supplier, Deployer, Coordinator). See SSVC documentation for a currently identified list: https://certcc.github.io/SSVC/topics/enumerating_stakeholders/",
///  "examples": [
///    "Supplier",
///    "Deployer",
///    "Coordinator"
///  ],
///  "type": "string",
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct Role(::std::string::String);
impl ::std::ops::Deref for Role {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Role> for ::std::string::String {
    fn from(value: Role) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Role> for Role {
    fn from(value: &Role) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for Role {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for Role {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for Role {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Role {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for Role {
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
///Schema version used to represent this Decision Point.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Schema version used to represent this Decision Point.",
///  "type": "string",
///  "enum": [
///    "1-0-1"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum SchemaVersion {
    #[serde(rename = "1-0-1")]
    _101,
}
impl ::std::convert::From<&Self> for SchemaVersion {
    fn from(value: &SchemaVersion) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for SchemaVersion {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::_101 => write!(f, "1-0-1"),
        }
    }
}
impl ::std::str::FromStr for SchemaVersion {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "1-0-1" => Ok(Self::_101),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SchemaVersion {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for SchemaVersion {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SchemaVersion {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///This schema defines the structure for selecting SSVC Decision Points and their evaluated values for a given vulnerability. Each vulnerability can have multiple Decision Points, and each Decision Point can have multiple selected values when full certainty is not available.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "$id": "https://certcc.github.io/SSVC/data/schema/v1/Decision_Point_Value_Selection-1-0-1.schema.json",
///  "title": "SSVC_v1",
///  "description": "This schema defines the structure for selecting SSVC Decision Points and their evaluated values for a given vulnerability. Each vulnerability can have multiple Decision Points, and each Decision Point can have multiple selected values when full certainty is not available.",
///  "type": "object",
///  "required": [
///    "id",
///    "schemaVersion",
///    "selections",
///    "timestamp"
///  ],
///  "properties": {
///    "id": {
///      "$ref": "#/$defs/id"
///    },
///    "role": {
///      "$ref": "#/$defs/role"
///    },
///    "schemaVersion": {
///      "$ref": "#/$defs/schemaVersion"
///    },
///    "selections": {
///      "title": "selections",
///      "description": "An array of Decision Points and their selected values for the identified Vulnerability.  If a clear evaluation is uncertain, multiple values may be listed for a Decision Point instead of waiting for perfect clarity.",
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/SsvcdecisionpointselectionSchema"
///      },
///      "minItems": 1
///    },
///    "timestamp": {
///      "$ref": "#/$defs/timestamp"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SsvcV1 {
    pub id: Id,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub role: ::std::option::Option<Role>,
    #[serde(rename = "schemaVersion")]
    pub schema_version: SchemaVersion,
    ///An array of Decision Points and their selected values for the identified Vulnerability.  If a clear evaluation is uncertain, multiple values may be listed for a Decision Point instead of waiting for perfect clarity.
    pub selections: ::std::vec::Vec<SsvcdecisionpointselectionSchema>,
    pub timestamp: Timestamp,
}
impl ::std::convert::From<&SsvcV1> for SsvcV1 {
    fn from(value: &SsvcV1) -> Self {
        value.clone()
    }
}
impl SsvcV1 {
    pub fn builder() -> builder::SsvcV1 {
        Default::default()
    }
}
///A down-selection of SSVC Decision Points that represent an evaluation at a specific time of a Vulnerability evaluation.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "A down-selection of SSVC Decision Points that represent an evaluation at a specific time of a Vulnerability evaluation.",
///  "type": "object",
///  "required": [
///    "name",
///    "namespace",
///    "values",
///    "version"
///  ],
///  "properties": {
///    "name": {
///      "description": "A short label that identifies a Decision Point.",
///      "examples": [
///        "Exploitation",
///        "Automatable"
///      ],
///      "type": "string",
///      "minLength": 1
///    },
///    "namespace": {
///      "description": "Namespace (a short, unique string): The value must be one of the official namespaces, currenlty \"ssvc\", \"cvss\" OR can start with 'x_' for private namespaces. See SSVC Documentation for details.",
///      "examples": [
///        "ssvc",
///        "cvss",
///        "x_custom",
///        "x_custom/extension"
///      ],
///      "type": "string",
///      "pattern": "^(?=.{3,100}$)(x_)?[a-z0-9]{3}([/.-]?[a-z0-9]+){0,97}$"
///    },
///    "values": {
///      "title": "values",
///      "description": "One or more Decision Point Values that were selected for this Decision Point. If the evaluation is uncertain, multiple values may be listed to reflect the potential range of possibilities.",
///      "type": "array",
///      "items": {
///        "description": "A short label that identifies a Decision Point Value",
///        "examples": [
///          "Public PoC",
///          "Yes"
///        ],
///        "type": "string",
///        "minLength": 1
///      },
///      "minItems": 1
///    },
///    "version": {
///      "description": "Version (a semantic version string) that identifies the version of a Decision Point.",
///      "examples": [
///        "1.0.1",
///        "1.0.1-alpha"
///      ],
///      "type": "string",
///      "pattern": "^(0|[1-9]\\d*)\\.(0|[1-9]\\d*)\\.(0|[1-9]\\d*)(?:-((?:0|[1-9]\\d*|\\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\\.(?:0|[1-9]\\d*|\\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\\+([0-9a-zA-Z-]+(?:\\.[0-9a-zA-Z-]+)*))?$"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SsvcdecisionpointselectionSchema {
    ///A short label that identifies a Decision Point.
    pub name: SsvcdecisionpointselectionSchemaName,
    ///Namespace (a short, unique string): The value must be one of the official namespaces, currenlty "ssvc", "cvss" OR can start with 'x_' for private namespaces. See SSVC Documentation for details.
    pub namespace: SsvcdecisionpointselectionSchemaNamespace,
    ///One or more Decision Point Values that were selected for this Decision Point. If the evaluation is uncertain, multiple values may be listed to reflect the potential range of possibilities.
    pub values: ::std::vec::Vec<ValuesItem>,
    ///Version (a semantic version string) that identifies the version of a Decision Point.
    pub version: SsvcdecisionpointselectionSchemaVersion,
}
impl ::std::convert::From<&SsvcdecisionpointselectionSchema>
for SsvcdecisionpointselectionSchema {
    fn from(value: &SsvcdecisionpointselectionSchema) -> Self {
        value.clone()
    }
}
impl SsvcdecisionpointselectionSchema {
    pub fn builder() -> builder::SsvcdecisionpointselectionSchema {
        Default::default()
    }
}
///A short label that identifies a Decision Point.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "A short label that identifies a Decision Point.",
///  "examples": [
///    "Exploitation",
///    "Automatable"
///  ],
///  "type": "string",
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct SsvcdecisionpointselectionSchemaName(::std::string::String);
impl ::std::ops::Deref for SsvcdecisionpointselectionSchemaName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<SsvcdecisionpointselectionSchemaName>
for ::std::string::String {
    fn from(value: SsvcdecisionpointselectionSchemaName) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SsvcdecisionpointselectionSchemaName>
for SsvcdecisionpointselectionSchemaName {
    fn from(value: &SsvcdecisionpointselectionSchemaName) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for SsvcdecisionpointselectionSchemaName {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for SsvcdecisionpointselectionSchemaName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for SsvcdecisionpointselectionSchemaName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for SsvcdecisionpointselectionSchemaName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for SsvcdecisionpointselectionSchemaName {
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
///Namespace (a short, unique string): The value must be one of the official namespaces, currenlty "ssvc", "cvss" OR can start with 'x_' for private namespaces. See SSVC Documentation for details.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Namespace (a short, unique string): The value must be one of the official namespaces, currenlty \"ssvc\", \"cvss\" OR can start with 'x_' for private namespaces. See SSVC Documentation for details.",
///  "examples": [
///    "ssvc",
///    "cvss",
///    "x_custom",
///    "x_custom/extension"
///  ],
///  "type": "string",
///  "pattern": "^(?=.{3,100}$)(x_)?[a-z0-9]{3}([/.-]?[a-z0-9]+){0,97}$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct SsvcdecisionpointselectionSchemaNamespace(::std::string::String);
impl ::std::ops::Deref for SsvcdecisionpointselectionSchemaNamespace {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<SsvcdecisionpointselectionSchemaNamespace>
for ::std::string::String {
    fn from(value: SsvcdecisionpointselectionSchemaNamespace) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SsvcdecisionpointselectionSchemaNamespace>
for SsvcdecisionpointselectionSchemaNamespace {
    fn from(value: &SsvcdecisionpointselectionSchemaNamespace) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for SsvcdecisionpointselectionSchemaNamespace {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if regress::Regex::new("^(?=.{3,100}$)(x_)?[a-z0-9]{3}([/.-]?[a-z0-9]+){0,97}$")
            .unwrap()
            .find(value)
            .is_none()
        {
            return Err(
                "doesn't match pattern \"^(?=.{3,100}$)(x_)?[a-z0-9]{3}([/.-]?[a-z0-9]+){0,97}$\""
                    .into(),
            );
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for SsvcdecisionpointselectionSchemaNamespace {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for SsvcdecisionpointselectionSchemaNamespace {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for SsvcdecisionpointselectionSchemaNamespace {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for SsvcdecisionpointselectionSchemaNamespace {
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
///Version (a semantic version string) that identifies the version of a Decision Point.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Version (a semantic version string) that identifies the version of a Decision Point.",
///  "examples": [
///    "1.0.1",
///    "1.0.1-alpha"
///  ],
///  "type": "string",
///  "pattern": "^(0|[1-9]\\d*)\\.(0|[1-9]\\d*)\\.(0|[1-9]\\d*)(?:-((?:0|[1-9]\\d*|\\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\\.(?:0|[1-9]\\d*|\\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\\+([0-9a-zA-Z-]+(?:\\.[0-9a-zA-Z-]+)*))?$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct SsvcdecisionpointselectionSchemaVersion(::std::string::String);
impl ::std::ops::Deref for SsvcdecisionpointselectionSchemaVersion {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<SsvcdecisionpointselectionSchemaVersion>
for ::std::string::String {
    fn from(value: SsvcdecisionpointselectionSchemaVersion) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SsvcdecisionpointselectionSchemaVersion>
for SsvcdecisionpointselectionSchemaVersion {
    fn from(value: &SsvcdecisionpointselectionSchemaVersion) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for SsvcdecisionpointselectionSchemaVersion {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if regress::Regex::new(
                "^(0|[1-9]\\d*)\\.(0|[1-9]\\d*)\\.(0|[1-9]\\d*)(?:-((?:0|[1-9]\\d*|\\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\\.(?:0|[1-9]\\d*|\\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\\+([0-9a-zA-Z-]+(?:\\.[0-9a-zA-Z-]+)*))?$",
            )
            .unwrap()
            .find(value)
            .is_none()
        {
            return Err(
                "doesn't match pattern \"^(0|[1-9]\\d*)\\.(0|[1-9]\\d*)\\.(0|[1-9]\\d*)(?:-((?:0|[1-9]\\d*|\\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\\.(?:0|[1-9]\\d*|\\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\\+([0-9a-zA-Z-]+(?:\\.[0-9a-zA-Z-]+)*))?$\""
                    .into(),
            );
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for SsvcdecisionpointselectionSchemaVersion {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for SsvcdecisionpointselectionSchemaVersion {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for SsvcdecisionpointselectionSchemaVersion {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for SsvcdecisionpointselectionSchemaVersion {
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
///Date and time when the evaluation of the Vulnerability was performed according to RFC 3339, section 5.6.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Date and time when the evaluation of the Vulnerability was performed according to RFC 3339, section 5.6.",
///  "type": "string",
///  "format": "date-time"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, Eq, PartialEq)]
#[serde(transparent)]
pub struct Timestamp(pub chrono::DateTime<chrono::offset::Utc>);
impl ::std::ops::Deref for Timestamp {
    type Target = chrono::DateTime<chrono::offset::Utc>;
    fn deref(&self) -> &chrono::DateTime<chrono::offset::Utc> {
        &self.0
    }
}
impl ::std::convert::From<Timestamp> for chrono::DateTime<chrono::offset::Utc> {
    fn from(value: Timestamp) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Timestamp> for Timestamp {
    fn from(value: &Timestamp) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<chrono::DateTime<chrono::offset::Utc>> for Timestamp {
    fn from(value: chrono::DateTime<chrono::offset::Utc>) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for Timestamp {
    type Err = <chrono::DateTime<chrono::offset::Utc> as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for Timestamp {
    type Error = <chrono::DateTime<chrono::offset::Utc> as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&String> for Timestamp {
    type Error = <chrono::DateTime<chrono::offset::Utc> as ::std::str::FromStr>::Err;
    fn try_from(value: &String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for Timestamp {
    type Error = <chrono::DateTime<chrono::offset::Utc> as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::fmt::Display for Timestamp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
///A short label that identifies a Decision Point Value
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "A short label that identifies a Decision Point Value",
///  "examples": [
///    "Public PoC",
///    "Yes"
///  ],
///  "type": "string",
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ValuesItem(::std::string::String);
impl ::std::ops::Deref for ValuesItem {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ValuesItem> for ::std::string::String {
    fn from(value: ValuesItem) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ValuesItem> for ValuesItem {
    fn from(value: &ValuesItem) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for ValuesItem {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ValuesItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ValuesItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ValuesItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ValuesItem {
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
/// Types for composing complex structures.
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct SsvcV1 {
        id: ::std::result::Result<super::Id, ::std::string::String>,
        role: ::std::result::Result<
            ::std::option::Option<super::Role>,
            ::std::string::String,
        >,
        schema_version: ::std::result::Result<
            super::SchemaVersion,
            ::std::string::String,
        >,
        selections: ::std::result::Result<
            ::std::vec::Vec<super::SsvcdecisionpointselectionSchema>,
            ::std::string::String,
        >,
        timestamp: ::std::result::Result<super::Timestamp, ::std::string::String>,
    }
    impl ::std::default::Default for SsvcV1 {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                role: Ok(Default::default()),
                schema_version: Err("no value supplied for schema_version".to_string()),
                selections: Err("no value supplied for selections".to_string()),
                timestamp: Err("no value supplied for timestamp".to_string()),
            }
        }
    }
    impl SsvcV1 {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Id>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn role<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Role>>,
            T::Error: ::std::fmt::Display,
        {
            self.role = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for role: {}", e));
            self
        }
        pub fn schema_version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SchemaVersion>,
            T::Error: ::std::fmt::Display,
        {
            self.schema_version = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for schema_version: {}", e)
                });
            self
        }
        pub fn selections<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::SsvcdecisionpointselectionSchema>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.selections = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for selections: {}", e)
                });
            self
        }
        pub fn timestamp<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Timestamp>,
            T::Error: ::std::fmt::Display,
        {
            self.timestamp = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for timestamp: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<SsvcV1> for super::SsvcV1 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SsvcV1,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                role: value.role?,
                schema_version: value.schema_version?,
                selections: value.selections?,
                timestamp: value.timestamp?,
            })
        }
    }
    impl ::std::convert::From<super::SsvcV1> for SsvcV1 {
        fn from(value: super::SsvcV1) -> Self {
            Self {
                id: Ok(value.id),
                role: Ok(value.role),
                schema_version: Ok(value.schema_version),
                selections: Ok(value.selections),
                timestamp: Ok(value.timestamp),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SsvcdecisionpointselectionSchema {
        name: ::std::result::Result<
            super::SsvcdecisionpointselectionSchemaName,
            ::std::string::String,
        >,
        namespace: ::std::result::Result<
            super::SsvcdecisionpointselectionSchemaNamespace,
            ::std::string::String,
        >,
        values: ::std::result::Result<
            ::std::vec::Vec<super::ValuesItem>,
            ::std::string::String,
        >,
        version: ::std::result::Result<
            super::SsvcdecisionpointselectionSchemaVersion,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for SsvcdecisionpointselectionSchema {
        fn default() -> Self {
            Self {
                name: Err("no value supplied for name".to_string()),
                namespace: Err("no value supplied for namespace".to_string()),
                values: Err("no value supplied for values".to_string()),
                version: Err("no value supplied for version".to_string()),
            }
        }
    }
    impl SsvcdecisionpointselectionSchema {
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SsvcdecisionpointselectionSchemaName>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn namespace<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SsvcdecisionpointselectionSchemaNamespace>,
            T::Error: ::std::fmt::Display,
        {
            self.namespace = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for namespace: {}", e)
                });
            self
        }
        pub fn values<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ValuesItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.values = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for values: {}", e)
                });
            self
        }
        pub fn version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SsvcdecisionpointselectionSchemaVersion>,
            T::Error: ::std::fmt::Display,
        {
            self.version = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for version: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<SsvcdecisionpointselectionSchema>
    for super::SsvcdecisionpointselectionSchema {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SsvcdecisionpointselectionSchema,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                name: value.name?,
                namespace: value.namespace?,
                values: value.values?,
                version: value.version?,
            })
        }
    }
    impl ::std::convert::From<super::SsvcdecisionpointselectionSchema>
    for SsvcdecisionpointselectionSchema {
        fn from(value: super::SsvcdecisionpointselectionSchema) -> Self {
            Self {
                name: Ok(value.name),
                namespace: Ok(value.namespace),
                values: Ok(value.values),
                version: Ok(value.version),
            }
        }
    }
}
