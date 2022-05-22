use smash::hash40;
use smash;
use smash::app::*;
use smash::lib::lua_const::*;
use smash::lua2cpp::L2CFighterCommon;
use acmd::{acmd, acmd_func};

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
    battle_object_kind = FIGHTER_KIND_DONKEY,
    animation = "special_hi",
    animcmd = "game_specialhi")]
pub fn dk_upb(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=5)
    if(is_excute){
        sv_module_access::damage(MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_ALWAYS, 0)
    }
    frame(Frame=18)
    if(is_excute){
        sv_module_access::damage(MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_NORMAL, 0)
    }
    frame(Frame=19)
    if(is_excute){
        HIT_NODE(hash40("armr"), HIT_STATUS_XLU)
        ATTACK(ID=0, Part=0, Bone=hash40("armr"), Damage=5.0, Angle=95, KBG=90, FKB=30, BKB=0, Size=6.0, X=6.5, Y=-1.0, Z=-1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_BODY)
        ATTACK(ID=1, Part=0, Bone=hash40("armr"), Damage=5.0, Angle=80, KBG=90, FKB=30, BKB=0, Size=5.0, X=0.0, Y=-1.0, Z=-1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_BODY)
        ATTACK(ID=2, Part=0, Bone=hash40("shoulderr"), Damage=5.0, Angle=80, KBG=90, FKB=30, BKB=0, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_BODY)
        ATTACK(ID=3, Part=0, Bone=hash40("armr"), Damage=5.0, Angle=366, KBG=90, FKB=30, BKB=0, Size=6.0, X=6.5, Y=-1.0, Z=-1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_BODY)
        ATTACK(ID=4, Part=0, Bone=hash40("armr"), Damage=5.0, Angle=366, KBG=90, FKB=30, BKB=0, Size=5.0, X=0.0, Y=-1.0, Z=-1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_BODY)
        ATTACK(ID=5, Part=0, Bone=hash40("shoulderr"), Damage=5.0, Angle=366, KBG=90, FKB=30, BKB=0, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_BODY)
    }
    wait(Frames=1)
    if(is_excute){
        AttackModule::clear_all()
    }
    frame(Frame=25)
    if(is_excute){
        HitModule::set_status_all(HitStatus(*HIT_STATUS_NORMAL),0)
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.4, Angle=367, KBG=55, FKB=90, BKB=0, Size=4.0, X=0.0, Y=7.0, Z=25.0, X2=0.0, Y2=12.0, Z2=25.0, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_BODY)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=1.4, Angle=20, KBG=100, FKB=65, BKB=0, Size=5.0, X=0.0, Y=12.0, Z=12.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_BODY)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=3.5, Angle=60, KBG=100, FKB=0, BKB=35, Size=5.7, X=0.0, Y=13.5, Z=-2.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_BODY)
    }
    wait(Frames=3)
    if(is_excute){
        AttackModule::clear_all()
    }
    frame(Frame=32)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.4, Angle=367, KBG=55, FKB=80, BKB=0, Size=4.0, X=0.0, Y=7.0, Z=25.0, X2=0.0, Y2=12.0, Z2=25.0, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_BODY)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=1.4, Angle=20, KBG=100, FKB=65, BKB=0, Size=5.0, X=0.0, Y=12.0, Z=12.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_BODY)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=3.5, Angle=60, KBG=100, FKB=0, BKB=35, Size=5.7, X=0.0, Y=13.5, Z=-2.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_BODY)
    }
    wait(Frames=2)
    if(is_excute){
        AttackModule::clear_all()
    }
    frame(Frame=40)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.4, Angle=367, KBG=55, FKB=90, BKB=0, Size=4.0, X=0.0, Y=7.0, Z=25.0, X2=0.0, Y2=12.0, Z2=25.0, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_BODY)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=1.4, Angle=20, KBG=100, FKB=65, BKB=0, Size=5.0, X=0.0, Y=12.0, Z=12.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_BODY)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=3.5, Angle=60, KBG=100, FKB=0, BKB=35, Size=5.7, X=0.0, Y=13.5, Z=-2.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_BODY)
    }
    wait(Frames=3)
    if(is_excute){
        AttackModule::clear_all()
    }
    frame(Frame=49)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.4, Angle=367, KBG=55, FKB=90, BKB=0, Size=4.0, X=0.0, Y=7.0, Z=25.0, X2=0.0, Y2=12.0, Z2=25.0, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_BODY)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=1.4, Angle=20, KBG=100, FKB=65, BKB=0, Size=5.0, X=0.0, Y=12.0, Z=12.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_BODY)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=3.5, Angle=60, KBG=100, FKB=0, BKB=35, Size=5.7, X=0.0, Y=13.5, Z=-2.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_BODY)
    }
    wait(Frames=2)
    if(is_excute){
        AttackModule::clear_all()
    }
    frame(Frame=55)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.4, Angle=367, KBG=55, FKB=90, BKB=0, Size=4.0, X=0.0, Y=7.0, Z=25.0, X2=0.0, Y2=12.0, Z2=25.0, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_BODY)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=1.4, Angle=20, KBG=100, FKB=65, BKB=0, Size=5.0, X=0.0, Y=12.0, Z=12.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_BODY)
        ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=3.5, Angle=60, KBG=100, FKB=0, BKB=35, Size=5.7, X=0.0, Y=13.5, Z=-2.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_BODY)
    }
    wait(Frames=4)
    if(is_excute){
        AttackModule::clear_all()
    }
    frame(Frame=62)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_DONKEY_STATUS_SPECIAL_HI_FLAG_YACL_DEFAULT)
        WorkModule::on_flag(Flag=FIGHTER_DONKEY_STATUS_SPECIAL_HI_FLAG_GROUND_SPINEND)
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=8.0, Angle=361, KBG=120, FKB=0, BKB=60, Size=8.0, X=0.0, Y=11.0, Z=20.0, X2=0.0, Y2=11.0, Z2=-4.0, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_BODY)
    }
    wait(Frames=1)
    if(is_excute){
        AttackModule::clear_all()
    }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
    battle_object_kind = FIGHTER_KIND_DONKEY,
    animation = "attack_lw3",
    animcmd = "game_attacklw3")]
pub fn dk_dtilt(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=5)
    if(is_excute){
        HIT_NODE(hash40("arml"), HIT_STATUS_XLU)
        HIT_NODE(hash40("shoulderl"), HIT_STATUS_XLU)
    }
    frame(Frame=6)
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("shoulderl"), Damage=6.0, Angle=361, KBG=100, FKB=0, BKB=20, Size=4.4, X=2.3, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=50.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=1, Part=0, Bone=hash40("arml"), Damage=6.0, Angle=361, KBG=100, FKB=0, BKB=20, Size=4.4, X=1.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=50.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=2, Part=0, Bone=hash40("arml"), Damage=6.0, Angle=361, KBG=100, FKB=0, BKB=20, Size=5.5, X=8.5, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=50.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
        AttackModule::set_attack_height_all(ATTACK_HEIGHT_LOW, false)
    }
    frame(Frame=8)
    if(is_excute){
        AttackModule::clear_all()
        HitModule::set_status_all(HitStatus(*HIT_STATUS_NORMAL),0)
    }
    });
}


pub fn install() {
    acmd::add_hooks!(
        dk_upb,
        dk_dtilt
    )
}


