pub use smash_script::macros::*;
use smash::lib::L2CValue;
use smash::lua2cpp::L2CAgentBase;
use smash::phx::Hash40;

use smash::app::sv_animcmd;

#[inline]
pub unsafe fn SHOOT_ITEM_BULLET(fighter: &mut L2CAgentBase) {
    fighter.clear_lua_stack();
    smash_script::lua_args!(fighter, );
    sv_animcmd::SHOOT_ITEM_BULLET(fighter.lua_state_agent);

}