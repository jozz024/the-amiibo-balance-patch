use smash::app::sv_animcmd::*;
use smash::phx::Hash40;

use smash::app::lua_bind::{
    ArticleModule, AttackModule, FighterAreaModuleImpl, MotionModule, WorkModule,
};
use smash::app::*;
use smash::lib::lua_const::*;
use smash::lua2cpp::*;
use smash_script::*;
use smashline::*;
use std::arch::asm;

#[acmd_script(
    agent = "mario",
    script =  "game_attacklw4",
    category = ACMD_GAME)]
unsafe fn mario_dsmash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(lua_state, 3.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(
            fighter.module_accessor,
            /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD,
        );
    }
    frame(lua_state, 5.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(
            fighter,
            /*ID*/ 0,
            /*Part*/ 0,
            /*Bone*/ Hash40::new("top"),
            /*Damage*/ 12.0,
            /*Angle*/ 32,
            /*KBG*/ 100,
            /*FKB*/ 0,
            /*BKB*/ 45,
            /*Size*/ 4.0,
            /*X*/ 0.0,
            /*Y*/ 3.6,
            /*Z*/ 12.5,
            /*X2*/ None,
            /*Y2*/ None,
            /*Z2*/ None,
            /*Hitlag*/ 1.0,
            /*SDI*/ 1.0,
            /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON,
            /*FacingRestrict*/ *ATTACK_LR_CHECK_POS,
            /*SetWeight*/ false,
            /*ShieldDamage*/ 0,
            /*Trip*/ 0.0,
            /*Rehit*/ 0,
            /*Reflectable*/ false,
            /*Absorbable*/ false,
            /*Flinchless*/ false,
            /*DisableHitlag*/ false,
            /*Direct_Hitbox*/ true,
            /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA,
            /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL,
            /*CollisionPart*/ *COLLISION_PART_MASK_ALL,
            /*FriendlyFire*/ false,
            /*Effect*/ Hash40::new("collision_attr_normal"),
            /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L,
            /*SFXType*/ *COLLISION_SOUND_ATTR_KICK,
            /*Type*/ *ATTACK_REGION_KICK,
        );
        macros::ATTACK(
            fighter,
            /*ID*/ 1,
            /*Part*/ 0,
            /*Bone*/ Hash40::new("top"),
            /*Damage*/ 12.0,
            /*Angle*/ 32,
            /*KBG*/ 100,
            /*FKB*/ 0,
            /*BKB*/ 45,
            /*Size*/ 3.3,
            /*X*/ 0.0,
            /*Y*/ 3.6,
            /*Z*/ 7.0,
            /*X2*/ None,
            /*Y2*/ None,
            /*Z2*/ None,
            /*Hitlag*/ 1.0,
            /*SDI*/ 1.0,
            /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON,
            /*FacingRestrict*/ *ATTACK_LR_CHECK_POS,
            /*SetWeight*/ false,
            /*ShieldDamage*/ 0,
            /*Trip*/ 0.0,
            /*Rehit*/ 0,
            /*Reflectable*/ false,
            /*Absorbable*/ false,
            /*Flinchless*/ false,
            /*DisableHitlag*/ false,
            /*Direct_Hitbox*/ true,
            /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA,
            /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL,
            /*CollisionPart*/ *COLLISION_PART_MASK_ALL,
            /*FriendlyFire*/ false,
            /*Effect*/ Hash40::new("collision_attr_normal"),
            /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L,
            /*SFXType*/ *COLLISION_SOUND_ATTR_KICK,
            /*Type*/ *ATTACK_REGION_KICK,
        );
        AttackModule::set_attack_height_all(
            fighter.module_accessor,
            AttackHeight(*ATTACK_HEIGHT_LOW),
            false,
        );
    }
    wait(lua_state, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(lua_state, 14.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(
            fighter,
            /*ID*/ 0,
            /*Part*/ 0,
            /*Bone*/ Hash40::new("top"),
            /*Damage*/ 14.0,
            /*Angle*/ 30,
            /*KBG*/ 100,
            /*FKB*/ 0,
            /*BKB*/ 45,
            /*Size*/ 4.0,
            /*X*/ 0.0,
            /*Y*/ 3.6,
            /*Z*/ -11.5,
            /*X2*/ None,
            /*Y2*/ None,
            /*Z2*/ None,
            /*Hitlag*/ 1.0,
            /*SDI*/ 1.0,
            /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON,
            /*FacingRestrict*/ *ATTACK_LR_CHECK_POS,
            /*SetWeight*/ false,
            /*ShieldDamage*/ 0,
            /*Trip*/ 0.0,
            /*Rehit*/ 0,
            /*Reflectable*/ false,
            /*Absorbable*/ false,
            /*Flinchless*/ false,
            /*DisableHitlag*/ false,
            /*Direct_Hitbox*/ true,
            /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA,
            /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL,
            /*CollisionPart*/ *COLLISION_PART_MASK_ALL,
            /*FriendlyFire*/ false,
            /*Effect*/ Hash40::new("collision_attr_normal"),
            /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M,
            /*SFXType*/ *COLLISION_SOUND_ATTR_KICK,
            /*Type*/ *ATTACK_REGION_KICK,
        );
        macros::ATTACK(
            fighter,
            /*ID*/ 1,
            /*Part*/ 0,
            /*Bone*/ Hash40::new("top"),
            /*Damage*/ 14.0,
            /*Angle*/ 30,
            /*KBG*/ 100,
            /*FKB*/ 0,
            /*BKB*/ 45,
            /*Size*/ 3.3,
            /*X*/ 0.0,
            /*Y*/ 3.6,
            /*Z*/ -6.0,
            /*X2*/ None,
            /*Y2*/ None,
            /*Z2*/ None,
            /*Hitlag*/ 1.0,
            /*SDI*/ 1.0,
            /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON,
            /*FacingRestrict*/ *ATTACK_LR_CHECK_POS,
            /*SetWeight*/ false,
            /*ShieldDamage*/ 0,
            /*Trip*/ 0.0,
            /*Rehit*/ 0,
            /*Reflectable*/ false,
            /*Absorbable*/ false,
            /*Flinchless*/ false,
            /*DisableHitlag*/ false,
            /*Direct_Hitbox*/ true,
            /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA,
            /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL,
            /*CollisionPart*/ *COLLISION_PART_MASK_ALL,
            /*FriendlyFire*/ false,
            /*Effect*/ Hash40::new("collision_attr_normal"),
            /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M,
            /*SFXType*/ *COLLISION_SOUND_ATTR_KICK,
            /*Type*/ *ATTACK_REGION_KICK,
        );
        AttackModule::set_attack_height_all(
            fighter.module_accessor,
            AttackHeight(*ATTACK_HEIGHT_LOW),
            false,
        );
    }
    wait(lua_state, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script(
    agent = "mario",
    script =  "game_attackdash",
    category = ACMD_GAME)]
unsafe fn mario_dashatk(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(lua_state, 6.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(
            fighter,
            /*ID*/ 0,
            /*Part*/ 0,
            /*Bone*/ Hash40::new("top"),
            /*Damage*/ 8.0,
            /*Angle*/ 14,
            /*KBG*/ 30,
            /*FKB*/ 0,
            /*BKB*/ 65,
            /*Size*/ 4.5,
            /*X*/ 0.0,
            /*Y*/ 2.5,
            /*Z*/ 5.4,
            /*X2*/ None,
            /*Y2*/ None,
            /*Z2*/ None,
            /*Hitlag*/ 1.25,
            /*SDI*/ 1.0,
            /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON,
            /*FacingRestrict*/ *ATTACK_LR_CHECK_F,
            /*SetWeight*/ false,
            /*ShieldDamage*/ 1,
            /*Trip*/ 0.0,
            /*Rehit*/ 0,
            /*Reflectable*/ false,
            /*Absorbable*/ false,
            /*Flinchless*/ false,
            /*DisableHitlag*/ false,
            /*Direct_Hitbox*/ true,
            /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA,
            /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL,
            /*CollisionPart*/ *COLLISION_PART_MASK_ALL,
            /*FriendlyFire*/ false,
            /*Effect*/ Hash40::new("collision_attr_normal"),
            /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M,
            /*SFXType*/ *COLLISION_SOUND_ATTR_KICK,
            /*Type*/ *ATTACK_REGION_KICK,
        );
        macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 0, /*ShieldstunMul*/ 0.875);
        FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor, 4.0, 6.0);
    }
    wait(lua_state, 4.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(
            fighter,
            /*ID*/ 0,
            /*Part*/ 0,
            /*Bone*/ Hash40::new("top"),
            /*Damage*/ 6.0,
            /*Angle*/ 14,
            /*KBG*/ 35,
            /*FKB*/ 0,
            /*BKB*/ 70,
            /*Size*/ 2.7,
            /*X*/ 0.0,
            /*Y*/ 2.5,
            /*Z*/ 4.9,
            /*X2*/ None,
            /*Y2*/ None,
            /*Z2*/ None,
            /*Hitlag*/ 1.25,
            /*SDI*/ 1.0,
            /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON,
            /*FacingRestrict*/ *ATTACK_LR_CHECK_F,
            /*SetWeight*/ false,
            /*ShieldDamage*/ 1,
            /*Trip*/ 0.0,
            /*Rehit*/ 0,
            /*Reflectable*/ false,
            /*Absorbable*/ false,
            /*Flinchless*/ false,
            /*DisableHitlag*/ false,
            /*Direct_Hitbox*/ true,
            /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA,
            /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL,
            /*CollisionPart*/ *COLLISION_PART_MASK_ALL,
            /*FriendlyFire*/ false,
            /*Effect*/ Hash40::new("collision_attr_normal"),
            /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S,
            /*SFXType*/ *COLLISION_SOUND_ATTR_KICK,
            /*Type*/ *ATTACK_REGION_KICK,
        );
        macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, /*ID*/ 0, /*ShieldstunMul*/ 0.875);
    }
    wait(lua_state, 16.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(lua_state, 32.0);
    if macros::is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor, 4.0, 4.0);
    }
    frame(lua_state, 41.0);
    if macros::is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor, 3.0, 3.0);
    }
}

#[acmd_script(
    agent = "mario",
    script = "game_specials",
    category = ACMD_GAME
)]
unsafe fn mario_sideb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_MARIO_GENERATE_ARTICLE_MANTLE, false, -1);
        notify_event_msc_cmd!(
            fighter,
            Hash40::new_raw(0x2127e37c07),
            *GROUND_CLIFF_CHECK_KIND_ALWAYS
        );
    }
    frame(lua_state, 6.0);
    if macros::is_excute(fighter) {
        macros::SEARCH(
            fighter,
            0,
            0,
            Hash40::new("top"),
            8.0,
            0.0,
            6.5,
            2.5,
            Some(0.0),
            Some(6.5),
            Some(8.0),
            *COLLISION_KIND_MASK_ATTACK,
            *HIT_STATUS_MASK_NORMAL,
            60,
            *COLLISION_SITUATION_MASK_GA,
            *COLLISION_CATEGORY_MASK_ALL,
            *COLLISION_PART_MASK_ALL,
            false,
        );
        WorkModule::set_float(
            boma,
            9.0,
            *FIGHTER_MARIO_STATUS_SPECIAL_S_WORK_ID_FLOAT_REFLECT_MOTION_FRAME,
        );
    }
    frame(lua_state, 9.0);
    if macros::is_excute(fighter) {
        search!(fighter, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
        shield!(
            fighter,
            *MA_MSC_CMD_SHIELD_ON,
            *COLLISION_KIND_REFLECTOR,
            *FIGHTER_MARIO_REFLECTOR_KIND_MANTLE,
            *FIGHTER_REFLECTOR_GROUP_EXTEND
        );
    }
    frame(lua_state, 12.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(
            boma,
            /*Flag*/ *FIGHTER_MARIO_STATUS_SPECIAL_S_FLAG_SPECIAL_FALL,
        );
        macros::ATTACK(
            fighter,
            0,
            0,
            Hash40::new("top"),
            7.0,
            110,
            100,
            80,
            0,
            7.5,
            0.0,
            6.7,
            9.7,
            None,
            None,
            None,
            1.0,
            1.0,
            *ATTACK_SETOFF_KIND_OFF,
            *ATTACK_LR_CHECK_F,
            false,
            4,
            0.0,
            0,
            false,
            false,
            false,
            false,
            true,
            *COLLISION_SITUATION_MASK_GA,
            *COLLISION_CATEGORY_MASK_ALL,
            *COLLISION_PART_MASK_ALL,
            false,
            Hash40::new("collision_attr_bind"),
            *ATTACK_SOUND_LEVEL_M,
            *COLLISION_SOUND_ATTR_MARIO_MANT,
            *ATTACK_REGION_OBJECT,
        );
        macros::ATTACK(
            fighter,
            1,
            0,
            Hash40::new("top"),
            7.0,
            110,
            100,
            80,
            0,
            5.0,
            0.0,
            6.7,
            5.0,
            None,
            None,
            None,
            1.0,
            1.0,
            *ATTACK_SETOFF_KIND_OFF,
            *ATTACK_LR_CHECK_F,
            false,
            4,
            0.0,
            0,
            false,
            false,
            false,
            false,
            true,
            *COLLISION_SITUATION_MASK_GA,
            *COLLISION_CATEGORY_MASK_ALL,
            *COLLISION_PART_MASK_ALL,
            false,
            Hash40::new("collision_attr_bind"),
            *ATTACK_SOUND_LEVEL_M,
            *COLLISION_SOUND_ATTR_MARIO_MANT,
            *ATTACK_REGION_OBJECT,
        );
    }
    MotionModule::set_rate(boma, 1.3);
    frame(lua_state, 15.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(
            fighter,
            Hash40::new_raw(0x2127e37c07),
            *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES
        );
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 21.0);
    if macros::is_excute(fighter) {
        shield!(
            fighter,
            *MA_MSC_CMD_SHIELD_OFF,
            *COLLISION_KIND_REFLECTOR,
            *FIGHTER_MARIO_REFLECTOR_KIND_MANTLE,
            *FIGHTER_REFLECTOR_GROUP_EXTEND
        );
        WorkModule::off_flag(
            boma,
            /*Flag*/ *FIGHTER_MARIO_STATUS_SPECIAL_S_FLAG_SPECIAL_FALL,
        );
    }
}

#[acmd_script(
    agent = "mario_fireball",
    script =  "game_regular",
    category = ACMD_GAME)]
unsafe fn mario_fireball(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    if macros::is_excute(fighter) {
        macros::ATTACK(
            fighter,
            /*ID*/ 0,
            /*Part*/ 0,
            /*Bone*/ Hash40::new("top"),
            /*Damage*/ 5.0,
            /*Angle*/ 361,
            /*KBG*/ 70,
            /*FKB*/ 0,
            /*BKB*/ 35,
            /*Size*/ 2.4,
            /*X*/ 0.0,
            /*Y*/ 0.0,
            /*Z*/ 0.0,
            /*X2*/ None,
            /*Y2*/ None,
            /*Z2*/ None,
            /*Hitlag*/ 0.6,
            /*SDI*/ 1.0,
            /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON,
            /*FacingRestrict*/ *ATTACK_LR_CHECK_SPEED,
            /*SetWeight*/ false,
            /*ShieldDamage*/ -2.5,
            /*Trip*/ 0.0,
            /*Rehit*/ 0,
            /*Reflectable*/ true,
            /*Absorbable*/ true,
            /*Flinchless*/ false,
            /*DisableHitlag*/ false,
            /*Direct_Hitbox*/ false,
            /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA,
            /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL,
            /*CollisionPart*/ *COLLISION_PART_MASK_ALL,
            /*FriendlyFire*/ false,
            /*Effect*/ Hash40::new("collision_attr_fire"),
            /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S,
            /*SFXType*/ *COLLISION_SOUND_ATTR_MARIO_FIREBALL,
            /*Type*/ *ATTACK_REGION_NONE,
        );
        AttackModule::enable_safe_pos(fighter.module_accessor);
    }
    frame(lua_state, 5.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(
            fighter,
            /*ID*/ 0,
            /*Part*/ 0,
            /*Bone*/ Hash40::new("top"),
            /*Damage*/ 5.0,
            /*Angle*/ 361,
            /*KBG*/ 65,
            /*FKB*/ 0,
            /*BKB*/ 28,
            /*Size*/ 2.2,
            /*X*/ 0.0,
            /*Y*/ 0.0,
            /*Z*/ 0.0,
            /*X2*/ None,
            /*Y2*/ None,
            /*Z2*/ None,
            /*Hitlag*/ 0.6,
            /*SDI*/ 1.0,
            /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON,
            /*FacingRestrict*/ *ATTACK_LR_CHECK_SPEED,
            /*SetWeight*/ false,
            /*ShieldDamage*/ -2.5,
            /*Trip*/ 0.0,
            /*Rehit*/ 0,
            /*Reflectable*/ true,
            /*Absorbable*/ true,
            /*Flinchless*/ false,
            /*DisableHitlag*/ false,
            /*Direct_Hitbox*/ false,
            /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA,
            /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL,
            /*CollisionPart*/ *COLLISION_PART_MASK_ALL,
            /*FriendlyFire*/ false,
            /*Effect*/ Hash40::new("collision_attr_fire"),
            /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S,
            /*SFXType*/ *COLLISION_SOUND_ATTR_MARIO_FIREBALL,
            /*Type*/ *ATTACK_REGION_NONE,
        );
    }
    frame(lua_state, 30.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(
            fighter,
            /*ID*/ 0,
            /*Part*/ 0,
            /*Bone*/ Hash40::new("top"),
            /*Damage*/ 4.0,
            /*Angle*/ 361,
            /*KBG*/ 60,
            /*FKB*/ 0,
            /*BKB*/ 22,
            /*Size*/ 2.0,
            /*X*/ 0.0,
            /*Y*/ 0.0,
            /*Z*/ 0.0,
            /*X2*/ None,
            /*Y2*/ None,
            /*Z2*/ None,
            /*Hitlag*/ 0.6,
            /*SDI*/ 1.0,
            /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON,
            /*FacingRestrict*/ *ATTACK_LR_CHECK_SPEED,
            /*SetWeight*/ false,
            /*ShieldDamage*/ -2,
            /*Trip*/ 0.0,
            /*Rehit*/ 0,
            /*Reflectable*/ true,
            /*Absorbable*/ true,
            /*Flinchless*/ false,
            /*DisableHitlag*/ false,
            /*Direct_Hitbox*/ false,
            /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA,
            /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL,
            /*CollisionPart*/ *COLLISION_PART_MASK_ALL,
            /*FriendlyFire*/ false,
            /*Effect*/ Hash40::new("collision_attr_fire"),
            /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S,
            /*SFXType*/ *COLLISION_SOUND_ATTR_MARIO_FIREBALL,
            /*Type*/ *ATTACK_REGION_NONE,
        );
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(mario_dsmash, mario_dashatk, mario_fireball, mario_sideb);
}
