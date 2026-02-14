use once_cell::sync::Lazy;
use parking_lot::RwLock;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

const BUFF_JSON_RELATIVE: &str = "meter-data/BuffName.json";
const TALENT_JSON_RELATIVE: &str = "meter-data/Talent.json";

#[derive(Debug, Deserialize)]
struct RawBuffEntry {
    #[serde(rename = "Id")]
    id: i32,
    #[serde(rename = "Icon")]
    icon: Option<String>,
    #[serde(rename = "NameDesign")]
    name: Option<String>,
    #[serde(rename = "SpriteFile")]
    sprite_file: Option<String>,
}

#[derive(Debug, Deserialize)]
struct RawTalentEntry {
    id: i32,
    #[serde(rename = "BuffIds")]
    buff_ids: Vec<i32>,
    #[serde(rename = "Des")]
    des: String,
    #[serde(rename = "TalentIcon")]
    talent_icon: String,
    #[serde(rename = "SpriteFile")]
    sprite_file: String,
}

#[derive(Debug, Clone)]
pub struct BuffNameEntry {
    pub name: String,
    pub icon: String,
    pub sprite_file: Option<String>,
    pub talent_name: Option<String>,
    pub talent_sprite_file: Option<String>,
}

#[derive(Debug, Clone)]
pub struct BuffSpriteEntry {
    pub base_id: i32,
    pub name: String,
    pub sprite_file: String,
    pub talent_name: Option<String>,
    pub talent_sprite_file: Option<String>,
}

/// Cache stores buff metadata keyed by buff id.
static BUFF_CACHE: Lazy<RwLock<HashMap<i32, BuffNameEntry>>> = Lazy::new(|| {
    let map = load_buff_names().unwrap_or_default();
    RwLock::new(map)
});

/// Maps talent BuffIds to their related effect base_ids.
static TALENT_SOURCE_MAP: Lazy<RwLock<HashMap<i32, Vec<i32>>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

fn locate_buff_file() -> Option<PathBuf> {
    // Try relative path first
    let mut p = PathBuf::from(BUFF_JSON_RELATIVE);
    if p.exists() {
        return Some(p);
    }

    // Try src-tauri prefixed
    p = PathBuf::from(format!("src-tauri/{}", BUFF_JSON_RELATIVE));
    if p.exists() {
        return Some(p);
    }

    // Try exe dir
    if let Ok(mut exe_dir) = std::env::current_exe() {
        exe_dir.pop();
        let candidate = exe_dir.join(BUFF_JSON_RELATIVE);
        if candidate.exists() {
            return Some(candidate);
        }
    }

    None
}

fn locate_talent_file() -> Option<PathBuf> {
    // Try relative path first
    let mut p = PathBuf::from(TALENT_JSON_RELATIVE);
    if p.exists() {
        return Some(p);
    }

    // Try src-tauri prefixed
    p = PathBuf::from(format!("src-tauri/{}", TALENT_JSON_RELATIVE));
    if p.exists() {
        return Some(p);
    }

    // Try exe dir
    if let Ok(mut exe_dir) = std::env::current_exe() {
        exe_dir.pop();
        let candidate = exe_dir.join(TALENT_JSON_RELATIVE);
        if candidate.exists() {
            return Some(candidate);
        }
    }

    None
}

fn load_talent_entries() -> Result<Vec<RawTalentEntry>, Box<dyn std::error::Error>> {
    let path = match locate_talent_file() {
        Some(p) => p,
        None => return Ok(Vec::new()),
    };
    let contents = fs::read_to_string(path)?;
    let entries: Vec<RawTalentEntry> = serde_json::from_str(&contents)?;
    Ok(entries)
}

fn build_talent_associations(
    buff_map: &mut HashMap<i32, BuffNameEntry>,
) -> HashMap<i32, Vec<i32>> {
    let talents = load_talent_entries().unwrap_or_default();
    let mut source_map: HashMap<i32, Vec<i32>> = HashMap::new();

    for talent in talents {
        if talent.buff_ids.is_empty() {
            continue;
        }
        for buff_id in talent.buff_ids {
            let mut related_ids = Vec::new();
            for offset in -5i32..=5 {
                let candidate = buff_id + offset;
                if let Some(entry) = buff_map.get(&candidate) {
                    if entry.sprite_file.is_some() {
                        related_ids.push(candidate);
                    }
                }
            }

            if related_ids.is_empty() {
                continue;
            }

            for &related_id in &related_ids {
                if let Some(entry) = buff_map.get_mut(&related_id) {
                    if entry.talent_name.is_none() {
                        entry.talent_name = Some(talent.des.clone());
                        if !talent.sprite_file.is_empty() {
                            entry.talent_sprite_file = Some(talent.sprite_file.clone());
                        }
                    }
                }
            }

            source_map.insert(buff_id, related_ids);
        }
    }

    source_map
}

fn load_buff_names() -> Result<HashMap<i32, BuffNameEntry>, Box<dyn std::error::Error>> {
    let path = match locate_buff_file() {
        Some(p) => p,
        None => return Ok(HashMap::new()),
    };

    let contents = fs::read_to_string(path)?;
    let entries: Vec<RawBuffEntry> = serde_json::from_str(&contents)?;
    let mut buff_map: HashMap<i32, BuffNameEntry> = HashMap::new();

    for entry in entries {
        let name = entry.name.unwrap_or_default();
        if name.is_empty() {
            continue;
        }
        let icon = entry.icon.unwrap_or_default();
        let sprite_file = entry.sprite_file.and_then(|v| if v.is_empty() { None } else { Some(v) });
        buff_map.insert(
            entry.id,
            BuffNameEntry {
                name,
                icon,
                sprite_file,
                talent_name: None,
                talent_sprite_file: None,
            },
        );
    }

    let source_map = build_talent_associations(&mut buff_map);
    let mut talent_cache = TALENT_SOURCE_MAP.write();
    *talent_cache = source_map;

    Ok(buff_map)
}

/// Returns the display name for the given buff id.
#[allow(dead_code)]
pub fn lookup_name(buff_id: i32) -> Option<String> {
    let cache = BUFF_CACHE.read();
    cache.get(&buff_id).map(|entry| entry.name.clone())
}

/// Returns true when the buff exists in the cache.
pub fn is_valid(buff_id: i32) -> bool {
    let cache = BUFF_CACHE.read();
    cache.contains_key(&buff_id)
}

/// Returns sprite file name when available.
pub fn lookup_sprite(buff_id: i32) -> Option<String> {
    let cache = BUFF_CACHE.read();
    cache.get(&buff_id).and_then(|entry| entry.sprite_file.clone())
}

/// Returns all buffs that have a sprite file for selection.
pub fn get_buffs_with_sprites() -> Vec<BuffSpriteEntry> {
    let cache = BUFF_CACHE.read();
    let mut result: Vec<BuffSpriteEntry> = cache
        .iter()
        .filter_map(|(id, entry)| {
            entry
                .sprite_file
                .as_ref()
                .map(|sprite| BuffSpriteEntry {
                    base_id: *id,
                    name: entry.name.clone(),
                    sprite_file: sprite.clone(),
                    talent_name: entry.talent_name.clone(),
                    talent_sprite_file: entry.talent_sprite_file.clone(),
                })
        })
        .collect();
    result.sort_by_key(|entry| entry.base_id);
    result
}

/// Searches buffs by name/talent name and returns matching base ids.
pub fn search_buffs_by_name(keyword: &str, limit: usize) -> Vec<(i32, BuffNameEntry)> {
    let needle = keyword.trim().to_lowercase();
    if needle.is_empty() {
        return Vec::new();
    }

    let cache = BUFF_CACHE.read();
    let mut result: Vec<(i32, BuffNameEntry)> = cache
        .iter()
        .filter_map(|(id, entry)| {
            let name_hit = entry.name.to_lowercase().contains(&needle);
            let talent_hit = entry
                .talent_name
                .as_ref()
                .map(|v| v.to_lowercase().contains(&needle))
                .unwrap_or(false);
            if name_hit || talent_hit {
                Some((*id, entry.clone()))
            } else {
                None
            }
        })
        .collect();

    result.sort_by_key(|(id, _)| *id);
    if result.len() > limit {
        result.truncate(limit);
    }
    result
}

/// Returns related buff base ids for a given source config id.
pub fn get_related_base_ids(source_config_id: i32) -> Vec<i32> {
    let map = TALENT_SOURCE_MAP.read();
    map.get(&source_config_id).cloned().unwrap_or_default()
}
/// Reload the cache from disk.
#[allow(dead_code)]
pub fn reload_cache() -> Result<(), Box<dyn std::error::Error>> {
    let new_map = load_buff_names()?;
    let mut cache = BUFF_CACHE.write();
    *cache = new_map;
    Ok(())
}

