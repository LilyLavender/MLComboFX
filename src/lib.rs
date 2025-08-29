#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    unused_imports,
	unused_assignments,
	non_upper_case_globals,
	non_snake_case,
    clippy::borrow_interior_mutable_const
)]

#[macro_use]
extern crate lazy_static;

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

pub static DEFAULT_EFFECT_TYPE: &str = "";
pub static DEFAULT_COMBO_FRAME: i32 = -1;
pub static DEFAULT_COOLDOWN_FRAME: i32 = 16;
pub static DEFAULT_COMBO_HITS: [i32; 4] = [2, 3, 4, 5];

mod frame;
mod data;

#[skyline::main(name = "ml_combo_fx")]
pub fn main() {
    frame::install();
}