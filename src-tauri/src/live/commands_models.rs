/// Represents the health of a boss.
#[derive(specta::Type, serde::Serialize, serde::Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BossHealth {
    /// The unique ID of the boss.
    pub uid: i64,
    /// The name of the boss.
    pub name: String,
    /// The current HP of the boss.
    pub current_hp: Option<i64>,
    /// The maximum HP of the boss.
    pub max_hp: Option<i64>,
}

/// Represents the header information for an encounter.
#[derive(specta::Type, serde::Serialize, serde::Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct HeaderInfo {
    /// The total DPS of the encounter.
    pub total_dps: f64,
    /// The total damage of the encounter.
    pub total_dmg: u128,
    /// The elapsed time of the encounter in milliseconds.
    pub elapsed_ms: u128,
    /// The timestamp of when the fight started, in milliseconds since the Unix epoch.
    pub fight_start_timestamp_ms: u128, // Unix timestamp when fight started
    /// A list of bosses in the encounter.
    pub bosses: Vec<BossHealth>,
    /// The ID of the scene where the encounter took place.
    pub scene_id: Option<i32>,
    /// The name of the scene where the encounter took place.
    pub scene_name: Option<String>,
    /// The current segment type ('boss', 'trash', or null if no segment active).
    pub current_segment_type: Option<String>,
    /// The display name for the current segment (boss name when available).
    pub current_segment_name: Option<String>,
}

/// Represents the players window.
#[derive(specta::Type, serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlayersWindow {
    /// A list of player rows.
    pub player_rows: PlayerRows,
}

/// A type alias for a list of player rows.
pub type PlayerRows = Vec<PlayerRow>;

/// Represents a row in the players window.
#[derive(specta::Type, serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlayerRow {
    /// The unique ID of the player.
    pub uid: u128,
    /// The name of the player.
    pub name: String,
    /// The class name of the player.
    pub class_name: String,
    /// The class spec name of the player.
    pub class_spec_name: String,
    /// The ability score of the player.
    pub ability_score: u128,
    /// The total damage dealt by the player.
    pub total_dmg: u128,
    /// The DPS of the player.
    pub dps: f64,
    /// The "True DPS" of the player (uses active damage time).
    pub tdps: f64,
    /// The accumulated active damage time used for True DPS, in milliseconds.
    pub active_time_ms: u128,
    /// The damage percentage of the player.
    pub dmg_pct: f64,
    /// The critical hit rate of the player.
    pub crit_rate: f64,
    /// The critical damage rate of the player.
    pub crit_dmg_rate: f64,
    /// The lucky hit rate of the player.
    pub lucky_rate: f64,
    /// The lucky damage rate of the player.
    pub lucky_dmg_rate: f64,
    /// The number of hits dealt by the player.
    pub hits: u128,
    /// The number of hits per minute dealt by the player.
    pub hits_per_minute: f64,
    /// The total damage dealt to bosses by the player.
    pub boss_dmg: u128,
    /// The DPS dealt to bosses by the player.
    pub boss_dps: f64,
    /// The percentage contribution of boss damage relative to all boss damage.
    pub boss_dmg_pct: f64,
    // Extended player attributes from Stage 4
    /// The rank level of the player.
    pub rank_level: Option<i64>,
    /// The current HP of the player.
    pub current_hp: Option<i64>,
    /// The maximum HP of the player.
    pub max_hp: Option<i64>,
    /// The critical hit stat of the player.
    pub crit_stat: Option<i64>,
    /// The lucky hit stat of the player.
    pub lucky_stat: Option<i64>,
    /// The haste of the player.
    pub haste: Option<i64>,
    /// The mastery of the player.
    pub mastery: Option<i64>,
    /// The element flag of the player.
    pub element_flag: Option<i64>,
    /// The energy flag of the player.
    pub energy_flag: Option<i64>,
    /// The reduction level of the player.
    pub reduction_level: Option<i64>,
}

/// Represents the skills window.
#[derive(specta::Type, serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SkillsWindow {
    /// A list of player rows for the current player.
    pub curr_player: PlayerRows,
    /// A list of skill rows.
    pub skill_rows: SkillRows,
}

/// A type alias for a list of skill rows.
pub type SkillRows = Vec<SkillRow>;

/// Represents a row in the skills window.
#[derive(specta::Type, serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SkillRow {
    /// Unique skill identifier (used as stable key in frontend).
    pub skill_id: i64,
    /// The name of the skill.
    pub name: String,
    /// The total damage dealt by the skill.
    pub total_dmg: u128,
    /// The DPS of the skill.
    pub dps: f64,
    /// The damage percentage of the skill.
    pub dmg_pct: f64,
    /// The critical hit rate of the skill.
    pub crit_rate: f64,
    /// The critical damage rate of the skill.
    pub crit_dmg_rate: f64,
    /// The lucky hit rate of the skill.
    pub lucky_rate: f64,
    /// The lucky damage rate of the skill.
    pub lucky_dmg_rate: f64,
    /// The number of hits dealt by the skill.
    pub hits: u128,
    /// The number of hits per minute dealt by the skill.
    pub hits_per_minute: f64,
}

/// Represents a skill cooldown state.
#[derive(specta::Type, serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SkillCdState {
    /// The skill level ID.
    pub skill_level_id: i32,
    /// The cooldown begin timestamp
    pub begin_time: i64,
    /// The total duration of the cooldown in milliseconds.
    /// -1 indicates a charge/resource style entry.
    pub duration: i32,
    /// The cooldown type enum value
    pub skill_cd_type: i32,
    /// The server-reported valid cooldown time in milliseconds.
    pub valid_cd_time: i32,
    /// Local timestamp when this cooldown state was received
    pub received_at: i64,
    /// Cooldown duration after applying AttrSkillCD/AttrSkillCDPCT and TempAttr rules.
    pub calculated_duration: i32,
    /// Cooldown accelerate rate for this skill
    pub cd_accelerate_rate: f32,
}

/// Represents a buff update state.
#[derive(specta::Type, serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BuffUpdateState {
    pub buff_uuid: i32,
    pub base_id: i32,
    pub layer: i32,
    pub duration_ms: i32,
    pub create_time_ms: i64,
    pub source_config_id: i32,
}

#[derive(specta::Type, serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BuffDefinition {
    pub base_id: i32,
    pub name: String,
    pub sprite_file: String,
    pub talent_name: Option<String>,
    pub talent_sprite_file: Option<String>,
    pub search_keywords: Vec<String>,
}

#[derive(specta::Type, serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BuffNameInfo {
    pub base_id: i32,
    pub name: String,
    pub has_sprite_file: bool,
}

#[derive(serde::Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BuffUpdatePayload {
    pub buffs: Vec<BuffUpdateState>,
}

#[derive(serde::Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SkillCdUpdatePayload {
    pub skill_cds: Vec<SkillCdState>,
}

#[derive(specta::Type, serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FightResourceState {
    /// The full list of resource values
    pub values: Vec<i64>,
    /// Local timestamp when this state was received
    pub received_at: i64,
}

#[derive(serde::Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FightResourceUpdatePayload {
    pub fight_res: FightResourceState,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct ModuleCalcProgressPayload {
    pub processed: u64,
    pub total: u64,
}
