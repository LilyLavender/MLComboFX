use super::*;
use std::{collections::HashMap, fs, path::Path};
use serde::Deserialize;
use toml;

#[derive(Debug, Deserialize)]
struct CharacterConfig {
    vfx_type: Option<String>,
    sfx_type: Option<String>,
    combo_frame: Option<i32>,
    cooldown_frame: Option<i32>,
    combo_hits: Option<Vec<i32>>,
}

#[derive(Debug, Default, Clone)]
pub struct CharacterData {
    pub vfx_type: String,
    pub sfx_type: String,
    pub combo_frame: i32,
    pub cooldown_frame: i32,
    pub combo_hits: Vec<i32>,
}

pub fn load_effect_config(config_path: &str) -> HashMap<String, CharacterData> {
    let mut character_map: HashMap<String, CharacterData> = HashMap::new();

    let content = match fs::read_to_string(config_path) {
        Ok(data) => data,
        Err(_) => {
            println!("[combo_fx] Failed to read file at {}", config_path);
            return character_map;
        }
    };

    let raw: toml::Value = match toml::from_str(&content) {
        Ok(v) => v,
        Err(e) => {
            println!("[combo_fx] Failed to parse TOML: {}", e);
            return character_map;
        }
    };

    if let toml::Value::Table(table) = raw {
        for (character_name, value) in table {
            if let toml::Value::Array(entries) = value {
                for entry in entries {
                    if let Ok(parsed) = entry.clone().try_into::<CharacterConfig>() {
                        let vfx_type = parsed.vfx_type.unwrap_or_default();
                        let sfx_type = parsed.sfx_type.unwrap_or_default();
                        if vfx_type.is_empty() && sfx_type.is_empty() {
                            continue;
                        }

                        let data = CharacterData {
                            vfx_type,
                            sfx_type,
                            combo_frame: parsed.combo_frame.unwrap_or(DEFAULT_COMBO_FRAME),
                            cooldown_frame: parsed.cooldown_frame.unwrap_or(DEFAULT_COOLDOWN_FRAME),
                            combo_hits: parsed.combo_hits.unwrap_or_else(|| DEFAULT_COMBO_HITS.to_vec()),
                        };

                        character_map.insert(character_name.clone(), data);
                    }
                }
            }
        }
    }

    println!("[combo_fx] Loaded effect configuration for {} character(s)", character_map.len());
    character_map
}