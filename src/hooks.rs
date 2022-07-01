use smash::app::BattleObjectModuleAccessor;
use smash::{app, hash40, lib::lua_const::*};

#[skyline::hook(offset = 0x208d350)]
pub unsafe fn replace_param_float(
    boma: u64,
    param_type: u64,
    param_hash: u64
) -> f32 {
    let module_accessor = &mut *(*((boma as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
    let ret = original!()(boma, param_type, param_hash);
    let fighter_kind = app::utility::get_kind(module_accessor);

    if fighter_kind == FIGHTER_KIND_DONKEY {
        if param_hash == 0 && param_type != hash40("weight") {
            return ret
        }
        if param_type == hash40("weight") {
            return 115.0
        }
    }
    ret
}