//! API: GET `/report/events/{view}/{code}`
use super::common::DataType;
use crate::v1::FFLogsV1Client;
use compact_str::CompactString;
use serde::{Deserialize, Serialize};
use serde_with::{BoolFromInt, serde_as};
use std::fmt;
use std::ops::Not;

/// Request
#[must_use = "`Request` does nothing unless you execute it"]
#[derive(Debug, Clone)]
pub struct Request {
    view: DataType,
    code: CompactString,
    params: Params,
    client: FFLogsV1Client,
}

#[doc(hidden)]
pub struct Path<'a> {
    view: &'a DataType,
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
        Path {
            view: &self.view,
            code: &self.code,
        }
    }

    fn query(&self) -> Option<&Self::Query> {
        Some(&self.params)
    }
}

#[doc(hidden)]
#[serde_as]
#[derive(Default, Debug, Clone, Serialize)]
pub struct Params {
    #[serde(skip_serializing_if = "Option::is_none")]
    start: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end: Option<u64>,
    #[serde(default, skip_serializing_if = "<&bool>::not")]
    #[serde_as(as = "BoolFromInt")]
    hostility: bool,
    #[serde(rename = "sourceid", skip_serializing_if = "Option::is_none")]
    source_id: Option<u64>,
    #[serde(rename = "sourceinstance", skip_serializing_if = "Option::is_none")]
    source_instance: Option<u64>,
    #[serde(rename = "source_class", skip_serializing_if = "Option::is_none")]
    source_class: Option<CompactString>,
    #[serde(rename = "targetid", skip_serializing_if = "Option::is_none")]
    target_id: Option<u64>,
    #[serde(rename = "targetinstance", skip_serializing_if = "Option::is_none")]
    target_instance: Option<u64>,
    #[serde(rename = "targetclass", skip_serializing_if = "Option::is_none")]
    target_class: Option<CompactString>,
    // Those fields seem not to be used in FFXIV
    // #[serde(
    //     rename = "sourceAurasPresent",
    //     skip_serializing_if = "HashSet::is_empty"
    // )]
    // #[serde_as(as = "StringWithSeparator::<CommaSeparator, CompactString>")]
    // source_auras_present: HashSet<CompactString>,
    // #[serde(
    //     rename = "sourceAurasAbsent",
    //     skip_serializing_if = "HashSet::is_empty"
    // )]
    // #[serde_as(as = "StringWithSeparator::<CommaSeparator, CompactString>")]
    // source_auras_absent: HashSet<CompactString>,
    // #[serde(
    //     rename = "targetAurasPresent",
    //     skip_serializing_if = "HashSet::is_empty"
    // )]
    // #[serde_as(as = "StringWithSeparator::<CommaSeparator, CompactString>")]
    // target_auras_present: HashSet<CompactString>,
    // #[serde(
    //     rename = "targetAurasAbsent",
    //     skip_serializing_if = "HashSet::is_empty"
    // )]
    // #[serde_as(as = "StringWithSeparator::<CommaSeparator, CompactString>")]
    // target_auras_absent: HashSet<CompactString>,
    #[serde(rename = "abilityid", skip_serializing_if = "Option::is_none")]
    ability_id: Option<u64>,
    #[serde(rename = "death", skip_serializing_if = "Option::is_none")]
    death: Option<u64>,
    #[serde(rename = "options", skip_serializing_if = "Option::is_none")]
    options: Option<u64>,
    #[serde(rename = "cutoff", skip_serializing_if = "Option::is_none")]
    cutoff: Option<u64>,
    #[serde(rename = "encounter", skip_serializing_if = "Option::is_none")]
    encounter: Option<u64>,
    #[serde(default, rename = "wipes", skip_serializing_if = "<&bool>::not")]
    #[serde_as(as = "BoolFromInt")]
    wipes: bool,
    #[serde(rename = "difficulty", skip_serializing_if = "Option::is_none")]
    difficulty: Option<u64>,
    #[serde(rename = "filter", skip_serializing_if = "Option::is_none")]
    filter: Option<CompactString>,
    #[serde(default, rename = "translate", skip_serializing_if = "<&bool>::not")]
    translate: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Response {
    pub events: Vec<ReportEvent>,
    pub count: usize,
    // pub auraAbilities: Vec<AuraAbilities>, // Seems not to be used in FFXIV
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportEvent {
    pub timestamp: u64,
    #[serde(flatten)]
    pub source: Source,
    pub source_is_friendly: bool,
    #[serde(flatten)]
    pub target: Target,
    pub target_is_friendly: bool,
    pub ability: Ability,
    pub fight: u64,
    #[serde(default)]
    pub source_resources: Option<Resources>,
    #[serde(default)]
    pub target_resources: Option<Resources>,

    #[serde(flatten)]
    pub details: ReportEventDetails,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", untagged)]
pub enum Source {
    Id {
        #[serde(rename = "sourceID")]
        id: u64,
    },
    Other {
        source: DetailSourceTarget,
    },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", untagged)]
pub enum Target {
    Id {
        #[serde(rename = "targetID")]
        id: u64,
    },
    Other {
        target: DetailSourceTarget,
    },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DetailSourceTarget {
    pub name: CompactString,
    pub id: i64,
    pub guid: u64,
    #[serde(rename = "type")]
    pub ty: CompactString,
    pub icon: CompactString,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ability {
    pub name: CompactString,
    pub guid: u64,
    #[serde(rename = "type")]
    pub ty: u64,
    pub ability_icon: CompactString,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Resources {
    pub hit_points: u64,
    pub max_hit_points: u64,
    pub mp: u64,
    #[serde(rename = "maxMP")]
    pub max_mp: u64,
    pub tp: u64,
    #[serde(rename = "maxTP")]
    pub max_tp: u64,
    pub x: u64,
    pub y: u64,
    pub facing: i64,
    #[serde(default)]
    pub absorb: u64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type")]
pub enum ReportEventDetails {
    #[serde(rename = "applybuff")]
    ApplyBuff(BuffEvent),
    #[serde(rename = "applydebuff")]
    ApplyDebuff(BuffEvent),
    #[serde(rename = "refreshdebuff")]
    RefreshDebuff(BuffEvent),
    #[serde(rename = "removebuff")]
    RemoveBuff,
    #[serde(rename = "removedebuff")]
    RemoveDebuff,
    #[serde(rename = "applybuffstack")]
    ApplyBuffStack(StackBuffEvent),
    #[serde(rename = "applydebuffstack")]
    ApplyDebuffStack(StackBuffEvent),

    #[serde(rename = "damage")]
    Damage(HitEvent),
    #[serde(rename = "calculateddamage")]
    CalculatedDamage(HitEvent),

    #[serde(rename = "heal")]
    Heal(HitEvent),
    #[serde(rename = "calculatedheal")]
    CalculatedHeal(HitEvent),

    #[serde(rename = "begincast")]
    BeginCast(BeginCastEvent),
    #[serde(rename = "cast")]
    Cast,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuffEvent {
    pub extra_ability: Option<Ability>,
    pub duration: u64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StackBuffEvent {
    pub stack: u64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HitEvent {
    pub hit_type: u64,
    pub amount: u64,
    #[serde(default)]
    pub unmitigated_amount: Option<u64>,
    #[serde(default)]
    pub direct_hit: bool,
    pub multiplier: Option<f64>,
    #[serde(rename = "packetID")]
    pub packet_id: Option<u64>,
    #[serde(default)]
    pub unpaired: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BeginCastEvent {
    pub duration: u64,
}

impl Request {
    pub(crate) fn new(
        client: FFLogsV1Client,
        view: DataType,
        code: impl Into<CompactString>,
    ) -> Self {
        Self {
            view,
            code: code.into(),
            params: Params::default(),
            client,
        }
    }

    /// Set the view.
    pub fn view(mut self, view: DataType) -> Self {
        self.view = view;
        self
    }

    /// Set the report code.
    pub fn code(mut self, code: impl Into<CompactString>) -> Self {
        self.code = code.into();
        self
    }

    /// Set the start time.
    ///
    /// This is a time from the start of the report in milliseconds. If omitted, 0 is assumed.
    pub fn start(mut self, start: u64) -> Self {
        self.params.start = Some(start);
        self
    }

    /// Set the end time.
    ///
    /// This is a time from the start of the report in milliseconds. If omitted, 0 is assumed.
    pub fn end(mut self, end: u64) -> Self {
        self.params.end = Some(end);
        self
    }

    /// Set the optional hostility value of boolean.
    ///
    /// The default is false.
    /// - `false` means to collect data for Friendlies.
    /// - `true` means to collect data for Enemies.
    pub fn hostility(mut self, hostility: bool) -> Self {
        self.params.hostility = hostility;
        self
    }

    /// Set the optional actor ID to filter to.
    ///
    /// If set, only events where the ID matches the source (or target for damage-taken)
    /// of the event will be returned.
    ///
    /// The actor's pets will also be included (unless the options field overrides).
    pub fn source_id(mut self, source_id: u64) -> Self {
        self.params.source_id = Some(source_id);
        self
    }

    /// Set the optional actor instance ID to filter to.
    ///
    /// If set, only events where the instance ID matches the source (or target for damage-taken)
    /// of the event will be returned.
    ///
    /// This is useful to look for all events involving NPC N, where N is the actor instance ID.
    pub fn source_instance(mut self, source_instance: u64) -> Self {
        self.params.source_instance = Some(source_instance);
        self
    }

    /// Set the optional actor class to filter to.
    ///
    /// If set, only events where the source (or target for damage-taken) involves that class
    /// (e.g., Mage) will be returned.
    pub fn source_class(mut self, source_class: impl Into<CompactString>) -> Self {
        self.params.source_class = Some(source_class.into());
        self
    }

    /// Set the optional actor ID to filter to.
    ///
    /// If set, only events where the ID matches the target (or source for damage-taken)
    /// of the event will be returned.
    ///
    /// This value is not used in the 'deaths', 'survivability', 'resources' and 'resources-gains'
    /// views.
    pub fn target_id(mut self, target_id: u64) -> Self {
        self.params.target_id = Some(target_id);
        self
    }

    /// Set the optional actor instance ID to filter to.
    ///
    /// If set, only events where the instance ID matches the target (or source for damage-taken)
    /// of the event will be returned.
    ///
    /// This is useful to look for all events involving NPC N, where N is the actor instance ID.
    ///
    /// This value is not used in the 'deaths', 'survivability', 'resources' and 'resources-gains'
    /// views.
    pub fn target_instance(mut self, target_instance: u64) -> Self {
        self.params.target_instance = Some(target_instance);
        self
    }

    /// Set the optional actor class to filter to.
    ///
    /// If set, only events where the target (or source for damage-taken) involves that class
    /// (e.g., Mage) will be returned.
    ///
    /// This value is not used in the 'deaths', 'survivability', 'resources' and 'resources-gains'
    /// views.
    pub fn target_class(mut self, target_class: impl Into<CompactString>) -> Self {
        self.params.target_class = Some(target_class.into());
        self
    }

    /// Set the optional ability ID to filter to.
    ///
    /// If set, only events where the ability matches will be returned.
    /// Consolidated abilities (WCL only) are represented using a negative number that matches the
    /// ability ID that everything is consolidated under.
    ///
    /// For the 'deaths' view, this represents a specific killing blow.
    /// For the resources views, the abilityid is not an ability but a resource type.
    ///
    /// Valid resource types can be viewed at https://www.fflogs.com/reports/resource_types/
    pub fn ability_id(mut self, ability_id: u64) -> Self {
        self.params.ability_id = Some(ability_id);
        self
    }

    /// Set the optional death to filter to.
    ///
    /// Only used for the deaths command.
    ///
    /// Select the Nth death in the time range that matches all the other filters.
    pub fn death(mut self, death: u64) -> Self {
        self.params.death = Some(death);
        self
    }

    /// Set the set of options for what to include/exclude.
    ///
    /// These correspond to options like Include Overkill in the Damage Done pane.
    /// Complete list will be forthcoming.
    ///
    /// If omitted, appropriate defaults that match WCL's default behavior will be chosen.
    ///
    /// This value is not used in the 'deaths', 'survivability', 'resources' and 'resources-gains'
    /// views.
    pub fn options(mut self, options: u64) -> Self {
        self.params.options = Some(options);
        self
    }

    /// Set the optional death cutoff.
    ///
    /// If set, events after that number of deaths have occurred will not be examined.
    pub fn cutoff(mut self, cutoff: u64) -> Self {
        self.params.cutoff = Some(cutoff);
        self
    }

    /// Set the optional encounter filter.
    ///
    /// If set to a specific encounter ID, only fights involving a specific encounter will be
    /// considered.
    ///
    /// The encounter IDs match those used in rankings/statistics.
    pub fn encounter(mut self, encounter: u64) -> Self {
        self.params.encounter = Some(encounter);
        self
    }

    /// Set the optional wipes filter.
    ///
    /// If set to `true`, only wipes will be considered.
    pub fn wipes(mut self, wipes: bool) -> Self {
        self.params.wipes = wipes;
        self
    }

    /// Set the optional difficulty filter.
    pub fn difficulty(mut self, difficulty: u64) -> Self {
        self.params.difficulty = Some(difficulty);
        self
    }

    /// Set the optional filter written in WCL's expression language.
    ///
    /// Events must match the filter to be included.
    pub fn filter(mut self, filter: impl Into<CompactString>) -> Self {
        self.params.filter = Some(filter.into());
        self
    }

    /// Set the optional flag indicating that the results should be translated into the language
    /// of the host (e.g., cn.fflogs.com would get Chinese results).
    pub fn translate(mut self, translate: bool) -> Self {
        self.params.translate = translate;
        self
    }
}

impl fmt::Display for Path<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "/report/events/{view}/{code}",
            view = self.view,
            code = self.code
        )
    }
}
