use super::*;
use skyline::nn::ro::LookupSymbol;
use skyline::hooks::{Region,getRegionAddress};
use skyline::libc::*;
use std::collections::HashMap;
use std::sync::RwLock;
use crate::data::*;

lazy_static! {
    pub static ref EFFECT_CONFIG: RwLock<HashMap<i32, CharacterData>> = RwLock::new(HashMap::new());
}

pub fn get_fighter_kind_from_string(target_kind: &str) -> i32 {
    match target_kind {
        "bayonetta" => *FIGHTER_KIND_BAYONETTA,
        "brave" => *FIGHTER_KIND_BRAVE,
        "buddy" => *FIGHTER_KIND_BUDDY,
        "captain" => *FIGHTER_KIND_CAPTAIN,
        "chrom" => *FIGHTER_KIND_CHROM,
        "cloud" => *FIGHTER_KIND_CLOUD,
        "daisy" => *FIGHTER_KIND_DAISY,
        "dedede" => *FIGHTER_KIND_DEDEDE,
        "demon" => *FIGHTER_KIND_DEMON,
        "diddy" => *FIGHTER_KIND_DIDDY,
        "dolly" => *FIGHTER_KIND_DOLLY,
        "donkey" => *FIGHTER_KIND_DONKEY,
        "duckhunt" => *FIGHTER_KIND_DUCKHUNT,
        "edge" => *FIGHTER_KIND_EDGE,
        "eflame" => *FIGHTER_KIND_EFLAME,
        "elight" => *FIGHTER_KIND_ELIGHT,
        "falco" => *FIGHTER_KIND_FALCO,
        "fox" => *FIGHTER_KIND_FOX,
        "gamewatch" => *FIGHTER_KIND_GAMEWATCH,
        "ganon" => *FIGHTER_KIND_GANON,
        "gaogaen" => *FIGHTER_KIND_GAOGAEN,
        "gekkouga" => *FIGHTER_KIND_GEKKOUGA,
        "ike" => *FIGHTER_KIND_IKE,
        "inkling" => *FIGHTER_KIND_INKLING,
        "jack" => *FIGHTER_KIND_JACK,
        "kamui" => *FIGHTER_KIND_KAMUI,
        "ken" => *FIGHTER_KIND_KEN,
        "kirby" => *FIGHTER_KIND_KIRBY,
        "koopa" => *FIGHTER_KIND_KOOPA,
        "koopajr" => *FIGHTER_KIND_KOOPAJR,
        "krool" => *FIGHTER_KIND_KROOL,
        "link" => *FIGHTER_KIND_LINK,
        "littlemac" => *FIGHTER_KIND_LITTLEMAC,
        "lucario" => *FIGHTER_KIND_LUCARIO,
        "lucas" => *FIGHTER_KIND_LUCAS,
        "lucina" => *FIGHTER_KIND_LUCINA,
        "luigi" => *FIGHTER_KIND_LUIGI,
        "mario" => *FIGHTER_KIND_MARIO,
        "mariod" => *FIGHTER_KIND_MARIOD,
        "marth" => *FIGHTER_KIND_MARTH,
        "master" => *FIGHTER_KIND_MASTER,
        "metaknight" => *FIGHTER_KIND_METAKNIGHT,
        "mewtwo" => *FIGHTER_KIND_MEWTWO,
        "miifighter" => *FIGHTER_KIND_MIIFIGHTER,
        "miigunner" => *FIGHTER_KIND_MIIGUNNER,
        "miiswordsman" => *FIGHTER_KIND_MIISWORDSMAN,
        "murabito" => *FIGHTER_KIND_MURABITO,
        "ness" => *FIGHTER_KIND_NESS,
        "packun" => *FIGHTER_KIND_PACKUN,
        "pacman" => *FIGHTER_KIND_PACMAN,
        "palutena" => *FIGHTER_KIND_PALUTENA,
        "peach" => *FIGHTER_KIND_PEACH,
        "pfushigisou" => *FIGHTER_KIND_PFUSHIGISOU,
        "pichu" => *FIGHTER_KIND_PICHU,
        "pickel" => *FIGHTER_KIND_PICKEL,
        "pikachu" => *FIGHTER_KIND_PIKACHU,
        "pikmin" => *FIGHTER_KIND_PIKMIN,
        "pit" => *FIGHTER_KIND_PIT,
        "pitb" => *FIGHTER_KIND_PITB,
        "plizardon" => *FIGHTER_KIND_PLIZARDON,
        "purin" => *FIGHTER_KIND_PURIN,
        "pzenigame" => *FIGHTER_KIND_PZENIGAME,
        "reflet" => *FIGHTER_KIND_REFLET,
        "richter" => *FIGHTER_KIND_RICHTER,
        "ridley" => *FIGHTER_KIND_RIDLEY,
        "robot" => *FIGHTER_KIND_ROBOT,
        "rockman" => *FIGHTER_KIND_ROCKMAN,
        "rosetta" => *FIGHTER_KIND_ROSETTA,
        "roy" => *FIGHTER_KIND_ROY,
        "ryu" => *FIGHTER_KIND_RYU,
        "samus" => *FIGHTER_KIND_SAMUS,
        "samusd" => *FIGHTER_KIND_SAMUSD,
        "sheik" => *FIGHTER_KIND_SHEIK,
        "shizue" => *FIGHTER_KIND_SHIZUE,
        "shulk" => *FIGHTER_KIND_SHULK,
        "simon" => *FIGHTER_KIND_SIMON,
        "snake" => *FIGHTER_KIND_SNAKE,
        "sonic" => *FIGHTER_KIND_SONIC,
        "szerosuit" => *FIGHTER_KIND_SZEROSUIT,
        "tantan" => *FIGHTER_KIND_TANTAN,
        "toonlink" => *FIGHTER_KIND_TOONLINK,
        "trail" => *FIGHTER_KIND_TRAIL,
        "wario" => *FIGHTER_KIND_WARIO,
        "wiifit" => *FIGHTER_KIND_WIIFIT,
        "wolf" => *FIGHTER_KIND_WOLF,
        "yoshi" => *FIGHTER_KIND_YOSHI,
        "younglink" => *FIGHTER_KIND_YOUNGLINK,
        "zelda" => *FIGHTER_KIND_ZELDA,
        _ => *FIGHTER_KIND_NONE,
    }
}

const FIGHTER_INSTANCE_WORK_ID_FLAG_COMBO_EFFECT_SPAWNED : i32 = 0x20000116;
const FIGHTER_INSTANCE_WORK_ID_INT_COMBO_TIMER : i32 = 0x1000010B;
const FIGHTER_INSTANCE_WORK_ID_INT_COMBO_COUNTER : i32 = 0x1000010C;

unsafe extern "C" fn combo_fighter_frame(fighter: &mut L2CFighterCommon) {
    let boma = fighter.module_accessor;
    let status_kind = StatusModule::status_kind(boma);
    let status_frame = fighter.global_table[0xe].get_f32();
    // let motion_kind = MotionModule::motion_kind(boma);
    let combo_timer = WorkModule::get_int(boma, FIGHTER_INSTANCE_WORK_ID_INT_COMBO_TIMER);
    
    // Decrease combo timer every frame
    let catch_statuses = [
        *FIGHTER_STATUS_KIND_CATCH_ATTACK,
        *FIGHTER_STATUS_KIND_CATCH_DASH,
        *FIGHTER_STATUS_KIND_CATCH_DASH_PULL,
        *FIGHTER_STATUS_KIND_CATCH_PULL,
        *FIGHTER_STATUS_KIND_CATCH_TURN,
        *FIGHTER_STATUS_KIND_CATCH_WAIT,
    ];
    if !catch_statuses.contains(&status_kind) {
        WorkModule::dec_int(boma, FIGHTER_INSTANCE_WORK_ID_INT_COMBO_TIMER);
    } else {
        // Decrease timer slower in grabs
        if status_frame % 3.0 == 0.0 {
            WorkModule::dec_int(boma, FIGHTER_INSTANCE_WORK_ID_INT_COMBO_TIMER);
        }
    }

    // Reset counter to 0 when timer expires
    if combo_timer <= 0 {
        WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_COMBO_COUNTER);
    }
    
    // Reset effect spawned flag after a few frames
    let kind = utility::get_kind(&mut *boma);
    let config = EFFECT_CONFIG.read().expect("Failed to read config");
    let max_combo_frame = config.get(&kind).map_or(75, |cfg| cfg.combo_frame);
    let cooldown_frames = config.get(&kind).map_or(16, |cfg| cfg.cooldown_frame);
    if combo_timer + cooldown_frames <= max_combo_frame {
        WorkModule::off_flag(boma, FIGHTER_INSTANCE_WORK_ID_FLAG_COMBO_EFFECT_SPAWNED);
    }
}

static mut NOTIFY_LOG_EVENT_COLLISION_HIT_OFFSET : usize = 0x67a7b0;

#[skyline::hook(offset = NOTIFY_LOG_EVENT_COLLISION_HIT_OFFSET)]
pub unsafe fn notify_log_event_collision_hit_replace(fighter_manager: *mut smash::app::FighterManager, attacker_id: u32, defender_id: u32, damage: f32, attack_kind: i32, damage_meteor: bool) -> u64 {
    let og = original!()(fighter_manager, attacker_id, defender_id, damage, attack_kind, damage_meteor);
    let attacker_boma = sv_battle_object::module_accessor(attacker_id);
    let defender_boma = sv_battle_object::module_accessor(defender_id);
    let attacker_category = utility::get_category(&mut *attacker_boma);
    let defender_category = utility::get_category(&mut *defender_boma);

    // Allowed categories
    let attacker_allowed_cats = [
        *BATTLE_OBJECT_CATEGORY_FIGHTER, // Fighter
        *BATTLE_OBJECT_CATEGORY_WEAPON, // Article
    ];
    let defender_allowed_cats = [
        *BATTLE_OBJECT_CATEGORY_FIGHTER, // Fighter
        *BATTLE_OBJECT_CATEGORY_ENEMY, // Stage hazard, etc?
        *BATTLE_OBJECT_CATEGORY_ITEM, // Assists, pokemon, probably the crates too
    ];
    let attacker_is_fighter = attacker_category == *BATTLE_OBJECT_CATEGORY_FIGHTER;

    // Handle attacker and defender not being the expected types
    if !attacker_allowed_cats.contains(&attacker_category)
    || !defender_allowed_cats.contains(&defender_category) {
        return og;
    }
    let owner_boma = if attacker_is_fighter {
        // Attacker is a fighter
        // attacker_boma = fighter's boma
        // owner_boma = fighter's boma
        attacker_boma
    } else {
        // Attacker is a projectile
        // attacker_boma = projectile's boma
        // owner_boma = fighter's boma
        sv_battle_object::module_accessor((WorkModule::get_int(attacker_boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32)
    };

    // Make sure character is allowed to have effects
    let attacker_kind = utility::get_kind(&mut *owner_boma);
    let config = EFFECT_CONFIG.read();
    if let Some(active_fighter) = config.as_ref().expect("Failed to acquire read lock").get(&attacker_kind) {
        if active_fighter.vfx_type == "" && active_fighter.sfx_type == "" {
            return og;
        }
    } else {
        return og;
    }

    // Clear counter when getting hit
    WorkModule::set_int(defender_boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_COMBO_COUNTER);
    WorkModule::set_int(defender_boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_COMBO_TIMER);

    // Successful hit, spawn effects & sound, set variables
    if !WorkModule::is_flag(owner_boma, FIGHTER_INSTANCE_WORK_ID_FLAG_COMBO_EFFECT_SPAWNED)
    && !WorkModule::is_flag(owner_boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL)
    && HitModule::get_whole(defender_boma, *HIT_STATUS_INVINCIBLE) == 0
    && damage > 1.0 {
        WorkModule::on_flag(owner_boma, FIGHTER_INSTANCE_WORK_ID_FLAG_COMBO_EFFECT_SPAWNED);
        WorkModule::inc_int(owner_boma, FIGHTER_INSTANCE_WORK_ID_INT_COMBO_COUNTER);
        let config_guard = EFFECT_CONFIG.read().expect("Failed to read config");
        let combo_frame = config_guard.get(&attacker_kind).map_or(0, |cfg| cfg.combo_frame);
        WorkModule::set_int(owner_boma, combo_frame, FIGHTER_INSTANCE_WORK_ID_INT_COMBO_TIMER);
        
        let vfx_type = config_guard.get(&attacker_kind).map(|cfg| cfg.vfx_type.clone()).unwrap_or_else(|| "".to_string());
        let vfx_type_short = match vfx_type.as_str() {
            "inside_story" => "bis",
            "color_splash" => "cs",
            "dream_team" => "dt",
            "superstar_saga" => "ssbm",
            "brothership" => "bro",
            _ => vfx_type.as_str(),
        };

        let sfx_type = config_guard.get(&attacker_kind).map(|cfg| cfg.sfx_type.clone()).unwrap_or_else(|| "".to_string());
        let sfx_type_short = match sfx_type.as_str() {
            "inside_story" => "bis",
            "color_splash" => "cs",
            "dream_team" => "dt",
            "superstar_saga" => "ssbm",
            "brothership" => "bro",
            _ => sfx_type.as_str(),
        };

        let combo_counter = WorkModule::get_int(owner_boma, FIGHTER_INSTANCE_WORK_ID_INT_COMBO_COUNTER);
        let pos = *PostureModule::pos(attacker_boma);
        let lr = PostureModule::lr(attacker_boma);
        let position_vector = Vector3f{ x: pos.x+8.0*lr, y: pos.y+12.0, z: pos.z };
        let zero_vector = Vector3f{x: 0.0, y: 0.0, z: 0.0};
        let hits = config_guard.get(&attacker_kind).map(|cfg| cfg.combo_hits.clone()).unwrap_or_else(|| DEFAULT_COMBO_HITS.to_vec());
        let combo_level = match combo_counter {
            x if x >= hits.get(3).cloned().unwrap_or(5) => "excellent",
            x if x >= hits.get(2).cloned().unwrap_or(4) => "great",
            x if x >= hits.get(1).cloned().unwrap_or(3) => "good",
            x if x >= hits.get(0).cloned().unwrap_or(2) => "ok",
            _ => "",
        };

        let final_sfx_str = format!("se_combo_{}_{}", sfx_type_short, combo_level);
        let final_vfx_str = format!("sys_combo_{}_{}", vfx_type_short, combo_level);
        let vfx_size = match sfx_type_short {
            "bis" => 1.3,
            "cs" => 1.2,
            "dt" => 1.18,
            "ssbm" => 1.2,
            "bro" => 1.5,
            _ => 1.0,
        };
        if combo_counter > hits.get(0).copied().unwrap_or(2) - 1 {
            SoundModule::play_se(owner_boma, Hash40::new(&final_sfx_str), true, false, false, false, enSEType(0));
            EffectModule::req(attacker_boma, Hash40::new(&final_vfx_str), &position_vector, &zero_vector, vfx_size, 0, -1, false, 0);
        }
    }

    og
}

fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|window| window == needle)
}

static OFFSET_SEARCH_CODE: &[u8] = &[
    0xff, 0x03, 0x03, 0xd1, //.text:0000007100675A20                 SUB             SP, SP, #0xC0
    0xe8, 0x2b, 0x00, 0xfd, //.text:0000007100675A24                 STR             D8, [SP,#0xB0+var_60]
    0xfc, 0x6f, 0x06, 0xa9, //.text:0000007100675A28                 STP             X28, X27, [SP,#0xB0+var_50]
    0xfa, 0x67, 0x07, 0xa9, //.text:0000007100675A2C                 STP             X26, X25, [SP,#0xB0+var_40]
    0xf8, 0x5f, 0x08, 0xa9, //.text:0000007100675A30                 STP             X24, X23, [SP,#0xB0+var_30]
    0xf6, 0x57, 0x09, 0xa9, //.text:0000007100675A34                 STP             X22, X21, [SP,#0xB0+var_20]
    0xf4, 0x4f, 0x0a, 0xa9, //.text:0000007100675A38                 STP             X20, X19, [SP,#0xB0+var_10]
    0xfd, 0x7b, 0x0b, 0xa9, //.text:0000007100675A3C                 STP             X29, X30, [SP,#0xB0+var_s0]
    0xfd, 0xc3, 0x02, 0x91, //.text:0000007100675A40                 ADD             X29, SP, #0xB0
    0xfb, 0x03, 0x00, 0xaa  //.text:0000007100675A44                 MOV             X27, X0
];

pub fn install() {
    let mut found_config: Option<HashMap<String, CharacterData>> = None;
    for entry in std::fs::read_dir("sd:/ultimate/mods").unwrap() {
        if let Ok(entry) = entry {
            let mut path = entry.path();
            if path.is_dir() {
                path.push("combo_fx.toml");

                if path.exists() {
                    found_config = Some(load_effect_config(path.to_str().unwrap()));
                }
            }
        }
    }

    let mut global_map = EFFECT_CONFIG.write().expect("Lock failed on EFFECT_CONFIG");
    if let Some(config_map) = found_config {
        *global_map = config_map
            .into_iter()
            .map(|(name, data)| (get_fighter_kind_from_string(&name), data))
            .collect();
    }

    unsafe {
        let text_ptr = getRegionAddress(Region::Text) as *const u8;
        let text_size = (getRegionAddress(Region::Rodata) as usize) - (text_ptr as usize);
        let text = std::slice::from_raw_parts(text_ptr, text_size);
        if let Some(offset) = find_subsequence(text, OFFSET_SEARCH_CODE) {
            NOTIFY_LOG_EVENT_COLLISION_HIT_OFFSET = offset;
        }
    }

    skyline::install_hook!(
        notify_log_event_collision_hit_replace
    );
    Agent::new("fighter")
        .on_line(Main, combo_fighter_frame)
        .install();
}