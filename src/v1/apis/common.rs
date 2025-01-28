//! Common types used in requests.
use serde::{Deserialize, Serialize};

/// The type of data requested.
#[derive(
    Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq, strum::Display, strum::EnumIter,
)]
#[serde(rename_all = "kebab-case")]
pub enum DataType {
    #[strum(serialize = "summary")]
    Summary,
    #[strum(serialize = "damage-done")]
    DamageDone,
    #[strum(serialize = "damage-taken")]
    DamageTaken,
    #[strum(serialize = "healing")]
    Healing,
    #[strum(serialize = "casts")]
    Casts,
    #[strum(serialize = "summons")]
    Summons,
    #[strum(serialize = "buffs")]
    Buffs,
    #[strum(serialize = "debuffs")]
    Debuffs,
    #[strum(serialize = "deaths")]
    Deaths,
    #[strum(serialize = "threat")]
    Threat,
    // FIXME: how to use this view?
    // #[strum(serialize = "resources")]
    // Resources,
    #[strum(serialize = "interrupts")]
    Interrupts,
    #[strum(serialize = "dispels")]
    Dispels,
}
