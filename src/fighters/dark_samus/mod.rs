use smash::app::sv_animcmd::*;
use smash::phx::Hash40;

use smash::lib::lua_const::*;
use smash::lua2cpp::*;
use smash::app::lua_bind::{AttackModule, WorkModule, VisibilityModule};
use smashline::*;
use smash_script::*;

use std::convert::TryInto;

#[acmd_script(
    agent = "samusd",
    script = "game_attacks4",
    category = ACMD_GAME
)]
unsafe fn dark_samus_fsmash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 4.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 10.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 12.0, 361, 122, 0, 26, 3.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 12.0, 361, 122, 0, 26, 3.3, -4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 2, 0, Hash40::new("armr"), 14.0, 361, 100, 0, 40, 4.5, 7.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PUNCH);
    }
    wait(lua_state, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}


#[acmd_script(
    agent = "samusd",
    script = "game_attacklw4",
    category = ACMD_GAME
)]
unsafe fn dark_samus_dsmash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 4.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 9.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 10.0, 30, 70, 0, 90, 4.2, 8.0, -0.5, 0.0, Some(-2.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(boma, smash::app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(lua_state, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 17.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 12.0, 30, 68, 0, 90, 4.6, 8.0, -0.5, 0.0, Some(-2.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(boma, smash::app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(lua_state, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script(
    agent="samusd",
    script="game_attacks3",
    category=ACMD_GAME
)]
unsafe fn dark_samus_ftilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 8.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("hip"), 11.0, 361, 80, 0, 30, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 12.0, 361, 100, 0, 30, 3.0, 0.6, 0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 2, 0, Hash40::new("kneer"), 13.0, 361, 100, 0, 30, 3.0, 6.0, -0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(lua_state, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script(
    agent="samusd",
    script="game_speciallw",
    category=ACMD_GAME
)]
unsafe fn dark_samus_downb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 4.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_JUMP);
    }
    frame(lua_state, 10.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_JUMP);
    }
    frame(lua_state, 11.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_WEAPON);
        VisibilityModule::set_int64(boma, Hash40::new("body").hash.try_into().unwrap(), Hash40::new("body_sphere").hash.try_into().unwrap());
        WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_MV);
    }
    frame(lua_state, 44.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(boma, /*Flag*/ *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_MV);
    }
    frame(lua_state, 45.0);
    if macros::is_excute(fighter) {
        VisibilityModule::set_int64(boma, Hash40::new("body").hash.try_into().unwrap(), Hash40::new("body_normal").hash.try_into().unwrap());
    }
    frame(lua_state, 45.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_CHK_CROUCH);
    }
    macros::FT_MOTION_RATE(fighter, 0.2);
}
pub fn install() {
    smashline::install_acmd_scripts!(
        dark_samus_fsmash,
        dark_samus_dsmash,
        dark_samus_ftilt,
        dark_samus_downb
    );
}