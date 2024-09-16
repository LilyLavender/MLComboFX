use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::{lua_const::*, L2CValue, L2CAgent},
        hash40
    },
    smash_script::*,
    smashline::{*, Priority::*}
};

const FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_COMBO_EFFECT_SPAWNED : i32 = 0x200000E4;
const FIGHTER_MARIO_INSTANCE_WORK_ID_INT_COMBO_TIMER : i32 = 0x100000C0;
const FIGHTER_MARIO_INSTANCE_WORK_ID_INT_COMBO_COUNTER : i32 = 0x100000C1;

unsafe extern "C" fn mario_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = fighter.module_accessor;
        let status_kind = StatusModule::status_kind(boma);
        let status_frame = fighter.global_table[0xe].get_f32();
        let motion_kind = MotionModule::motion_kind(boma);
        
        // Decrease combo timer every frame
        if ![
            *FIGHTER_STATUS_KIND_CATCH_ATTACK,
            *FIGHTER_STATUS_KIND_CATCH_DASH,
            *FIGHTER_STATUS_KIND_CATCH_DASH_PULL,
            *FIGHTER_STATUS_KIND_CATCH_PULL,
            *FIGHTER_STATUS_KIND_CATCH_TURN,
            *FIGHTER_STATUS_KIND_CATCH_WAIT,
        ].contains(&status_kind) {
            WorkModule::dec_int(boma, FIGHTER_MARIO_INSTANCE_WORK_ID_INT_COMBO_TIMER);
        }
        if WorkModule::get_int(boma, FIGHTER_MARIO_INSTANCE_WORK_ID_INT_COMBO_TIMER) <= 0 {
            WorkModule::set_int(boma, 0, FIGHTER_MARIO_INSTANCE_WORK_ID_INT_COMBO_COUNTER);
        }

        // Spawn combo effect & sound
        if !WorkModule::is_flag(boma, FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_COMBO_EFFECT_SPAWNED) {
            if ([
                *FIGHTER_STATUS_KIND_AIR_LASSO,
                *FIGHTER_STATUS_KIND_APPEAL,
                *FIGHTER_STATUS_KIND_ATTACK,
                *FIGHTER_STATUS_KIND_ATTACK_100,
                *FIGHTER_STATUS_KIND_ATTACK_AIR,
                *FIGHTER_STATUS_KIND_ATTACK_DASH,
                *FIGHTER_STATUS_KIND_ATTACK_HI3,
                *FIGHTER_STATUS_KIND_ATTACK_HI4,
                *FIGHTER_STATUS_KIND_ATTACK_LW3,
                *FIGHTER_STATUS_KIND_ATTACK_LW4,
                *FIGHTER_STATUS_KIND_ATTACK_S3,
                *FIGHTER_STATUS_KIND_ATTACK_S4,
                *FIGHTER_STATUS_KIND_CLIFF_ATTACK,
                *FIGHTER_STATUS_KIND_LADDER_ATTACK,
                *FIGHTER_STATUS_KIND_SPECIAL_HI,
                *FIGHTER_STATUS_KIND_SPECIAL_LW,
                *FIGHTER_STATUS_KIND_SPECIAL_N,
                *FIGHTER_STATUS_KIND_SPECIAL_S,
            ].contains(&status_kind)
            && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT))
            || (motion_kind == hash40("throw_f") && status_frame >= 13.0) // 19.0 for luigi
            || (motion_kind == hash40("throw_b") && status_frame >= 44.0) // 19.0 for luigi
            || (motion_kind == hash40("throw_hi") && status_frame >= 18.0) 
            || (motion_kind == hash40("throw_lw") && status_frame >= 18.0) {
                WorkModule::on_flag(boma, FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_COMBO_EFFECT_SPAWNED);
                WorkModule::inc_int(boma, FIGHTER_MARIO_INSTANCE_WORK_ID_INT_COMBO_COUNTER);
                WorkModule::set_int(boma, 50, FIGHTER_MARIO_INSTANCE_WORK_ID_INT_COMBO_TIMER);

                let combo_counter = WorkModule::get_int(boma, FIGHTER_MARIO_INSTANCE_WORK_ID_INT_COMBO_COUNTER);
                if combo_counter == 2 {
                    macros::PLAY_SE(fighter, Hash40::new("se_mario_bros_combo_ok"));
                    macros::EFFECT(fighter, Hash40::new("bros_combo_ok"), Hash40::new("head"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
                } else if combo_counter == 3 {
                    macros::PLAY_SE(fighter, Hash40::new("se_mario_bros_combo_good"));
                    macros::EFFECT(fighter, Hash40::new("bros_combo_good"), Hash40::new("head"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
                } else if combo_counter == 4 {
                    macros::PLAY_SE(fighter, Hash40::new("se_mario_bros_combo_great"));
                    macros::EFFECT(fighter, Hash40::new("bros_combo_great"), Hash40::new("head"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
                } else if combo_counter >= 5 {
                    macros::PLAY_SE(fighter, Hash40::new("se_mario_bros_combo_excellent"));
                    macros::EFFECT(fighter, Hash40::new("bros_combo_excellent"), Hash40::new("head"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
                }
            }
            
        }
        
        // Reset effect spawned flag
        if status_frame <= 1.0 {
            WorkModule::off_flag(boma, FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_COMBO_EFFECT_SPAWNED)
        }
    }
}

pub fn install() {
    Agent::new("mario") // luigi
        .on_line(Main, mario_frame)
        .install();
}