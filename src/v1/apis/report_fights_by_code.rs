//! API: GET `/report/fights/{code}`

use crate::v1::FFLogsV1Client;
use compact_str::CompactString;
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::fmt;
use std::ops::Not;

/// Request
#[must_use = "`Request` does nothing unless you execute it"]
#[derive(Debug, Clone)]
pub struct Request {
    code: CompactString,
    params: Params,
    client: FFLogsV1Client,
}

#[doc(hidden)]
pub struct Path<'a> {
    code: &'a CompactString,
}

impl super::ApiRequest for Request {
    type Response = Response;

    type Path<'a> = Path<'a>;

    type Query = Params;

    fn client(&self) -> &FFLogsV1Client {
        &self.client
    }

    fn path(&self) -> Self::Path<'_> {
        Path { code: &self.code }
    }

    fn query(&self) -> Option<&Self::Query> {
        Some(&self.params)
    }
}

#[doc(hidden)]
#[derive(Default, Debug, Clone, Serialize)]
pub struct Params {
    #[serde(default, skip_serializing_if = "<&bool>::not")]
    translate: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub lang: CompactString,
    pub fights: SmallVec<Fight, 4>,
    pub friendlies: SmallVec<Friendly, 32>,
    pub enemies: SmallVec<Enemy, 32>,
    pub friendly_pets: SmallVec<FriendlyPet, 32>,
    // pub enemy_pets: SmallVec<EnemyPet, 32>,
    pub log_version: u64,
    pub game_version: u64,
    // pub phases: Vec<Phase>,
    pub title: CompactString,
    pub owner: CompactString,
    pub start: u64,
    pub end: u64,
    pub zone: u64,
    pub exported_characters: SmallVec<ExportedCharacter, 32>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fight {
    pub id: u64,
    pub boss: u64,
    #[serde(rename = "start_time")]
    pub start_time: u64,
    #[serde(rename = "end_time")]
    pub end_time: u64,
    pub name: CompactString,
    #[serde(rename = "zoneID")]
    pub zone_id: u64,
    pub zone_name: CompactString,
    pub zone_counter: u64,
    pub size: u64,
    pub difficulty: u64,
    pub kill: bool,
    pub partial: u64,
    pub in_progress: bool,
    pub standard_composition: bool,
    pub has_echo: bool,
    #[serde(rename = "hasTrustNPCs")]
    pub has_trust_npcs: bool,
    pub combat_time: u64,
    pub boss_percentage: u64,
    pub fight_percentage: u64,
    pub last_phase_as_absolute_index: u64,
    pub last_phase_for_percentage_display: u64,
    pub maps: SmallVec<Map, 1>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Map {
    #[serde(rename = "mapID")]
    pub map_id: u64,
    pub map_name: CompactString,
    pub map_file: CompactString,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Friendly {
    pub name: CompactString,
    pub id: u64,
    pub guid: u64,
    #[serde(rename = "type")]
    pub ty: CompactString,
    pub server: Option<CompactString>,
    pub icon: CompactString,
    pub fights: SmallVec<FriendlyFight, 32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FriendlyFight {
    pub id: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Enemy {
    pub name: CompactString,
    pub id: u64,
    pub guid: u64,
    #[serde(rename = "type")]
    pub ty: CompactString,
    pub icon: CompactString,
    pub fights: Vec<EnemyFight>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnemyFight {
    pub id: u64,
    pub instances: u64,
    pub groups: Option<u64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FriendlyPet {
    pub name: CompactString,
    pub id: u64,
    pub guid: u64,
    #[serde(rename = "type")]
    pub ty: CompactString,
    pub icon: CompactString,
    pub pet_owner: Option<u64>,
    pub fights: Vec<FriendlyPetFight>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FriendlyPetFight {
    pub id: u64,
    pub instances: u64,
}

// #[derive(Debug, Clone, Deserialize)]
// pub struct EnemyPet {
//     // todo
// }

// #[derive(Debug, Clone, Deserialize)]
// pub struct Phase {
//     // todo
// }

#[derive(Debug, Clone, Deserialize)]
pub struct ExportedCharacter {
    pub id: u64,
    pub name: CompactString,
    pub server: CompactString,
    pub region: CompactString,
}

impl Request {
    pub(crate) fn new(client: FFLogsV1Client, code: impl Into<CompactString>) -> Self {
        Self {
            code: code.into(),
            params: Params::default(),
            client,
        }
    }

    /// Set the report code.
    pub fn code(mut self, code: impl Into<CompactString>) -> Self {
        self.code = code.into();
        self
    }

    /// Set the optional flag indicating that the results should be translated into the language
    /// of the host (e.g., cn.fflogs.com would get Chinese results)
    pub fn translate(mut self, translate: bool) -> Self {
        self.params.translate = translate;
        self
    }
}

impl fmt::Display for Path<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "/report/fights/{code}", code = self.code)
    }
}
