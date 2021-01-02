#![feature(proc_macro_hygiene)]
#![feature(asm)]
use exports::*;
use skyline::nro::add_hook;
use smash::{hash40, phx::Hash40};
use smash::lua2cpp::L2CFighterCommon;
use smash::app::{BattleObjectModuleAccessor, BattleObject};
use smash::lua_State;

mod exports;
mod bt;
// mod vtables;

static mut LOCATION: usize = 0;
#[skyline::hook(replace = LOCATION)]
fn stub(param_1: u64) { }

#[export_name = "_ZN7lua2cpp41create_agent_fighter_status_script_donkeyEN3phx6Hash40EPN3app12BattleObjectEPNS2_26BattleObjectModuleAccessorEP9lua_State"]
extern "C" fn status_replace(name: smash::phx::Hash40, obj: &mut BattleObject, boma: &mut BattleObjectModuleAccessor, state: &mut lua_State) -> *mut FighterDonkey {
    println!("status call");
    let mut ret = 0 as *mut FighterDonkey;
    if name.hash == hash40("donkey") {
        ret = FighterDonkey::new_ptr(obj, boma, state);
    }
    ret
}

#[export_name = "_ZN7lua2cpp47create_agent_fighter_animcmd_sound_share_donkeyEN3phx6Hash40EPN3app12BattleObjectEPNS2_26BattleObjectModuleAccessorEP9lua_State"]
extern "C" fn animcmd_sound_share_replace(name: smash::phx::Hash40, obj: &mut BattleObject, boma: &mut BattleObjectModuleAccessor, state: &mut lua_State) -> *mut FighterAnimcmdSoundShareDonkey {
    println!("animcmd sound share call");
    let mut ret = 0 as *mut FighterAnimcmdSoundShareDonkey;
    if name.hash == hash40("donkey") {
        ret = FighterAnimcmdSoundShareDonkey::new_ptr(obj, boma, state);
    }
    ret
}

#[export_name = "_ZN7lua2cpp41create_agent_fighter_animcmd_sound_donkeyEN3phx6Hash40EPN3app12BattleObjectEPNS2_26BattleObjectModuleAccessorEP9lua_State"]
extern "C" fn animcmd_sound_replace(name: smash::phx::Hash40, obj: &mut BattleObject, boma: &mut BattleObjectModuleAccessor, state: &mut lua_State) -> *mut FighterAnimcmdSoundDonkey {
    let mut ret = 0 as *mut FighterAnimcmdSoundDonkey;
    println!("animcmd sound call");
    if name.hash == 0x1280c4791f {
        ret = FighterAnimcmdDonkey::new_ptr(obj, boma, state) as *mut FighterAnimcmdSoundDonkey;
    }
    else if name.hash == hash40("donkey") {
        ret = FighterAnimcmdSoundDonkey::new_ptr(obj, boma, state);
    }
    ret
}

#[export_name = "_ZN7lua2cpp46create_agent_fighter_animcmd_game_share_donkeyEN3phx6Hash40EPN3app12BattleObjectEPNS2_26BattleObjectModuleAccessorEP9lua_State"]
extern "C" fn animcmd_game_share_replace(name: smash::phx::Hash40, obj: &mut BattleObject, boma: &mut BattleObjectModuleAccessor, state: &mut lua_State) -> *mut FighterAnimcmdGameShareDonkey {
    let mut ret = 0 as *mut FighterAnimcmdGameShareDonkey;
    println!("animcmd game share call");
    if (name.hash == hash40("donkey")) {
        ret = FighterAnimcmdGameShareDonkey::new_ptr(obj, boma, state);
    }
    ret
}

#[export_name = "_ZN7lua2cpp40create_agent_fighter_animcmd_game_donkeyEN3phx6Hash40EPN3app12BattleObjectEPNS2_26BattleObjectModuleAccessorEP9lua_State"]
extern "C" fn animcmd_game_replace(name: smash::phx::Hash40, obj: &mut BattleObject, boma: &mut BattleObjectModuleAccessor, state: &mut lua_State) -> *mut FighterAnimcmdGameDonkey {
    let mut ret = 0 as *mut FighterAnimcmdGameDonkey;
    println!("animcmd game call");
    if name.hash == hash40("donkey") {
        ret = FighterAnimcmdGameDonkey::new_ptr(obj, boma, state);
    }
    ret
}

#[export_name = "_ZN7lua2cpp52create_agent_fighter_animcmd_expression_share_donkeyEN3phx6Hash40EPN3app12BattleObjectEPNS2_26BattleObjectModuleAccessorEP9lua_State"]
extern "C" fn animcmd_expression_share_replace(name: smash::phx::Hash40, obj: &mut BattleObject, boma: &mut BattleObjectModuleAccessor, state: &mut lua_State) -> *mut FighterAnimcmdExpressionShareDonkey {
    let mut ret = 0 as *mut FighterAnimcmdExpressionShareDonkey;
    println!("animcmd expression share call");
    if name.hash == hash40("donkey") {
        ret = FighterAnimcmdExpressionShareDonkey::new_ptr(obj, boma, state);
    }
    ret
}

#[export_name = "_ZN7lua2cpp46create_agent_fighter_animcmd_expression_donkeyEN3phx6Hash40EPN3app12BattleObjectEPNS2_26BattleObjectModuleAccessorEP9lua_State"]
extern "C" fn animcmd_expression_replace(name: smash::phx::Hash40, obj: &mut BattleObject, boma: &mut BattleObjectModuleAccessor, state: &mut lua_State) -> *mut FighterAnimcmdExpressionDonkey {
    let mut ret = 0 as *mut FighterAnimcmdExpressionDonkey;
    println!("animcmd expression call");
    if name.hash == hash40("donkey") {
        ret = FighterAnimcmdExpressionDonkey::new_ptr(obj, boma, state);
    }
    ret
}

#[export_name = "_ZN7lua2cpp48create_agent_fighter_animcmd_effect_share_donkeyEN3phx6Hash40EPN3app12BattleObjectEPNS2_26BattleObjectModuleAccessorEP9lua_State"]
extern "C" fn animcmd_effect_share_replace(name: smash::phx::Hash40, obj: &mut BattleObject, boma: &mut BattleObjectModuleAccessor, state: &mut lua_State) -> *mut FighterAnimcmdEffectShareDonkey {
    let mut ret = 0 as *mut FighterAnimcmdEffectShareDonkey;
    println!("animcmd effect share call");
    if name.hash == hash40("donkey") {
        ret = FighterAnimcmdEffectShareDonkey::new_ptr(obj, boma, state);
    }
    ret
}

#[export_name = "_ZN7lua2cpp42create_agent_fighter_animcmd_effect_donkeyEN3phx6Hash40EPN3app12BattleObjectEPNS2_26BattleObjectModuleAccessorEP9lua_State"]
extern "C" fn animcmd_effect_replace(name: smash::phx::Hash40, obj: &mut BattleObject, boma: &mut BattleObjectModuleAccessor, state: &mut lua_State) -> *mut FighterAnimcmdEffectDonkey {
    let mut ret = 0 as *mut FighterAnimcmdEffectDonkey;
    println!("animcmd effect call");
    if name.hash == 0x1280c4791f {
        ret = FighterAnimcmdDonkey::new_ptr(obj, boma, state) as *mut FighterAnimcmdEffectDonkey;
    }
    else if name.hash == hash40("donkey") {
        ret = FighterAnimcmdEffectDonkey::new_ptr(obj, boma, state);
    }
    ret
}

#[export_name = "_ZN7lua2cpp35create_agent_fighter_ai_mode_donkeyEN3phx6Hash40EPN3app12BattleObjectEPNS2_26BattleObjectModuleAccessorEP9lua_State"]
extern "C" fn ai_mode_replace(name: smash::phx::Hash40, obj: &mut BattleObject, boma: &mut BattleObjectModuleAccessor, state: &mut lua_State) -> *mut FighterAIModeDonkey {
    let mut ret = 0 as *mut FighterAIModeDonkey;
    println!("animcmd ai mode call");
    if name.hash == hash40("donkey") {
        ret = FighterAIModeDonkey::new_ptr(obj, boma, state);
    }
    ret
}

#[export_name = "_ZN7lua2cpp37create_agent_fighter_ai_action_donkeyEN3phx6Hash40EPN3app12BattleObjectEPNS2_26BattleObjectModuleAccessorEP9lua_State"]
extern "C" fn ai_action_replace(name: smash::phx::Hash40, obj: &mut BattleObject, boma: &mut BattleObjectModuleAccessor, state: &mut lua_State) -> *mut FighterAIActionDonkey {
    let mut ret = 0 as *mut FighterAIActionDonkey;
    println!("animcmd ai mode call");
    if name.hash == hash40("donkey") {
        ret = FighterAIActionDonkey::new_ptr(obj, boma, state);
    }
    ret
}

// fn dk_callback(info: &skyline::nro::NroInfo) {
//     if (info.name == "donkey") {
//         skyline::install_hooks!(ai_action_replace, ai_mode_replace, animcmd_effect_share_replace, animcmd_effect_replace, animcmd_expression_share_replace, animcmd_expression_replace, animcmd_game_share_replace, animcmd_game_replace, animcmd_sound_share_replace, animcmd_sound_replace);
//     }
// }

#[skyline::main(name = "custom-nro")]
pub fn main() {
}
