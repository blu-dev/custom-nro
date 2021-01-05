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

// #[export_name = "_ZN7lua2cpp41create_agent_fighter_status_script_donkeyEN3phx6Hash40EPN3app12BattleObjectEPNS2_26BattleObjectModuleAccessorEP9lua_State"]
// static mut OFFSET4: usize = 0usize;
// #[skyline::hook(replace = OFFSET4)]
// extern "C" fn status_replace(name: smash::phx::Hash40, obj: &mut BattleObject, boma: &mut BattleObjectModuleAccessor, state: &mut lua_State) -> *mut FighterDonkey {
//     println!("status call");
//     let mut ret = 0 as *mut FighterDonkey;
//     if name.hash == hash40("donkey") {
//         ret = FighterDonkey::new_ptr(obj, boma, state);
//     }
//     ret
// }

// #[export_name = "_ZN7lua2cpp47create_agent_fighter_animcmd_sound_share_donkeyEN3phx6Hash40EPN3app12BattleObjectEPNS2_26BattleObjectModuleAccessorEP9lua_State"]
// extern "C" fn animcmd_sound_share_replace(name: smash::phx::Hash40, obj: &mut BattleObject, boma: &mut BattleObjectModuleAccessor, state: &mut lua_State) -> *mut FighterAnimcmdSoundShareDonkey {
//     println!("animcmd sound share call");
//     let mut ret = 0 as *mut FighterAnimcmdSoundShareDonkey;
//     if name.hash == hash40("donkey") {
//         ret = FighterAnimcmdSoundShareDonkey::new_ptr(obj, boma, state);
//     }
//     ret
// }

// #[export_name = "_ZN7lua2cpp41create_agent_fighter_animcmd_sound_donkeyEN3phx6Hash40EPN3app12BattleObjectEPNS2_26BattleObjectModuleAccessorEP9lua_State"]
// extern "C" fn animcmd_sound_replace(name: smash::phx::Hash40, obj: &mut BattleObject, boma: &mut BattleObjectModuleAccessor, state: &mut lua_State) -> *mut FighterAnimcmdSoundDonkey {
//     let mut ret = 0 as *mut FighterAnimcmdSoundDonkey;
//     println!("animcmd sound call");
//     if name.hash == 0x1280c4791f {
//         ret = FighterAnimcmdDonkey::new_ptr(obj, boma, state) as *mut FighterAnimcmdSoundDonkey;
//     }
//     else if name.hash == hash40("donkey") {
//         ret = FighterAnimcmdSoundDonkey::new_ptr(obj, boma, state);
//     }
//     ret
// }

// #[export_name = "_ZN7lua2cpp46create_agent_fighter_animcmd_game_share_donkeyEN3phx6Hash40EPN3app12BattleObjectEPNS2_26BattleObjectModuleAccessorEP9lua_State"]
// extern "C" fn animcmd_game_share_replace(name: smash::phx::Hash40, obj: &mut BattleObject, boma: &mut BattleObjectModuleAccessor, state: &mut lua_State) -> *mut FighterAnimcmdGameShareDonkey {
//     let mut ret = 0 as *mut FighterAnimcmdGameShareDonkey;
//     println!("animcmd game share call");
//     if (name.hash == hash40("donkey")) {
//         ret = FighterAnimcmdGameShareDonkey::new_ptr(obj, boma, state);
//     }
//     ret
// }

// #[export_name = "_ZN7lua2cpp40create_agent_fighter_animcmd_game_donkeyEN3phx6Hash40EPN3app12BattleObjectEPNS2_26BattleObjectModuleAccessorEP9lua_State"]
// extern "C" fn animcmd_game_replace(name: smash::phx::Hash40, obj: &mut BattleObject, boma: &mut BattleObjectModuleAccessor, state: &mut lua_State) -> *mut FighterAnimcmdGameDonkey {
//     let mut ret = 0 as *mut FighterAnimcmdGameDonkey;
//     println!("animcmd game call");
//     if name.hash == hash40("donkey") {
//         ret = FighterAnimcmdGameDonkey::new_ptr(obj, boma, state);
//     }
//     ret
// }

// #[export_name = "_ZN7lua2cpp52create_agent_fighter_animcmd_expression_share_donkeyEN3phx6Hash40EPN3app12BattleObjectEPNS2_26BattleObjectModuleAccessorEP9lua_State"]
// extern "C" fn animcmd_expression_share_replace(name: smash::phx::Hash40, obj: &mut BattleObject, boma: &mut BattleObjectModuleAccessor, state: &mut lua_State) -> *mut FighterAnimcmdExpressionShareDonkey {
//     let mut ret = 0 as *mut FighterAnimcmdExpressionShareDonkey;
//     println!("animcmd expression share call");
//     if name.hash == hash40("donkey") {
//         ret = FighterAnimcmdExpressionShareDonkey::new_ptr(obj, boma, state);
//     }
//     ret
// }

// #[export_name = "_ZN7lua2cpp46create_agent_fighter_animcmd_expression_donkeyEN3phx6Hash40EPN3app12BattleObjectEPNS2_26BattleObjectModuleAccessorEP9lua_State"]
// extern "C" fn animcmd_expression_replace(name: smash::phx::Hash40, obj: &mut BattleObject, boma: &mut BattleObjectModuleAccessor, state: &mut lua_State) -> *mut FighterAnimcmdExpressionDonkey {
//     let mut ret = 0 as *mut FighterAnimcmdExpressionDonkey;
//     println!("animcmd expression call");
//     if name.hash == hash40("donkey") {
//         ret = FighterAnimcmdExpressionDonkey::new_ptr(obj, boma, state);
//     }
//     ret
// }

// #[export_name = "_ZN7lua2cpp48create_agent_fighter_animcmd_effect_share_donkeyEN3phx6Hash40EPN3app12BattleObjectEPNS2_26BattleObjectModuleAccessorEP9lua_State"]
// extern "C" fn animcmd_effect_share_replace(name: smash::phx::Hash40, obj: &mut BattleObject, boma: &mut BattleObjectModuleAccessor, state: &mut lua_State) -> *mut FighterAnimcmdEffectShareDonkey {
//     let mut ret = 0 as *mut FighterAnimcmdEffectShareDonkey;
//     println!("animcmd effect share call");
//     if name.hash == hash40("donkey") {
//         ret = FighterAnimcmdEffectShareDonkey::new_ptr(obj, boma, state);
//     }
//     ret
// }

// #[export_name = "_ZN7lua2cpp42create_agent_fighter_animcmd_effect_donkeyEN3phx6Hash40EPN3app12BattleObjectEPNS2_26BattleObjectModuleAccessorEP9lua_State"]
// extern "C" fn animcmd_effect_replace(name: smash::phx::Hash40, obj: &mut BattleObject, boma: &mut BattleObjectModuleAccessor, state: &mut lua_State) -> *mut FighterAnimcmdEffectDonkey {
//     let mut ret = 0 as *mut FighterAnimcmdEffectDonkey;
//     println!("animcmd effect call");
//     if name.hash == 0x1280c4791f {
//         ret = FighterAnimcmdDonkey::new_ptr(obj, boma, state) as *mut FighterAnimcmdEffectDonkey;
//     }
//     else if name.hash == hash40("donkey") {
//         ret = FighterAnimcmdEffectDonkey::new_ptr(obj, boma, state);
//     }
//     ret
// }

// #[export_name = "_ZN7lua2cpp35create_agent_fighter_ai_mode_donkeyEN3phx6Hash40EPN3app12BattleObjectEPNS2_26BattleObjectModuleAccessorEP9lua_State"]
// extern "C" fn ai_mode_replace(name: smash::phx::Hash40, obj: &mut BattleObject, boma: &mut BattleObjectModuleAccessor, state: &mut lua_State) -> *mut FighterAIModeDonkey {
//     let mut ret = 0 as *mut FighterAIModeDonkey;
//     println!("animcmd ai mode call");
//     if name.hash == hash40("donkey") {
//         ret = FighterAIModeDonkey::new_ptr(obj, boma, state);
//     }
//     ret
// }

// #[export_name = "_ZN7lua2cpp37create_agent_fighter_ai_action_donkeyEN3phx6Hash40EPN3app12BattleObjectEPNS2_26BattleObjectModuleAccessorEP9lua_State"]
// extern "C" fn ai_action_replace(name: smash::phx::Hash40, obj: &mut BattleObject, boma: &mut BattleObjectModuleAccessor, state: &mut lua_State) -> *mut FighterAIActionDonkey {
//     let mut ret = 0 as *mut FighterAIActionDonkey;
//     println!("animcmd ai mode call");
//     if name.hash == hash40("donkey") {
//         ret = FighterAIActionDonkey::new_ptr(obj, boma, state);
//     }
//     ret
// }

static mut CAN_PRINT: bool = false;

static mut OFFSET: usize = 0usize;

fn get_string(kind: u64) -> String {
    match (kind) {
        0x0 => "LUA_SCRIPT_STATUS_FUNC_STATUS_PRE".to_string(),
        0x1 => "LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN".to_string(),
        0x2 => "LUA_SCRIPT_STATUS_FUNC_STATUS_END".to_string(),
        0x3 => "LUA_SCRIPT_STATUS_FUNC_INIT_STATUS".to_string(),
        0x4 => "LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS".to_string(),
        0x5 => "LUA_SCRIPT_STATUS_FUNC_EXEC_STOP".to_string(),
        0x6 => "LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS_POST".to_string(),
        0x7 => "LUA_SCRIPT_STATUS_FUNC_EXIT_STATUS".to_string(),
        0x8 => "LUA_SCRIPT_STATUS_FUNC_MAP_CORRECTION".to_string(),
        0x9 => "LUA_SCRIPT_STATUS_FUNC_FIX_CAMERA".to_string(),
        0xA => "LUA_SCRIPT_STATUS_FUNC_FIX_POS_SLOW".to_string(),
        _ => "Unknown".to_string()
    }
}

fn get_status_string(kind: u64) -> String {
    match (kind) {
        0xc5 => "FIGHTER_STATUS_KIND_SHOULDERED_DONKEY_START".to_string(),
        0xc6 => "FIGHTER_STATUS_KIND_SHOULDERED_DONKEY".to_string(),
        0x10 => "FIGHTER_KIND_DONKEY".to_string(),
        0x0 => "FIGHTER_DONKEY_GENERATE_ARTICLE_DKBARREL".to_string(),
        0x11000005 => "FIGHTER_STATUS_SHOULDERED_DONKEY_WORK_INT_START_LR".to_string(),
        0xc7 => "FIGHTER_STATUS_KIND_SHOULDERED_DONKEY_THROWN".to_string(),
        0x1ea => "FIGHTER_DONKEY_STATUS_KIND_MAX".to_string(),
        0x1d2 => "FIGHTER_DONKEY_STATUS_KIND_SHOULDER_START".to_string(),
        0x1c9 => "FIGHTER_DONKEY_STATUS_KIND_SUPER_LIFT_WAIT".to_string(),
        0x1ca => "FIGHTER_DONKEY_STATUS_KIND_SUPER_LIFT_WALK".to_string(),
        0x1cb => "FIGHTER_DONKEY_STATUS_KIND_SUPER_LIFT_TURN".to_string(),
        0x1cc => "FIGHTER_DONKEY_STATUS_KIND_SUPER_LIFT_JUMP_SQUAT".to_string(),
        0x1cd => "FIGHTER_DONKEY_STATUS_KIND_SUPER_LIFT_JUMP_SQUAT_B".to_string(),
        0x1ce => "FIGHTER_DONKEY_STATUS_KIND_SUPER_LIFT_JUMP".to_string(),
        0x1cf => "FIGHTER_DONKEY_STATUS_KIND_SUPER_LIFT_FALL".to_string(),
        0x1d0 => "FIGHTER_DONKEY_STATUS_KIND_SUPER_LIFT_LANDING".to_string(),
        0x1d1 => "FIGHTER_DONKEY_STATUS_KIND_SUPER_LIFT_PASS".to_string(),
        0x1d3 => "FIGHTER_DONKEY_STATUS_KIND_SHOULDER_WAIT".to_string(),
        0x1d4 => "FIGHTER_DONKEY_STATUS_KIND_SHOULDER_WALK".to_string(),
        0x1d5 => "FIGHTER_DONKEY_STATUS_KIND_SHOULDER_TURN".to_string(),
        0x1d6 => "FIGHTER_DONKEY_STATUS_KIND_SHOULDER_JUMP_SQUAT".to_string(),
        0x1d7 => "FIGHTER_DONKEY_STATUS_KIND_SHOULDER_JUMP_SQUAT_B".to_string(),
        0x1d8 => "FIGHTER_DONKEY_STATUS_KIND_SHOULDER_JUMP".to_string(),
        0x1d9 => "FIGHTER_DONKEY_STATUS_KIND_SHOULDER_FALL".to_string(),
        0x1da => "FIGHTER_DONKEY_STATUS_KIND_SHOULDER_LANDING".to_string(),
        0x1db => "FIGHTER_DONKEY_STATUS_KIND_SHOULDER_PASS".to_string(),
        0x1dc => "FIGHTER_DONKEY_STATUS_KIND_SHOULDER_END".to_string(),
        0x1dd => "FIGHTER_DONKEY_STATUS_KIND_THROW_F_F".to_string(),
        0x1de => "FIGHTER_DONKEY_STATUS_KIND_THROW_F_B".to_string(),
        0x1df => "FIGHTER_DONKEY_STATUS_KIND_THROW_F_HI".to_string(),
        0x1e0 => "FIGHTER_DONKEY_STATUS_KIND_THROW_F_LW".to_string(),
        0x1e6 => "FIGHTER_DONKEY_STATUS_KIND_SPECIAL_LW_LOOP".to_string(),
        0x1e7 => "FIGHTER_DONKEY_STATUS_KIND_SPECIAL_LW_END".to_string(),
        0x1e1 => "FIGHTER_DONKEY_STATUS_KIND_SPECIAL_N_LOOP".to_string(),
        0x1e2 => "FIGHTER_DONKEY_STATUS_KIND_SPECIAL_N_CANCEL".to_string(),
        0x1e3 => "FIGHTER_DONKEY_STATUS_KIND_SPECIAL_N_JUMP_CANCEL".to_string(),
        0x1e4 => "FIGHTER_DONKEY_STATUS_KIND_SPECIAL_N_ATTACK".to_string(),
        0x1e5 => "FIGHTER_DONKEY_STATUS_KIND_SPECIAL_N_END".to_string(),
        0x1e8 => "FIGHTER_DONKEY_STATUS_KIND_FINAL_ATTACK".to_string(),
        0x1e9 => "FIGHTER_DONKEY_STATUS_KIND_FINAL_END".to_string(),
        0x11000005 => "FIGHTER_DONKEY_STATUS_FINAL_WORK_INT_MOTION_KIND_GROUND".to_string(),
        0x11000006 => "FIGHTER_DONKEY_STATUS_FINAL_WORK_INT_MOTION_KIND_AIR".to_string(),
        0x2100000c => "FIGHTER_DONKEY_STATUS_FINAL_FLAG_MOT_CHANGE".to_string(),
        0x2100000f => "FIGHTER_DONKEY_STATUS_FINAL_FLAG_TO_ATTACK".to_string(),
        0x2100000d => "FIGHTER_DONKEY_STATUS_FINAL_FLAG_ATTACK_HIT".to_string(),
        0x1000009 => "FIGHTER_DONKEY_STATUS_FINAL_WORK_FLOAT_FIX_POS_X".to_string(),
        0x100000a => "FIGHTER_DONKEY_STATUS_FINAL_WORK_FLOAT_FIX_POS_Y".to_string(),
        0x21000011 => "FIGHTER_DONKEY_STATUS_FINAL_FLAG_ATTACK_END_HIT".to_string(),
        0x2100000c => "FIGHTER_DONKEY_STATUS_SHOULDER_START_FLAG_SHOULDER_WAIT".to_string(),
        0x2100000d => "FIGHTER_DONKEY_STATUS_SPECIAL_HI_FLAG_YACL_DEFAULT".to_string(),
        0x2100000f => "FIGHTER_DONKEY_STATUS_SPECIAL_HI_FLAG_GROUND_SPINEND".to_string(),
        0x2100000e => "FIGHTER_DONKEY_STATUS_SPECIAL_HI_FLAG_CLIFF_CHECK".to_string(),
        0x2100000c => "FIGHTER_DONKEY_STATUS_SPECIAL_HI_FLAG_MOT_CHANGE".to_string(),
        0x21000010 => "FIGHTER_DONKEY_STATUS_SPECIAL_HI_FLAG_GROUND_MOT_FRAME".to_string(),
        0x2100000c => "FIGHTER_DONKEY_STATUS_SPECIAL_LW_FLAG_LOOP".to_string(),
        0x2100000d => "FIGHTER_DONKEY_STATUS_SPECIAL_LW_FLAG_ATTACK".to_string(),
        0x100000ba => "FIGHTER_DONKEY_INSTANCE_WORK_ID_INT_SPECIAL_N_COUNT".to_string(),
        0x11000008 => "FIGHTER_DONKEY_STATUS_SPECIAL_N_WORK_INT_POWER_ADD".to_string(),
        0x11000006 => "FIGHTER_DONKEY_STATUS_SPECIAL_N_WORK_INT_DEFAULT_POWER_0".to_string(),
        0x11000007 => "FIGHTER_DONKEY_STATUS_SPECIAL_N_WORK_INT_DEFAULT_POWER_1".to_string(),
        0x2100000c => "FIGHTER_DONKEY_STATUS_SPECIAL_N_FLAG_MOT_CHANGE".to_string(),
        0x11000005 => "FIGHTER_DONKEY_STATUS_SPECIAL_N_WORK_INT_PUNCH_COUNT".to_string(),
        0x1000007 => "FIGHTER_DONKEY_STATUS_SPECIAL_N_WORK_FLOAT_MOTION_RATE".to_string(),
        0x4 => "FIGHTER_DONKEY_SPECIAL_N_CANCEL_TYPE_GROUND_GUARD".to_string(),
        0x1100000a => "FIGHTER_DONKEY_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE".to_string(),
        0x1 => "FIGHTER_DONKEY_SPECIAL_N_CANCEL_TYPE_GROUND_ESCAPE".to_string(),
        0x2 => "FIGHTER_DONKEY_SPECIAL_N_CANCEL_TYPE_GROUND_ESCAPE_F".to_string(),
        0x3 => "FIGHTER_DONKEY_SPECIAL_N_CANCEL_TYPE_GROUND_ESCAPE_B".to_string(),
        0x5 => "FIGHTER_DONKEY_SPECIAL_N_CANCEL_TYPE_GROUND_JUMP".to_string(),
        0x6 => "FIGHTER_DONKEY_SPECIAL_N_CANCEL_TYPE_AIR_ESCAPE_AIR".to_string(),
        0x7 => "FIGHTER_DONKEY_SPECIAL_N_CANCEL_TYPE_AIR_JUMP".to_string(),
        0x0 => "FIGHTER_DONKEY_SPECIAL_N_CANCEL_TYPE_NONE".to_string(),
        0x2100000e => "FIGHTER_DONKEY_STATUS_SPECIAL_N_FLAG_CANCEL_FACE".to_string(),
        0x2100000d => "FIGHTER_DONKEY_STATUS_SPECIAL_N_FLAG_MAX".to_string(),
        0x2100000d => "FIGHTER_DONKEY_STATUS_SPECIAL_S_FLAG_FALL_START".to_string(),
        0x2100000c => "FIGHTER_DONKEY_STATUS_SPECIAL_S_FLAG_MOT_CHANGE".to_string(),
        0x232 => "FIGHTER_KIRBY_STATUS_KIND_DONKEY_SPECIAL_N".to_string(),
        0x233 => "FIGHTER_KIRBY_STATUS_KIND_DONKEY_SPECIAL_N_LOOP".to_string(),
        0x234 => "FIGHTER_KIRBY_STATUS_KIND_DONKEY_SPECIAL_N_CANCEL".to_string(),
        0x235 => "FIGHTER_KIRBY_STATUS_KIND_DONKEY_SPECIAL_N_JUMP_CANCEL".to_string(),
        0x236 => "FIGHTER_KIRBY_STATUS_KIND_DONKEY_SPECIAL_N_ATTACK".to_string(),
        0x237 => "FIGHTER_KIRBY_STATUS_KIND_DONKEY_SPECIAL_N_END".to_string(),
        0x1100000b => "FIGHTER_DONKEY_STATUS_SPECIAL_N_WORK_INT_IGNORE_CRITICAL_ATTACK_ID".to_string(),
        0x2100000e => "FIGHTER_DONKEY_STATUS_FINAL_FLAG_HIT_ALL_OPPONENT".to_string(),
        _ => "Unknown".to_string()
    }
}

#[skyline::hook(replace = OFFSET)]
fn status_replace(agent_base: &mut smash::lua2cpp::L2CAgentBase, param_1: &mut smash::lib::L2CValue, param_2: &mut smash::lib::L2CValue, function: *mut skyline::libc::c_void) {
    unsafe {
        if CAN_PRINT {
            let text = OFFSET1 - 0x640;
            let func_off = (function as usize) - text;
            let status_name = get_status_string(param_1.get_int());
            let timing = get_string(param_2.get_int());
            if status_name != "Unknown" && timing != "Unknown" {
                println!("{}_{}:{:X}", status_name, timing, func_off);
            }
            else {
                println!("Unknown:\tkind: {:X}\tcategory: {:X}\tfunc_off: {:X}", param_1.get_int(), param_2.get_int(), func_off);
            }
        }
    }
}

static mut OFFSET1: usize = 0usize;
static mut OFFSET2: usize = 0usize;
static mut OFFSET3: usize = 0usize;

#[skyline::hook(replace = OFFSET1, inline)]
fn start_hook(ctx: &skyline::hooks::InlineCtx) {
    unsafe { CAN_PRINT = true; }
}
#[skyline::hook(replace = OFFSET2, inline)]
fn end_hook(ctx: &skyline::hooks::InlineCtx) {
    unsafe { CAN_PRINT = false; }
}
#[skyline::hook(replace = OFFSET3, inline)]
fn get_value(ctx: &skyline::hooks::InlineCtx) {
    println!("Value: {:X}", ctx.registers[21].bindgen_union_field);
}

fn dk_callback(info: &skyline::nro::NroInfo) {
    if (info.name == "common") {
        unsafe {
            OFFSET = (*info.module.ModuleObject).module_base as usize + 0x4fd310;
        }
        skyline::install_hook!(status_replace);
    }
    else if (info.name == "donkey") {

        // skyline::install_hooks!(ai_action_replace, ai_mode_replace, animcmd_effect_share_replace, animcmd_effect_replace, animcmd_expression_share_replace, animcmd_expression_replace, animcmd_game_share_replace, animcmd_game_replace, animcmd_sound_share_replace, animcmd_sound_replace);
        unsafe {
            OFFSET1 = (*info.module.ModuleObject).module_base as usize + 0x2700;
            OFFSET2 = (*info.module.ModuleObject).module_base as usize + 0x3928;
            OFFSET3 = (*info.module.ModuleObject).module_base as usize + 0x664;
            // OFFSET4 = (*info.module.ModuleObject).module_base as usize + 0x480;
        }
        // skyline::install_hooks!(status_replace, get_value);
        skyline::install_hooks!(start_hook, end_hook);
    }
}

#[skyline::main(name = "custom-nro")]
pub fn main() {
    skyline::nro::add_hook(dk_callback);
}
