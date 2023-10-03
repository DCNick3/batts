use serde::{Deserialize, Deserializer, Serialize, Serializer};
use snafu::{ResultExt, Snafu};
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Id(Uuid);

impl Id {
    pub fn is_default(&self) -> bool {
        self.0 == Uuid::from_bytes([0; 16])
    }
}

/// A backdoor to have better types when implementing the query
impl Default for Id {
    fn default() -> Self {
        Id(Uuid::from_bytes([0; 16]))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Snafu)]
pub enum IdError {
    /// Id contains invalid characters. Only base58 characters are allowed.
    Alphabet { source: bs58::decode::Error },
    /// Id length is invalid. It must be 16 bytes/22 characters long.
    Length,
}

impl Display for Id {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", bs58::encode(self.0.as_bytes()).into_string())
    }
}

impl FromStr for Id {
    type Err = IdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = bs58::decode(s).into_vec().context(AlphabetSnafu)?;
        let uuid = Uuid::from_slice(&bytes).map_err(|_| IdError::Length)?;
        Ok(Id(uuid))
    }
}

impl Serialize for Id {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        bs58::encode(self.0.as_bytes())
            .into_string()
            .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Id {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(serde::de::Error::custom)
    }
}
