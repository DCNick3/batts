use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;
use std::str::FromStr;
use ts_rs::TS;
use uuid::Uuid;

/// An identified of an aggregate. It is an uuid under the hood.
#[derive(Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Id(Uuid);

impl Id {
    /// Generate a new random Id.
    pub fn generate() -> Self {
        Id(Uuid::new_v4())
    }
    /// Create an Id from a uuid.
    pub const fn from_uuid(uuid: Uuid) -> Self {
        Id(uuid)
    }
}

impl TS for Id {
    fn name() -> String {
        "string".to_string()
    }

    fn name_with_type_args(args: Vec<String>) -> String {
        assert!(args.is_empty(), "called name_with_type_args on primitive");
        Self::name()
    }

    fn inline() -> String {
        Self::name()
    }

    fn dependencies() -> Vec<ts_rs::Dependency>
    where
        Self: 'static,
    {
        vec![]
    }

    fn transparent() -> bool {
        false
    }
}

/// Error that can occur when parsing an Id from a string.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum IdError {
    /// Id contains invalid characters. Only base58 characters are allowed.
    #[error("Id contains invalid characters. Only base58 characters are allowed")]
    Alphabet(#[from] bs58::decode::Error),
    /// Id length is invalid. It must be 16 bytes/22 characters long.
    #[error("Id length is invalid. It must be 16 bytes/22 characters long.")]
    Length,
}

impl Debug for Id {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Id({})", bs58::encode(self.0.as_bytes()).into_string())
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", bs58::encode(self.0.as_bytes()).into_string())
    }
}

impl FromStr for Id {
    type Err = IdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = bs58::decode(s).into_vec().map_err(IdError::Alphabet)?;
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

/// Trait for types that can be converted to id. Used for id newtypes
pub trait AnyId: Debug + Send + Sync + Clone + Copy + PartialEq + Eq + Hash + 'static {
    /// Convert to id.
    fn from_id(id: Id) -> Self;
    /// Convert from id.
    fn id(&self) -> Id;
}

impl AnyId for Id {
    fn from_id(id: Id) -> Self {
        id
    }

    fn id(&self) -> Id {
        *self
    }
}
