
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
///DecisionPoint
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "description",
///    "key",
///    "name",
///    "namespace",
///    "schemaVersion",
///    "values",
///    "version"
///  ],
///  "properties": {
///    "description": {
///      "description": "A full description of the Decision Point, explaining what it represents and how it is used in SSVC.",
///      "type": "string",
///      "minLength": 1
///    },
///    "key": {
///      "description": "A short, unique string (or key) used as a shorthand identifier for a Decision Point.",
///      "examples": [
///        "E",
///        "A"
///      ],
///      "type": "string",
///      "minLength": 1
///    },
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
///    "schemaVersion": {
///      "$ref": "#/$defs/schemaVersion"
///    },
///    "values": {
///      "description": "A set of possible answers for a given Decision Point",
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/decision_point_value"
///      },
///      "minItems": 1,
///      "uniqueItems": true
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
pub struct DecisionPoint {
    ///A full description of the Decision Point, explaining what it represents and how it is used in SSVC.
    pub description: DecisionPointDescription,
    ///A short, unique string (or key) used as a shorthand identifier for a Decision Point.
    pub key: DecisionPointKey,
    ///A short label that identifies a Decision Point.
    pub name: DecisionPointName,
    ///Namespace (a short, unique string): The value must be one of the official namespaces, currenlty "ssvc", "cvss" OR can start with 'x_' for private namespaces. See SSVC Documentation for details.
    pub namespace: DecisionPointNamespace,
    #[serde(rename = "schemaVersion")]
    pub schema_version: SchemaVersion,
    ///A set of possible answers for a given Decision Point
    pub values: Vec<DecisionPointValue>,
    ///Version (a semantic version string) that identifies the version of a Decision Point.
    pub version: DecisionPointVersion,
}
impl ::std::convert::From<&DecisionPoint> for DecisionPoint {
    fn from(value: &DecisionPoint) -> Self {
        value.clone()
    }
}
impl DecisionPoint {
    pub fn builder() -> builder::DecisionPoint {
        Default::default()
    }
}
///A full description of the Decision Point, explaining what it represents and how it is used in SSVC.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "A full description of the Decision Point, explaining what it represents and how it is used in SSVC.",
///  "type": "string",
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct DecisionPointDescription(::std::string::String);
impl ::std::ops::Deref for DecisionPointDescription {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<DecisionPointDescription> for ::std::string::String {
    fn from(value: DecisionPointDescription) -> Self {
        value.0
    }
}
impl ::std::convert::From<&DecisionPointDescription> for DecisionPointDescription {
    fn from(value: &DecisionPointDescription) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for DecisionPointDescription {
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
impl ::std::convert::TryFrom<&str> for DecisionPointDescription {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for DecisionPointDescription {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DecisionPointDescription {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for DecisionPointDescription {
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
///A short, unique string (or key) used as a shorthand identifier for a Decision Point.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "A short, unique string (or key) used as a shorthand identifier for a Decision Point.",
///  "examples": [
///    "E",
///    "A"
///  ],
///  "type": "string",
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct DecisionPointKey(::std::string::String);
impl ::std::ops::Deref for DecisionPointKey {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<DecisionPointKey> for ::std::string::String {
    fn from(value: DecisionPointKey) -> Self {
        value.0
    }
}
impl ::std::convert::From<&DecisionPointKey> for DecisionPointKey {
    fn from(value: &DecisionPointKey) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for DecisionPointKey {
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
impl ::std::convert::TryFrom<&str> for DecisionPointKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for DecisionPointKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DecisionPointKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for DecisionPointKey {
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
pub struct DecisionPointName(::std::string::String);
impl ::std::ops::Deref for DecisionPointName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<DecisionPointName> for ::std::string::String {
    fn from(value: DecisionPointName) -> Self {
        value.0
    }
}
impl ::std::convert::From<&DecisionPointName> for DecisionPointName {
    fn from(value: &DecisionPointName) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for DecisionPointName {
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
impl ::std::convert::TryFrom<&str> for DecisionPointName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for DecisionPointName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DecisionPointName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for DecisionPointName {
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
pub struct DecisionPointNamespace(::std::string::String);
impl ::std::ops::Deref for DecisionPointNamespace {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<DecisionPointNamespace> for ::std::string::String {
    fn from(value: DecisionPointNamespace) -> Self {
        value.0
    }
}
impl ::std::convert::From<&DecisionPointNamespace> for DecisionPointNamespace {
    fn from(value: &DecisionPointNamespace) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for DecisionPointNamespace {
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
impl ::std::convert::TryFrom<&str> for DecisionPointNamespace {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for DecisionPointNamespace {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DecisionPointNamespace {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for DecisionPointNamespace {
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
///Decision points are the basic building blocks of SSVC decision functions. Individual decision points describe a single aspect of the input to a decision function.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "$id": "https://certcc.github.io/SSVC/data/schema/v1/Decision_Point-1-0-1.schema.json",
///  "title": "Decision Point schema definition",
///  "description": "Decision points are the basic building blocks of SSVC decision functions. Individual decision points describe a single aspect of the input to a decision function.",
///  "$ref": "#/$defs/decision_point"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, Eq, PartialEq)]
#[serde(transparent)]
pub struct DecisionPointSchemaDefinition(pub DecisionPoint);
impl ::std::ops::Deref for DecisionPointSchemaDefinition {
    type Target = DecisionPoint;
    fn deref(&self) -> &DecisionPoint {
        &self.0
    }
}
impl ::std::convert::From<DecisionPointSchemaDefinition> for DecisionPoint {
    fn from(value: DecisionPointSchemaDefinition) -> Self {
        value.0
    }
}
impl ::std::convert::From<&DecisionPointSchemaDefinition>
for DecisionPointSchemaDefinition {
    fn from(value: &DecisionPointSchemaDefinition) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<DecisionPoint> for DecisionPointSchemaDefinition {
    fn from(value: DecisionPoint) -> Self {
        Self(value)
    }
}
///DecisionPointValue
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "description",
///    "key",
///    "name"
///  ],
///  "properties": {
///    "description": {
///      "description": "A full description of the Decision Point Value.",
///      "examples": [
///        "One of the following is true: (1) Typical public PoC exists in sources such as Metasploit or websites like ExploitDB; or (2) the vulnerability has a well-known method of exploitation.",
///        "Attackers can reliably automate steps 1-4 of the kill chain."
///      ],
///      "type": "string",
///      "minLength": 1
///    },
///    "key": {
///      "description": "A short, unique string (or key) used as a shorthand identifier for a Decision Point Value.",
///      "examples": [
///        "P",
///        "Y"
///      ],
///      "type": "string",
///      "minLength": 1
///    },
///    "name": {
///      "description": "A short label that identifies a Decision Point Value",
///      "examples": [
///        "Public PoC",
///        "Yes"
///      ],
///      "type": "string",
///      "minLength": 1
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct DecisionPointValue {
    ///A full description of the Decision Point Value.
    pub description: DecisionPointValueDescription,
    ///A short, unique string (or key) used as a shorthand identifier for a Decision Point Value.
    pub key: DecisionPointValueKey,
    ///A short label that identifies a Decision Point Value
    pub name: DecisionPointValueName,
}
impl ::std::convert::From<&DecisionPointValue> for DecisionPointValue {
    fn from(value: &DecisionPointValue) -> Self {
        value.clone()
    }
}
impl DecisionPointValue {
    pub fn builder() -> builder::DecisionPointValue {
        Default::default()
    }
}
///A full description of the Decision Point Value.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "A full description of the Decision Point Value.",
///  "examples": [
///    "One of the following is true: (1) Typical public PoC exists in sources such as Metasploit or websites like ExploitDB; or (2) the vulnerability has a well-known method of exploitation.",
///    "Attackers can reliably automate steps 1-4 of the kill chain."
///  ],
///  "type": "string",
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct DecisionPointValueDescription(::std::string::String);
impl ::std::ops::Deref for DecisionPointValueDescription {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<DecisionPointValueDescription> for ::std::string::String {
    fn from(value: DecisionPointValueDescription) -> Self {
        value.0
    }
}
impl ::std::convert::From<&DecisionPointValueDescription>
for DecisionPointValueDescription {
    fn from(value: &DecisionPointValueDescription) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for DecisionPointValueDescription {
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
impl ::std::convert::TryFrom<&str> for DecisionPointValueDescription {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for DecisionPointValueDescription {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DecisionPointValueDescription {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for DecisionPointValueDescription {
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
///A short, unique string (or key) used as a shorthand identifier for a Decision Point Value.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "A short, unique string (or key) used as a shorthand identifier for a Decision Point Value.",
///  "examples": [
///    "P",
///    "Y"
///  ],
///  "type": "string",
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct DecisionPointValueKey(::std::string::String);
impl ::std::ops::Deref for DecisionPointValueKey {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<DecisionPointValueKey> for ::std::string::String {
    fn from(value: DecisionPointValueKey) -> Self {
        value.0
    }
}
impl ::std::convert::From<&DecisionPointValueKey> for DecisionPointValueKey {
    fn from(value: &DecisionPointValueKey) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for DecisionPointValueKey {
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
impl ::std::convert::TryFrom<&str> for DecisionPointValueKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for DecisionPointValueKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DecisionPointValueKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for DecisionPointValueKey {
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
pub struct DecisionPointValueName(::std::string::String);
impl ::std::ops::Deref for DecisionPointValueName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<DecisionPointValueName> for ::std::string::String {
    fn from(value: DecisionPointValueName) -> Self {
        value.0
    }
}
impl ::std::convert::From<&DecisionPointValueName> for DecisionPointValueName {
    fn from(value: &DecisionPointValueName) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for DecisionPointValueName {
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
impl ::std::convert::TryFrom<&str> for DecisionPointValueName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for DecisionPointValueName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DecisionPointValueName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for DecisionPointValueName {
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
pub struct DecisionPointVersion(::std::string::String);
impl ::std::ops::Deref for DecisionPointVersion {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<DecisionPointVersion> for ::std::string::String {
    fn from(value: DecisionPointVersion) -> Self {
        value.0
    }
}
impl ::std::convert::From<&DecisionPointVersion> for DecisionPointVersion {
    fn from(value: &DecisionPointVersion) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for DecisionPointVersion {
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
impl ::std::convert::TryFrom<&str> for DecisionPointVersion {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for DecisionPointVersion {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DecisionPointVersion {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for DecisionPointVersion {
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
/// Types for composing complex structures.
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct DecisionPoint {
        description: ::std::result::Result<
            super::DecisionPointDescription,
            ::std::string::String,
        >,
        key: ::std::result::Result<super::DecisionPointKey, ::std::string::String>,
        name: ::std::result::Result<super::DecisionPointName, ::std::string::String>,
        namespace: ::std::result::Result<
            super::DecisionPointNamespace,
            ::std::string::String,
        >,
        schema_version: ::std::result::Result<
            super::SchemaVersion,
            ::std::string::String,
        >,
        values: ::std::result::Result<
            Vec<super::DecisionPointValue>,
            ::std::string::String,
        >,
        version: ::std::result::Result<
            super::DecisionPointVersion,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for DecisionPoint {
        fn default() -> Self {
            Self {
                description: Err("no value supplied for description".to_string()),
                key: Err("no value supplied for key".to_string()),
                name: Err("no value supplied for name".to_string()),
                namespace: Err("no value supplied for namespace".to_string()),
                schema_version: Err("no value supplied for schema_version".to_string()),
                values: Err("no value supplied for values".to_string()),
                version: Err("no value supplied for version".to_string()),
            }
        }
    }
    impl DecisionPoint {
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DecisionPointDescription>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for description: {}", e)
                });
            self
        }
        pub fn key<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DecisionPointKey>,
            T::Error: ::std::fmt::Display,
        {
            self.key = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for key: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DecisionPointName>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn namespace<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DecisionPointNamespace>,
            T::Error: ::std::fmt::Display,
        {
            self.namespace = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for namespace: {}", e)
                });
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
        pub fn values<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<Vec<super::DecisionPointValue>>,
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
            T: ::std::convert::TryInto<super::DecisionPointVersion>,
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
    impl ::std::convert::TryFrom<DecisionPoint> for super::DecisionPoint {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DecisionPoint,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                key: value.key?,
                name: value.name?,
                namespace: value.namespace?,
                schema_version: value.schema_version?,
                values: value.values?,
                version: value.version?,
            })
        }
    }
    impl ::std::convert::From<super::DecisionPoint> for DecisionPoint {
        fn from(value: super::DecisionPoint) -> Self {
            Self {
                description: Ok(value.description),
                key: Ok(value.key),
                name: Ok(value.name),
                namespace: Ok(value.namespace),
                schema_version: Ok(value.schema_version),
                values: Ok(value.values),
                version: Ok(value.version),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DecisionPointValue {
        description: ::std::result::Result<
            super::DecisionPointValueDescription,
            ::std::string::String,
        >,
        key: ::std::result::Result<super::DecisionPointValueKey, ::std::string::String>,
        name: ::std::result::Result<
            super::DecisionPointValueName,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for DecisionPointValue {
        fn default() -> Self {
            Self {
                description: Err("no value supplied for description".to_string()),
                key: Err("no value supplied for key".to_string()),
                name: Err("no value supplied for name".to_string()),
            }
        }
    }
    impl DecisionPointValue {
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DecisionPointValueDescription>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for description: {}", e)
                });
            self
        }
        pub fn key<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DecisionPointValueKey>,
            T::Error: ::std::fmt::Display,
        {
            self.key = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for key: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DecisionPointValueName>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<DecisionPointValue> for super::DecisionPointValue {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DecisionPointValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                key: value.key?,
                name: value.name?,
            })
        }
    }
    impl ::std::convert::From<super::DecisionPointValue> for DecisionPointValue {
        fn from(value: super::DecisionPointValue) -> Self {
            Self {
                description: Ok(value.description),
                key: Ok(value.key),
                name: Ok(value.name),
            }
        }
    }
}
