use crate::v1::FFLogsV1Client;
use crate::v1::reqs::ApiRequest;
use compact_str::CompactString;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Request {
    encounter_id: u64,
    params: Params,
    client: FFLogsV1Client,
}

pub struct Path<'a> {
    encounter_id: &'a u64,
}

impl ApiRequest for Request {
    type Output = Response;

    type Path<'a> = Path<'a>;

    type Query = Params;

    fn client(&self) -> &FFLogsV1Client {
        &self.client
    }

    fn path(&self) -> Self::Path<'_> {
        Path {
            encounter_id: &self.encounter_id,
        }
    }

    fn query(&self) -> Option<&Self::Query> {
        Some(&self.params)
    }
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct Params {
    #[serde(skip_serializing_if = "Option::is_none")]
    metric: Option<Metric>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<CompactString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    partition: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    class: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spec: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bracket: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    server: Option<CompactString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<CompactString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<CompactString>,
}

/// Valid fight metrics are 'speed', 'execution' and 'feats'.
/// Valid character metrics are 'dps', 'hps', 'bossdps, 'tankhps', or 'playerspeed'.
#[derive(Debug, Copy, Clone, Serialize)]
pub enum Metric {
    #[serde(rename = "speed")]
    Speed,
    #[serde(rename = "execution")]
    Execution,
    #[serde(rename = "feats")]
    Feats,
    #[serde(rename = "dps")]
    Dps,
    #[serde(rename = "hps")]
    Hps,
    #[serde(rename = "bossdps")]
    BossDps,
    #[serde(rename = "tankhps")]
    TankHps,
    #[serde(rename = "playerspeed")]
    PlayerSpeed,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub page: usize,
    pub has_more_pages: bool,
    pub count: usize,
    pub rankings: Vec<Ranking>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ranking {
    pub name: String,
    pub class: u64,
    pub spec: u64,
    pub total: f64,
    pub duration: u64,
    pub start_time: u64,
    #[serde(rename = "fightID")]
    pub fight_id: u64,
    #[serde(rename = "reportID")]
    pub report_id: String,
    pub guild_name: Option<String>,
    #[serde(rename = "serverName")]
    pub server_name: String,
    #[serde(rename = "regionName")]
    pub region_name: String,
    pub hidden: bool,
    pub patch: i64,

    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl Request {
    pub(crate) fn new(client: FFLogsV1Client, encounter_id: u64) -> Self {
        Self {
            encounter_id,
            params: Params::default(),
            client,
        }
    }

    pub fn metric(&mut self, metric: Metric) -> &mut Self {
        self.params.metric = Some(metric);
        self
    }

    pub fn size(&mut self, size: impl Into<CompactString>) -> &mut Self {
        self.params.size = Some(size.into());
        self
    }

    pub fn partition(&mut self, partition: u64) -> &mut Self {
        self.params.partition = Some(partition);
        self
    }

    pub fn class(&mut self, class: u64) -> &mut Self {
        self.params.class = Some(class);
        self
    }

    pub fn spec(&mut self, spec: u64) -> &mut Self {
        self.params.spec = Some(spec);
        self
    }

    pub fn bracket(&mut self, bracket: u64) -> &mut Self {
        self.params.bracket = Some(bracket);
        self
    }

    pub fn server(&mut self, server: impl Into<CompactString>) -> &mut Self {
        self.params.server = Some(server.into());
        self
    }

    pub fn region(&mut self, region: impl Into<CompactString>) -> &mut Self {
        self.params.region = Some(region.into());
        self
    }

    pub fn page(&mut self, page: u64) -> &mut Self {
        self.params.page = Some(page);
        self
    }

    pub fn filter(&mut self, filter: impl Into<CompactString>) -> &mut Self {
        self.params.filter = Some(filter.into());
        self
    }
}

impl fmt::Display for Path<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "/rankings/encounter/{}", self.encounter_id)
    }
}
