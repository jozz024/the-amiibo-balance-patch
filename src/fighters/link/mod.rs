use smash::app::sv_animcmd::*;
use smash::phx::Hash40;

use smash::lib::lua_const::*;
use smash::lua2cpp::*;
use smash::app::lua_bind::{AttackModule, WorkModule, ArticleModule, DamageModule};
use smashline::*;
use smash_script::*;


#[acmd_script(
    agent = "link",
    script =  "game_attackhi4",
    category = ACMD_GAME)]
unsafe fn link_usmash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 6.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 10.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("colonells"), 4.0, 105, 100, 55, 0, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("sword2"), 4.0, 115, 100, 38, 0, 4.8, 2.5, 0.0, -3.5, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("sword2"), 4.0, 107, 100, 28, 0, 4.8, 2.5, 0.0, -3.5, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 3, 0, Hash40::new("sword2"), 4.0, 115, 100, 28, 0, 3.0, 8.5, 0.0, -2.2, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 4, 0, Hash40::new("sword2"), 4.0, 107, 100, 28, 0, 3.0, 8.5, 0.0, -2.2, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame_revised(boma, 0, 3.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 1, 3.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 2, 3.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 3, 3.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 4, 3.0, false);
    }
    wait(lua_state, 4.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 3, 0, Hash40::new("sword2"), 4.0, 135, 100, 45, 0, 3.0, 8.5, 0.0, -2.2, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 25.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("colonells"), 3.0, 105, 100, 55, 0, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("sword2"), 3.0, 115, 100, 38, 0, 4.8, 2.5, 0.0, -3.5, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("sword2"), 3.0, 107, 100, 28, 0, 4.8, 2.5, 0.0, -3.5, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 3, 0, Hash40::new("sword2"), 3.0, 115, 100, 28, 0, 3.0, 8.5, 0.0, -2.2, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 4, 0, Hash40::new("sword2"), 3.0, 107, 100, 28, 0, 3.0, 8.5, 0.0, -2.2, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame_revised(boma, 0, 3.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 1, 3.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 2, 3.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 3, 3.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 4, 3.0, false);
    }
    wait(lua_state, 4.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 3, 0, Hash40::new("sword2"), 3.0, 135, 100, 40, 0, 3.0, 8.5, 0.0, -2.2, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 41.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("sword2"), 11.0, 80, 96, 0, 55, 4.3, 7.0, 0.0, -3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("sword2"), 11.0, 90, 96, 0, 55, 4.3, 1.0, 0.0, -3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("armr"), 10.0, 90, 96, 0, 55, 3.0, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 9.0, 90, 96, 0, 55, 5.2, 0.0, 11.0, -0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 5.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script(
    agent = "link",
    script = "game_attacks4",
    category = ACMD_GAME)]
unsafe fn link_fsmash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 10.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    macros::FT_MOTION_RATE(fighter, 1.5);
    frame(lua_state, 14.0);
    macros::FT_MOTION_RATE(fighter, 1);
    frame(lua_state, 15.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("sword2"), 7.0, 69, 12, 0, 45, 4.3, 2.7, -2.0, -2.3, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("sword2"), 14.0, 361, 100, 0, 30, 3.5, 8.5, -2.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 7.0, 60, 12, 0, 45, 3.2, 0.0, 8.5, 6.0, Some(0.0), Some(8.5), Some(3.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        let damage = DamageModule::damage(boma, 0);
        if damage == 0.0 {
            ArticleModule::generate_article(boma, *FIGHTER_LINK_GENERATE_ARTICLE_SWORD_BEAM, false, 0);
        }
    }
    wait(lua_state, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    wait(lua_state, 3.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    frame(lua_state, 35.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(boma, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
}


pub fn install() {
    smashline::install_acmd_scripts!(
        link_usmash,
        link_fsmash
    );
}