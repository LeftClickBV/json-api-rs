use std::{
    fmt::{self, Display, Formatter},
    str::FromStr,
};

use serde::{
    de::{Deserializer, Error as DeError},
    ser::Serializer,
    Deserialize, Serialize,
};

use crate::{error::Error, value::Map};

/// Information about this implementation of the specification.
///
/// For more information, check out the *[JSON API object]* section of the JSON API
/// specification.
///
/// [JSON API object]: https://goo.gl/hZUcEt
#[non_exhaustive]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct JsonApi {
    /// Non-standard meta information. If this value of this field is empty, it will not
    /// be included if the object is serialized. For more information, check out the
    /// *[meta information]* section of the JSON API specification.
    ///
    /// [meta information]: https://goo.gl/LyrGF8
    #[serde(default, skip_serializing_if = "Map::is_empty")]
    pub meta: Map,

    /// The latest version of the JSON API specification that is supported by
    /// this implementation. Defaults to the latest available version.
    pub version: Version,
}

impl JsonApi {
    /// Returns a new `JsonApi` with the specified `version`.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate json_api;
    /// #
    /// # fn main() {
    /// use json_api::doc::{JsonApi, Version};
    /// assert_eq!(JsonApi::default(), JsonApi::new(Version::V1));
    /// # }
    /// ```
    pub fn new(version: Version) -> Self {
        JsonApi {
            version,
            meta: Default::default(),
        }
    }
}

/// The version of the specification.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Version {
    /// Version 1.0
    V1,
}

impl Default for Version {
    fn default() -> Self {
        Version::V1
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(match *self {
            Version::V1 => "1.0",
        })
    }
}

impl FromStr for Version {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "1.0" => Ok(Version::V1),
            v => Err(Error::unsupported_version(v)),
        }
    }
}

impl<'de> Deserialize<'de> for Version {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        value.parse().map_err(D::Error::custom)
    }
}

impl Serialize for Version {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(match *self {
            Version::V1 => "1.0",
        })
    }
}
