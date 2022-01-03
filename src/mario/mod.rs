use smash::hash40;
use smash;
use smash::app::*;
use smash::lib::lua_const::*;
use smash::lua2cpp::L2CFighterCommon;
use acmd::{acmd, acmd_func};

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_MARIO, 
    animation = "attack_lw4",
    animcmd = "game_attacklw4")]
pub fn mario_dsmash(fighter: &mut L2CFighterCommon) {
    acmd!({
            frame(Frame=3)
            if(is_excute){
              WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
            }
            frame(Frame=5)
            if(is_excute){
              ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=12.0, Angle=32, KBG=100, FKB=0, BKB=45, Size=4.0, X=0.0, Y=3.6, Z=12.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
              ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=12.0, Angle=32, KBG=100, FKB=0, BKB=45, Size=3.3, X=0.0, Y=3.6, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
              AttackModule::set_attack_height_all(AttackHeight(*ATTACK_HEIGHT_LOW), false)
            }
            wait(Frames=2)
            if(is_excute){
              AttackModule::clear_all()
            }
            frame(Frame=14)
            if(is_excute){
              ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=14.0, Angle=30, KBG=100, FKB=0, BKB=45, Size=4.0, X=0.0, Y=3.6, Z=-11.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
              ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=14.0, Angle=30, KBG=100, FKB=0, BKB=45, Size=3.3, X=0.0, Y=3.6, Z=-6.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
              AttackModule::set_attack_height_all(AttackHeight(*ATTACK_HEIGHT_LOW), false)
            }
            wait(Frames=1)
            if(is_excute){
              AttackModule::clear_all()
            }
        });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_MARIO, 
    animation = "special_s",
    animcmd = "game_specials")]
pub fn mario_sideb(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(is_excute){
            ArticleModule::generate_article(FIGHTER_MARIO_GENERATE_ARTICLE_MANTLE, false, 0)
            sv_battle_object::notify_event_msc_cmd(smash::phx::Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS)
        }
        frame(Frame=6)
        if(is_excute){
            SEARCH(0, 0, hash40("top"), 8.0, 0.0, 6.5, 2.5, 0.0, 6.5, 8.0, COLLISION_KIND_MASK_ATTACK, HIT_STATUS_MASK_NORMAL, 60, COLLISION_SITUATION_MASK_GA, COLLISION_CATEGORY_MASK_ALL, COLLISION_PART_MASK_ALL, false, LUA_VOID, LUA_VOID, LUA_VOID)
            WorkModule::set_float(9.0, FIGHTER_MARIO_STATUS_SPECIAL_S_WORK_ID_FLOAT_REFLECT_MOTION_FRAME)
        }
        frame(Frame=9)
        if(is_excute){
            sv_module_access::search(MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL)
            sv_module_access::shield(MSC=MA_MSC_CMD_SHIELD_ON, Type=COLLISION_KIND_REFLECTOR, ID=FIGHTER_MARIO_REFLECTOR_KIND_MANTLE, Group=FIGHTER_REFLECTOR_GROUP_EXTEND)
        }
        frame(Frame=12)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_MARIO_STATUS_SPECIAL_S_FLAG_SPECIAL_FALL)
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=7.0, Angle=110, KBG=100, FKB=80, BKB=0, Size=7.5, X=0.0, Y=6.7, Z=9.7, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=4, Trip=10.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_turn"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MARIO_MANT, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=7.0, Angle=110, KBG=100, FKB=80, BKB=0, Size=5.0, X=0.0, Y=6.7, Z=5.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=4, Trip=10.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_turn"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MARIO_MANT, Type=ATTACK_REGION_OBJECT)
        }
        frame(Frame=15)
        if(is_excute){
            sv_battle_object::notify_event_msc_cmd(0x2127e37c07, GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES)
            AttackModule::clear_all()
        }
        frame(Frame=21)
        if(is_excute){
            sv_module_access::shield(MSC=MA_MSC_CMD_SHIELD_OFF, Type=COLLISION_KIND_REFLECTOR, ID=FIGHTER_MARIO_REFLECTOR_KIND_MANTLE, Group=FIGHTER_REFLECTOR_GROUP_EXTEND)
            WorkModule::off_flag(Flag=FIGHTER_MARIO_STATUS_SPECIAL_S_FLAG_SPECIAL_FALL)
        }
    });
}

pub fn install() {
    acmd::add_hooks!(
        mario_sideb,
        mario_dsmash
        
    );
}
