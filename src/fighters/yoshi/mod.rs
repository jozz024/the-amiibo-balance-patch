use smash::app::sv_animcmd::*;
use smash::phx::Hash40;

use smash::lib::lua_const::*;
use smash::lua2cpp::*;
use smash::app::lua_bind::{AttackModule, WorkModule, JostleModule};
use smashline::*;
use smash_script::*;

#[acmd_script(
    agent = "yoshi",
    script = "game_attackairf",
    category = ACMD_GAME
)]
unsafe fn yoshi_fair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 4.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 16.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("head"), 15.0, 361, 90, 0, 0, 4.2, 1.5, -0.5, -0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("head"), 14.0, 270, 90, 0, 0, 4.0, 7.0, 1.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);
    }
    wait(lua_state, 5.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 40.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(boma, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 67.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    macros::FT_MOTION_RATE(fighter, 0.2);
}


#[acmd_script(
    agent = "yoshi",
    script = "game_specialsend",
    category = ACMD_GAME
)]
unsafe fn yoshi_sideb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
        JostleModule::set_status(boma, true);
    }
    frame(lua_state, 13.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_YOSHI_STATUS_SPECIAL_S_FLAG_HIDE_EGG);
    }
    macros::FT_MOTION_RATE(fighter, 0.2);
}

pub fn install() {
    smashline::install_acmd_scripts!(yoshi_fair, yoshi_sideb);
}