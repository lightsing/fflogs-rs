//! API: GET `/zones`
use crate::v1::FFLogsV1Client;
use crate::v1::apis::ApiRequest;
use serde::Deserialize;
use std::fmt;

/// Request
#[must_use = "`Request` does nothing unless you execute it"]
#[derive(Debug, Clone)]
pub struct Request {
    client: FFLogsV1Client,
}

#[doc(hidden)]
pub struct Path;

impl ApiRequest for Request {
    type Response = Vec<Zone>;

    type Path<'a> = Path;

    type Query = ();

    fn client(&self) -> &FFLogsV1Client {
        &self.client
    }

    fn path(&self) -> Self::Path<'_> {
        Path
    }
}

/// Represents a zone with its details including encounters and brackets.
#[derive(Debug, Clone, Deserialize)]
pub struct Zone {
    /// A unique identifier representing this specific zone.
    pub id: u64,
    /// The English name of the raid zone.
    pub name: String,
    /// Whether the rankings and statistics for the zone are frozen.
    pub frozen: bool,
    /// The encounters for this zone.
    pub encounters: Vec<Encounter>,
    /// ???? This is not same as it in the API documentation.
    pub brackets: Brackets,
    /// partitions
    #[serde(default)]
    pub partitions: Vec<Partition>,
}

/// Represents an encounter with its details.
#[derive(Debug, Clone, Deserialize)]
pub struct Encounter {
    /// A unique identifier representing this specific encounter.
    pub id: u64,
    /// The English name of the encounter.
    pub name: String,
}

/// ????
#[derive(Debug, Clone, Deserialize)]
pub struct Brackets {
    /// ????
    pub min: f32,
    /// ????
    pub max: f32,
    /// ????
    pub bucket: f32,
    /// ???
    #[serde(rename = "type")]
    pub ty: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Partition {
    pub area: Option<i32>,
    pub name: String,
    pub compact: String,
    pub filtered_name: Option<String>,
    #[serde(default)]
    pub default: bool,
    #[serde(default)]
    pub frozen: bool,
}

impl Request {
    pub(crate) fn new(client: FFLogsV1Client) -> Self {
        Self { client }
    }
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "/zones")
    }
}
