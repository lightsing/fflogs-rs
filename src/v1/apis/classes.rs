//! API: GET /classes
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
    type Response = Vec<Class>;

    type Path<'a> = Path;

    type Query = ();

    fn client(&self) -> &FFLogsV1Client {
        &self.client
    }

    fn path(&self) -> Self::Path<'_> {
        Path
    }
}

/// Represents a class with its details and possible specs.
#[derive(Debug, Clone, Deserialize)]
pub struct Class {
    /// A unique identifier representing this specific class.
    pub id: i32,
    /// The English name of the class.
    pub name: String,
    /// The possible specs for this class.
    pub specs: Vec<Spec>,
}

/// Represents a spec with its details.
#[derive(Debug, Clone, Deserialize)]
pub struct Spec {
    /// A unique identifier representing this specific spec.
    pub id: i32,
    /// The English name of the spec.
    pub name: String,
}

impl Request {
    pub(crate) fn new(client: FFLogsV1Client) -> Self {
        Self { client }
    }
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "/classes")
    }
}
