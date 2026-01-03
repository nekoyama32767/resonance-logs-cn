-- Enable foreign keys
PRAGMA foreign_keys = ON;

-- Sessions table (legacy/unused but kept for compatibility)
CREATE TABLE IF NOT EXISTS sessions (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  started_at_ms INTEGER NOT NULL,
  ended_at_ms INTEGER,
  version TEXT,
  platform TEXT
);

-- Entities table (players only)
CREATE TABLE IF NOT EXISTS entities (
  entity_id INTEGER PRIMARY KEY,
  name TEXT,
  class_id INTEGER,
  class_spec INTEGER,
  ability_score INTEGER,
  level INTEGER,
  first_seen_ms INTEGER,
  last_seen_ms INTEGER,
  attributes TEXT
);
CREATE INDEX IF NOT EXISTS idx_entities_last_seen ON entities(last_seen_ms);

-- Detailed player data
CREATE TABLE IF NOT EXISTS detailed_playerdata (
  player_id INTEGER PRIMARY KEY NOT NULL,
  last_seen_ms INTEGER NOT NULL,
  vdata_bytes BLOB
);

-- Encounters table
CREATE TABLE IF NOT EXISTS encounters (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  started_at_ms INTEGER NOT NULL,
  ended_at_ms INTEGER,
  local_player_id INTEGER,
  total_dmg INTEGER DEFAULT 0,
  total_heal INTEGER DEFAULT 0,
  scene_id INTEGER,
  scene_name TEXT,
  duration REAL NOT NULL DEFAULT 0.0,
  uploaded_at_ms INTEGER,
  remote_encounter_id INTEGER,
  is_favorite INTEGER NOT NULL DEFAULT 0,
  is_manually_reset INTEGER NOT NULL DEFAULT 0
);
CREATE INDEX IF NOT EXISTS idx_encounters_started ON encounters(started_at_ms);

-- Actor encounter stats
CREATE TABLE IF NOT EXISTS actor_encounter_stats (
  encounter_id INTEGER NOT NULL,
  actor_id INTEGER NOT NULL,
  name TEXT,
  class_id INTEGER,
  class_spec INTEGER,
  ability_score INTEGER,
  level INTEGER,
  damage_dealt INTEGER NOT NULL DEFAULT 0,
  heal_dealt INTEGER NOT NULL DEFAULT 0,
  damage_taken INTEGER NOT NULL DEFAULT 0,
  hits_dealt INTEGER NOT NULL DEFAULT 0,
  hits_heal INTEGER NOT NULL DEFAULT 0,
  hits_taken INTEGER NOT NULL DEFAULT 0,
  crit_hits_dealt INTEGER NOT NULL DEFAULT 0,
  crit_hits_heal INTEGER NOT NULL DEFAULT 0,
  crit_hits_taken INTEGER NOT NULL DEFAULT 0,
  lucky_hits_dealt INTEGER NOT NULL DEFAULT 0,
  lucky_hits_heal INTEGER NOT NULL DEFAULT 0,
  lucky_hits_taken INTEGER NOT NULL DEFAULT 0,
  crit_total_dealt INTEGER NOT NULL DEFAULT 0,
  crit_total_heal INTEGER NOT NULL DEFAULT 0,
  crit_total_taken INTEGER NOT NULL DEFAULT 0,
  lucky_total_dealt INTEGER NOT NULL DEFAULT 0,
  lucky_total_heal INTEGER NOT NULL DEFAULT 0,
  lucky_total_taken INTEGER NOT NULL DEFAULT 0,
  boss_damage_dealt INTEGER NOT NULL DEFAULT 0,
  boss_hits_dealt INTEGER NOT NULL DEFAULT 0,
  boss_crit_hits_dealt INTEGER NOT NULL DEFAULT 0,
  boss_lucky_hits_dealt INTEGER NOT NULL DEFAULT 0,
  boss_crit_total_dealt INTEGER NOT NULL DEFAULT 0,
  boss_lucky_total_dealt INTEGER NOT NULL DEFAULT 0,
  revives INTEGER NOT NULL DEFAULT 0,
  dps REAL NOT NULL DEFAULT 0.0,
  active_dmg_time_ms INTEGER NOT NULL DEFAULT 0,
  tdps REAL NOT NULL DEFAULT 0.0,
  duration REAL NOT NULL DEFAULT 0.0,
  is_player INTEGER NOT NULL DEFAULT 0,
  is_local_player INTEGER NOT NULL DEFAULT 0,
  attributes TEXT,
  PRIMARY KEY (encounter_id, actor_id),
  FOREIGN KEY(encounter_id) REFERENCES encounters(id) ON DELETE CASCADE
);

-- Damage skill stats
CREATE TABLE IF NOT EXISTS damage_skill_stats (
  encounter_id INTEGER NOT NULL,
  attacker_id INTEGER NOT NULL,
  defender_id INTEGER,
  skill_id INTEGER NOT NULL,
  hits INTEGER NOT NULL DEFAULT 0,
  total_value INTEGER NOT NULL DEFAULT 0,
  crit_hits INTEGER NOT NULL DEFAULT 0,
  lucky_hits INTEGER NOT NULL DEFAULT 0,
  crit_total INTEGER NOT NULL DEFAULT 0,
  lucky_total INTEGER NOT NULL DEFAULT 0,
  hp_loss_total INTEGER NOT NULL DEFAULT 0,
  shield_loss_total INTEGER NOT NULL DEFAULT 0,
  hit_details TEXT NOT NULL,
  monster_name TEXT,
  PRIMARY KEY (encounter_id, attacker_id, defender_id, skill_id),
  FOREIGN KEY(encounter_id) REFERENCES encounters(id) ON DELETE CASCADE
);

-- Heal skill stats
CREATE TABLE IF NOT EXISTS heal_skill_stats (
  encounter_id INTEGER NOT NULL,
  healer_id INTEGER NOT NULL,
  target_id INTEGER,
  skill_id INTEGER NOT NULL,
  hits INTEGER NOT NULL DEFAULT 0,
  total_value INTEGER NOT NULL DEFAULT 0,
  crit_hits INTEGER NOT NULL DEFAULT 0,
  lucky_hits INTEGER NOT NULL DEFAULT 0,
  crit_total INTEGER NOT NULL DEFAULT 0,
  lucky_total INTEGER NOT NULL DEFAULT 0,
  heal_details TEXT NOT NULL,
  monster_name TEXT,
  PRIMARY KEY (encounter_id, healer_id, target_id, skill_id),
  FOREIGN KEY(encounter_id) REFERENCES encounters(id) ON DELETE CASCADE
);

-- Encounter bosses
CREATE TABLE IF NOT EXISTS encounter_bosses (
  encounter_id INTEGER NOT NULL,
  monster_name TEXT NOT NULL,
  hits INTEGER NOT NULL DEFAULT 0,
  total_damage INTEGER NOT NULL DEFAULT 0,
  max_hp INTEGER,
  is_defeated INTEGER NOT NULL DEFAULT 0,
  PRIMARY KEY (encounter_id, monster_name),
  FOREIGN KEY(encounter_id) REFERENCES encounters(id) ON DELETE CASCADE
);

-- Death events
CREATE TABLE IF NOT EXISTS death_events (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  encounter_id INTEGER NOT NULL,
  timestamp_ms INTEGER NOT NULL,
  actor_id INTEGER NOT NULL,
  killer_id INTEGER,
  skill_id INTEGER,
  is_local_player INTEGER NOT NULL DEFAULT 0,
  attempt_index INTEGER,
  FOREIGN KEY(encounter_id) REFERENCES encounters(id) ON DELETE CASCADE
);
CREATE INDEX IF NOT EXISTS idx_death_events_encounter ON death_events(encounter_id);

-- Attempts
CREATE TABLE IF NOT EXISTS attempts (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  encounter_id INTEGER NOT NULL,
  attempt_index INTEGER NOT NULL,
  started_at_ms INTEGER NOT NULL,
  ended_at_ms INTEGER,
  reason TEXT NOT NULL,
  boss_hp_start INTEGER,
  boss_hp_end INTEGER,
  total_deaths INTEGER NOT NULL DEFAULT 0,
  FOREIGN KEY(encounter_id) REFERENCES encounters(id) ON DELETE CASCADE
);
CREATE INDEX IF NOT EXISTS idx_attempts_encounter ON attempts(encounter_id);

-- Dungeon segments
CREATE TABLE IF NOT EXISTS dungeon_segments (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  encounter_id INTEGER NOT NULL,
  segment_type TEXT NOT NULL,
  boss_entity_id INTEGER,
  boss_monster_type_id INTEGER,
  boss_name TEXT,
  started_at_ms INTEGER NOT NULL,
  ended_at_ms INTEGER,
  total_damage INTEGER NOT NULL DEFAULT 0,
  hit_count INTEGER NOT NULL DEFAULT 0,
  FOREIGN KEY(encounter_id) REFERENCES encounters(id) ON DELETE CASCADE
);
CREATE INDEX IF NOT EXISTS idx_dungeon_segments_encounter ON dungeon_segments(encounter_id);

-- Buffs
CREATE TABLE IF NOT EXISTS buffs (
  encounter_id INTEGER NOT NULL,
  entity_id INTEGER NOT NULL,
  buff_id INTEGER NOT NULL,
  events TEXT NOT NULL,
  PRIMARY KEY (encounter_id, entity_id, buff_id),
  FOREIGN KEY(encounter_id) REFERENCES encounters(id) ON DELETE CASCADE
);

-- App config
CREATE TABLE IF NOT EXISTS app_config (
  key TEXT PRIMARY KEY NOT NULL,
  value TEXT NOT NULL
);

