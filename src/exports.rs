use smash::{app::Fighter, phx::Hash40};
use smash::lua2cpp::*;
use smash::app::{BattleObjectModuleAccessor, BattleObject};
use smash::lua_State;
use smash::lib;

pub struct FighterDonkey {
    pub fighter_common: L2CFighterCommon,
    reserved: [u8; 0x228 - 0xE0]
}
pub mod vtable {
    pub unsafe fn make_new(original: *const *mut skyline::libc::c_void, length: usize) -> *mut *mut skyline::libc::c_void {
        let new_vtable = std::alloc::alloc(std::alloc::Layout::from_size_align(length * std::mem::size_of::<*mut skyline::libc::c_void>(), 8).unwrap()) as *mut *mut skyline::libc::c_void;
        std::ptr::copy_nonoverlapping(original, new_vtable, length);
        new_vtable
    }

    pub unsafe fn destroy(vtable: *mut *mut skyline::libc::c_void, length: usize) {
        std::alloc::dealloc(vtable as *mut u8, std::alloc::Layout::from_size_align(length * std::mem::size_of::<*mut skyline::libc::c_void>(), 8).unwrap());
    }
}

impl FighterDonkey {
    const VTABLE_LENGTH: usize = 15;

    unsafe fn call_virt_dtor(fighter: &mut FighterDonkey) {
        let dtor_ptr = *(fighter.fighter_common.fighter_base.agent_base.agent.vtable as *const *const fn(&mut FighterDonkey));
        let dtor: unsafe extern "C" fn(&mut FighterDonkey) = std::mem::transmute(dtor_ptr);
        vtable::destroy(fighter.fighter_common.fighter_base.agent_base.agent.vtable as *mut *mut skyline::libc::c_void, FighterDonkey::VTABLE_LENGTH);
        dtor(fighter);
    }

    unsafe extern "C" fn del_destroy(fighter: *mut FighterDonkey) {
        FighterDonkey::call_virt_dtor(&mut *fighter);
        std::alloc::dealloc(fighter as *mut u8, std::alloc::Layout::from_size_align(std::mem::size_of::<FighterDonkey>(), 1).unwrap());
    }

    unsafe extern "C" fn setup_status_scripts(fighter: &mut FighterDonkey) {
        // Set count of status array
        let this_ptr: *mut u8 = fighter as *mut FighterDonkey as *mut u8;
        let some_l2cval: *mut lib::L2CValue = (this_ptr as u64 + 200) as *mut lib::L2CValue;
        let other_l2cval: &mut lib::L2CValue = &mut (*some_l2cval)[0xC];
        other_l2cval.inner.raw = 0x1F3;
        
        L2CAgentBase::reserve_status_data_array(&mut *(this_ptr as *mut L2CAgentBase), 0x1F3);
        fighter.fighter_common.sub_set_fighter_common_table();
    }

    pub fn new(obj: &mut BattleObject, boma: &mut BattleObjectModuleAccessor, state: &mut lua_State) -> FighterDonkey {
        unsafe {
            let mut ret = FighterDonkey { fighter_common: std::mem::zeroed(), reserved: [0; 0x228 - 0xE0] };
            ret.fighter_common.L2CFighterCommon(obj, boma, state);
            let new_vtable = vtable::make_new(ret.fighter_common.fighter_base.agent_base.agent.vtable as *const *mut skyline::libc::c_void, FighterDonkey::VTABLE_LENGTH);
            ret.fighter_common.fighter_base.agent_base.agent.vtable = new_vtable as u64;
            let vtable_slice = std::slice::from_raw_parts_mut(new_vtable, FighterDonkey::VTABLE_LENGTH);
            vtable_slice[1] = std::mem::transmute(FighterDonkey::del_destroy as *const fn());
            vtable_slice[9] = std::mem::transmute(FighterDonkey::setup_status_scripts as *const fn());
            ret
        }
    }

    pub fn new_ptr(obj: &mut BattleObject, boma: &mut BattleObjectModuleAccessor, state: &mut lua_State) -> *mut FighterDonkey {
        unsafe {
            let ret = std::alloc::alloc(std::alloc::Layout::from_size_align(std::mem::size_of::<FighterDonkey>(), 1).unwrap()) as *mut FighterDonkey;
            (*ret).fighter_common.L2CFighterCommon(obj, boma, state);
            let new_vtable = vtable::make_new((*ret).fighter_common.fighter_base.agent_base.agent.vtable as *const *mut skyline::libc::c_void, FighterDonkey::VTABLE_LENGTH);
            (*ret).fighter_common.fighter_base.agent_base.agent.vtable = new_vtable as u64;
            let vtable_slice = std::slice::from_raw_parts_mut(new_vtable, FighterDonkey::VTABLE_LENGTH);
            vtable_slice[1] = std::mem::transmute(FighterDonkey::del_destroy as *const fn());
            vtable_slice[9] = std::mem::transmute(FighterDonkey::setup_status_scripts as *const fn());
            ret
        }
    }
}

impl Drop for FighterDonkey {
    fn drop(&mut self) {
        unsafe { 
            FighterDonkey::call_virt_dtor(self);
        }
    }
}

pub struct FighterAnimcmdSoundShareDonkey {
    pub fighter_animcmd_sound: L2CFighterAnimcmdSoundCommon,
    reserved: [u8; 0xC8 - 0x48]
}

impl FighterAnimcmdSoundShareDonkey {
    const VTABLE_LENGTH: usize = 10;

    unsafe fn call_virt_dtor(fighter: &mut FighterAnimcmdSoundShareDonkey) {
        let dtor_ptr = *(fighter.fighter_animcmd_sound.agent.vtable as *const *const fn(&mut FighterAnimcmdSoundShareDonkey));
        let dtor: unsafe extern "C" fn(&mut FighterAnimcmdSoundShareDonkey) = std::mem::transmute(dtor_ptr);
        vtable::destroy(fighter.fighter_animcmd_sound.agent.vtable as *mut *mut skyline::libc::c_void, FighterAnimcmdSoundShareDonkey::VTABLE_LENGTH);
        dtor(fighter);
    }

    unsafe extern "C" fn del_destroy(fighter: *mut FighterAnimcmdSoundShareDonkey) {
        FighterAnimcmdSoundShareDonkey::call_virt_dtor(&mut *fighter);
        std::alloc::dealloc(fighter as *mut u8, std::alloc::Layout::from_size_align(std::mem::size_of::<FighterAnimcmdSoundShareDonkey>(), 1).unwrap());
    }

    pub fn new(obj: &mut BattleObject, boma: &mut BattleObjectModuleAccessor, state: &mut lua_State) -> FighterAnimcmdSoundShareDonkey {
        unsafe {
            let mut ret = FighterAnimcmdSoundShareDonkey { fighter_animcmd_sound: std::mem::zeroed(), reserved: [0; 0xC8 - 0x48] };
            ret.fighter_animcmd_sound.L2CFighterAnimcmdSoundCommon(obj, boma, state);
            let new_vtable = vtable::make_new(ret.fighter_animcmd_sound.agent.vtable as *const *mut skyline::libc::c_void, FighterAnimcmdSoundShareDonkey::VTABLE_LENGTH);
            ret.fighter_animcmd_sound.agent.vtable = new_vtable as u64;
            let vtable_slice = std::slice::from_raw_parts_mut(new_vtable, FighterAnimcmdSoundShareDonkey::VTABLE_LENGTH);
            vtable_slice[1] = std::mem::transmute(FighterAnimcmdSoundShareDonkey::del_destroy as *const fn());
            ret
        }
    }

    pub fn new_ptr(obj: &mut BattleObject, boma: &mut BattleObjectModuleAccessor, state: &mut lua_State) -> *mut FighterAnimcmdSoundShareDonkey{
        unsafe {
            let ret = std::alloc::alloc(std::alloc::Layout::from_size_align(std::mem::size_of::<FighterAnimcmdSoundShareDonkey>(), 1).unwrap()) as *mut FighterAnimcmdSoundShareDonkey;
            (*ret).fighter_animcmd_sound.L2CFighterAnimcmdSoundCommon(obj, boma, state);
            let new_vtable = vtable::make_new((*ret).fighter_animcmd_sound.agent.vtable as *const *mut skyline::libc::c_void, FighterAnimcmdSoundShareDonkey::VTABLE_LENGTH);
            (*ret).fighter_animcmd_sound.agent.vtable = new_vtable as u64;
            let vtable_slice = std::slice::from_raw_parts_mut(new_vtable, FighterAnimcmdSoundShareDonkey::VTABLE_LENGTH);
            vtable_slice[1] = std::mem::transmute(FighterAnimcmdSoundShareDonkey::del_destroy as *const fn());
            ret
        }
    }
}

impl Drop for FighterAnimcmdSoundShareDonkey {
    fn drop(&mut self) {
        unsafe {
            FighterAnimcmdSoundShareDonkey::call_virt_dtor(self);
        }
    }
}

pub struct FighterAnimcmdSoundDonkey {
    fighter_animcmd_sound: L2CFighterAnimcmdSoundCommon,
    reserved: [u8; 0xC8 - 0x48]
}

impl FighterAnimcmdSoundDonkey {
    const VTABLE_LENGTH: usize = 10;

    unsafe fn call_virt_dtor(fighter: &mut FighterAnimcmdSoundDonkey) {
        let dtor_ptr = *(fighter.fighter_animcmd_sound.agent.vtable as *const *const fn(&mut FighterAnimcmdSoundDonkey));
        let dtor: unsafe extern "C" fn(&mut FighterAnimcmdSoundDonkey) = std::mem::transmute(dtor_ptr);
        vtable::destroy(fighter.fighter_animcmd_sound.agent.vtable as *mut *mut skyline::libc::c_void, FighterAnimcmdSoundDonkey::VTABLE_LENGTH);
        dtor(fighter);
    }

    unsafe extern "C" fn del_destroy(fighter: *mut FighterAnimcmdSoundDonkey) {
        FighterAnimcmdSoundDonkey::call_virt_dtor(&mut *fighter);
        std::alloc::dealloc(fighter as *mut u8, std::alloc::Layout::from_size_align(std::mem::size_of::<FighterAnimcmdSoundDonkey>(), 1).unwrap());
    }

    pub fn new(obj: &mut BattleObject, boma: &mut BattleObjectModuleAccessor, state: &mut lua_State) -> FighterAnimcmdSoundDonkey {
        unsafe {
            let mut ret = FighterAnimcmdSoundDonkey { fighter_animcmd_sound: std::mem::zeroed(), reserved: [0; 0xC8 - 0x48] };
            ret.fighter_animcmd_sound.L2CFighterAnimcmdSoundCommon(obj, boma, state);
            let new_vtable = vtable::make_new(ret.fighter_animcmd_sound.agent.vtable as *const *mut skyline::libc::c_void, FighterAnimcmdSoundDonkey::VTABLE_LENGTH);
            ret.fighter_animcmd_sound.agent.vtable = new_vtable as u64;
            let vtable_slice = std::slice::from_raw_parts_mut(new_vtable, FighterAnimcmdSoundDonkey::VTABLE_LENGTH);
            vtable_slice[1] = std::mem::transmute(FighterAnimcmdSoundDonkey::del_destroy as *const fn());
            ret
        }
    }

    pub fn new_ptr(obj: &mut BattleObject, boma: &mut BattleObjectModuleAccessor, state: &mut lua_State) -> *mut FighterAnimcmdSoundDonkey {
        unsafe {
            let ret = std::alloc::alloc(std::alloc::Layout::from_size_align(std::mem::size_of::<FighterAnimcmdSoundDonkey>(), 1).unwrap()) as *mut FighterAnimcmdSoundDonkey;
            (*ret).fighter_animcmd_sound.L2CFighterAnimcmdSoundCommon(obj, boma, state);
            let new_vtable = vtable::make_new((*ret).fighter_animcmd_sound.agent.vtable as *const *mut skyline::libc::c_void, FighterAnimcmdSoundDonkey::VTABLE_LENGTH);
            (*ret).fighter_animcmd_sound.agent.vtable = new_vtable as u64;
            let vtable_slice = std::slice::from_raw_parts_mut(new_vtable, FighterAnimcmdSoundDonkey::VTABLE_LENGTH);
            vtable_slice[1] = std::mem::transmute(FighterAnimcmdSoundDonkey::del_destroy as *const fn());
            ret
        }
    }
}

impl Drop for FighterAnimcmdSoundDonkey {
    fn drop(&mut self) {
        unsafe {
            FighterAnimcmdSoundDonkey::call_virt_dtor(self);
        }
    }
}

pub struct FighterAnimcmdDonkey {
    fighter_animcmd_base: L2CFighterAnimcmdBase,
    reserved: [u8; 0xC8 - 0x48]
}

impl FighterAnimcmdDonkey {
    const VTABLE_LENGTH: usize = 9;

    unsafe fn call_virt_dtor(fighter: &mut FighterAnimcmdDonkey) {
        let dtor_ptr = *(fighter.fighter_animcmd_base.fighter_base.agent_base.agent.vtable as *const *const fn(*mut FighterAnimcmdDonkey));
        let dtor: unsafe extern "C" fn(&mut FighterAnimcmdDonkey) = std::mem::transmute(dtor_ptr);
        vtable::destroy(fighter.fighter_animcmd_base.fighter_base.agent_base.agent.vtable as *mut *mut skyline::libc::c_void, FighterAnimcmdDonkey::VTABLE_LENGTH);
        dtor(fighter);
    }

    unsafe extern "C" fn del_destroy(fighter: *mut FighterAnimcmdDonkey) {
        FighterAnimcmdDonkey::call_virt_dtor(&mut *fighter);
        std::alloc::dealloc(fighter as *mut u8, std::alloc::Layout::from_size_align(std::mem::size_of::<FighterAnimcmdDonkey>(), 1).unwrap());
    }

    pub fn new(obj: &mut BattleObject, boma: &mut BattleObjectModuleAccessor, state: &mut lua_State) -> FighterAnimcmdDonkey {
        unsafe {
            let mut ret = FighterAnimcmdDonkey { fighter_animcmd_base: std::mem::zeroed(), reserved: [0; 0xC8 - 0x48] };
            ret.fighter_animcmd_base.L2CFighterAnimcmdBase(obj, boma, state);
            let new_vtable = vtable::make_new(ret.fighter_animcmd_base.fighter_base.agent_base.agent.vtable as *const *mut skyline::libc::c_void, FighterAnimcmdDonkey::VTABLE_LENGTH);
            ret.fighter_animcmd_base.fighter_base.agent_base.agent.vtable = new_vtable as u64;
            let vtable_slice = std::slice::from_raw_parts_mut(new_vtable, FighterAnimcmdDonkey::VTABLE_LENGTH);
            vtable_slice[1] = std::mem::transmute(FighterAnimcmdDonkey::del_destroy as *const fn());
            ret
        }
    }

    pub fn new_ptr(obj: &mut BattleObject, boma: &mut BattleObjectModuleAccessor, state: &mut lua_State) -> *mut FighterAnimcmdDonkey {
        unsafe {
            let ret = std::alloc::alloc(std::alloc::Layout::from_size_align(std::mem::size_of::<FighterAnimcmdDonkey>(), 1).unwrap()) as *mut FighterAnimcmdDonkey;
            (*ret).fighter_animcmd_base.L2CFighterAnimcmdBase(obj, boma, state);
            let new_vtable = vtable::make_new((*ret).fighter_animcmd_base.fighter_base.agent_base.agent.vtable as *const *mut skyline::libc::c_void, FighterAnimcmdDonkey::VTABLE_LENGTH);
            (*ret).fighter_animcmd_base.fighter_base.agent_base.agent.vtable = new_vtable as u64;
            let vtable_slice = std::slice::from_raw_parts_mut(new_vtable, FighterAnimcmdDonkey::VTABLE_LENGTH);
            vtable_slice[1] = std::mem::transmute(FighterAnimcmdDonkey::del_destroy as *const fn());
            ret
        }
    }
}

pub struct FighterAnimcmdGameShareDonkey {
    fighter_animcmd_game: L2CFighterAnimcmdGameCommon,
    reserved: [u8; 0xC8 - 0x48]
}

impl FighterAnimcmdGameShareDonkey {
    const VTABLE_LENGTH: usize = 9;

    unsafe fn call_virt_dtor(fighter: &mut FighterAnimcmdGameShareDonkey) {
        let dtor_ptr = *(fighter.fighter_animcmd_game.agent_base.agent.vtable as *const *const fn(&mut FighterAnimcmdGameShareDonkey));
        let dtor: unsafe extern "C" fn(&mut FighterAnimcmdGameShareDonkey) = std::mem::transmute(dtor_ptr);
        vtable::destroy(fighter.fighter_animcmd_game.agent_base.agent.vtable as *mut *mut skyline::libc::c_void, FighterAnimcmdGameShareDonkey::VTABLE_LENGTH);
        dtor(fighter);
    }

    unsafe extern "C" fn del_destroy(fighter: *mut FighterAnimcmdGameShareDonkey) {
        FighterAnimcmdGameShareDonkey::call_virt_dtor(&mut *fighter);
        std::alloc::dealloc(fighter as *mut u8, std::alloc::Layout::from_size_align(std::mem::size_of::<FighterAnimcmdGameShareDonkey>(), 1).unwrap());
    }

    pub fn new(obj: &mut BattleObject, boma: &mut BattleObjectModuleAccessor, state: &mut lua_State) -> FighterAnimcmdGameShareDonkey {
        unsafe {
            let mut ret = FighterAnimcmdGameShareDonkey { fighter_animcmd_game: std::mem::zeroed(), reserved: [0; 0xC8 - 0x48] };
            ret.fighter_animcmd_game.L2CFighterAnimcmdGameCommon(obj, boma, state);
            let new_vtable = vtable::make_new(ret.fighter_animcmd_game.agent_base.agent.vtable as *const *mut skyline::libc::c_void, FighterAnimcmdGameShareDonkey::VTABLE_LENGTH);
            ret.fighter_animcmd_game.agent_base.agent.vtable = new_vtable as u64;
            let vtable_slice = std::slice::from_raw_parts_mut(new_vtable, FighterAnimcmdGameShareDonkey::VTABLE_LENGTH);
            vtable_slice[1] = std::mem::transmute(FighterAnimcmdGameShareDonkey::del_destroy as *const fn());
            ret
        }
    }

    pub fn new_ptr(obj: &mut BattleObject, boma: &mut BattleObjectModuleAccessor, state: &mut lua_State) -> *mut FighterAnimcmdGameShareDonkey {
        unsafe {
            let mut ret = std::alloc::alloc(std::alloc::Layout::from_size_align(std::mem::size_of::<FighterAnimcmdGameShareDonkey>(), 1).unwrap()) as *mut FighterAnimcmdGameShareDonkey;
            (*ret).fighter_animcmd_game.L2CFighterAnimcmdGameCommon(obj, boma, state);
            let new_vtable = vtable::make_new((*ret).fighter_animcmd_game.agent_base.agent.vtable as *const *mut skyline::libc::c_void, FighterAnimcmdGameShareDonkey::VTABLE_LENGTH);
            (*ret).fighter_animcmd_game.agent_base.agent.vtable = new_vtable as u64;
            let vtable_slice = std::slice::from_raw_parts_mut(new_vtable, FighterAnimcmdGameShareDonkey::VTABLE_LENGTH);
            vtable_slice[1] = std::mem::transmute(FighterAnimcmdGameShareDonkey::del_destroy as *const fn());
            ret
        }
    }
}

pub struct FighterAnimcmdGameDonkey {
    fighter_animcmd_game: L2CFighterAnimcmdGameCommon,
    reserved: [u8; 0xC8 - 0x48]
}

impl FighterAnimcmdGameDonkey {
    const VTABLE_LENGTH: usize = 9;

    unsafe fn call_virt_dtor(fighter: &mut FighterAnimcmdGameDonkey) {
        let dtor_ptr = *(fighter.fighter_animcmd_game.agent_base.agent.vtable as *const *const fn(&mut FighterAnimcmdGameDonkey));
        let dtor: unsafe extern "C" fn(&mut FighterAnimcmdGameDonkey) = std::mem::transmute(dtor_ptr);
        vtable::destroy(fighter.fighter_animcmd_game.agent_base.agent.vtable as *mut *mut skyline::libc::c_void, FighterAnimcmdGameDonkey::VTABLE_LENGTH);
        dtor(fighter);
    }

    unsafe extern "C" fn del_destroy(fighter: *mut FighterAnimcmdGameDonkey) {
        FighterAnimcmdGameDonkey::call_virt_dtor(&mut *fighter);
        std::alloc::dealloc(fighter as *mut u8, std::alloc::Layout::from_size_align(std::mem::size_of::<FighterAnimcmdGameDonkey>(), 1).unwrap());
    }

    pub fn new(obj: &mut BattleObject, boma: &mut BattleObjectModuleAccessor, state: &mut lua_State) -> FighterAnimcmdGameDonkey {
        unsafe {
            let mut ret = FighterAnimcmdGameDonkey { fighter_animcmd_game: std::mem::zeroed(), reserved: [0; 0xC8 - 0x48] };
            ret.fighter_animcmd_game.L2CFighterAnimcmdGameCommon(obj, boma, state);
            let new_vtable = vtable::make_new(ret.fighter_animcmd_game.agent_base.agent.vtable as *const *mut skyline::libc::c_void, FighterAnimcmdGameDonkey::VTABLE_LENGTH);
            ret.fighter_animcmd_game.agent_base.agent.vtable = new_vtable as u64;
            let vtable_slice = std::slice::from_raw_parts_mut(new_vtable, FighterAnimcmdGameDonkey::VTABLE_LENGTH);
            vtable_slice[1] = std::mem::transmute(FighterAnimcmdGameDonkey::del_destroy as *const fn());
            ret
        }
    }

    pub fn new_ptr(obj: &mut BattleObject, boma: &mut BattleObjectModuleAccessor, state: &mut lua_State) -> *mut FighterAnimcmdGameDonkey {
        unsafe {
            let mut ret = std::alloc::alloc(std::alloc::Layout::from_size_align(std::mem::size_of::<FighterAnimcmdGameDonkey>(), 1).unwrap()) as *mut FighterAnimcmdGameDonkey;
            (*ret).fighter_animcmd_game.L2CFighterAnimcmdGameCommon(obj, boma, state);
            let new_vtable = vtable::make_new((*ret).fighter_animcmd_game.agent_base.agent.vtable as *const *mut skyline::libc::c_void, FighterAnimcmdGameDonkey::VTABLE_LENGTH);
            (*ret).fighter_animcmd_game.agent_base.agent.vtable = new_vtable as u64;
            let vtable_slice = std::slice::from_raw_parts_mut(new_vtable, FighterAnimcmdGameDonkey::VTABLE_LENGTH);
            vtable_slice[1] = std::mem::transmute(FighterAnimcmdGameDonkey::del_destroy as *const fn());
            ret
        }
    }
}

pub struct FighterAnimcmdExpressionShareDonkey {
    fighter_animcmd_expression: L2CFighterAnimcmdExpressionCommon,
    reserved: [u8; 0xC8 - 0x48]
}

impl FighterAnimcmdExpressionShareDonkey {
    const VTABLE_LENGTH: usize = 9;

    unsafe fn call_virt_dtor(fighter: &mut FighterAnimcmdExpressionShareDonkey) {
        let dtor_ptr = *(fighter.fighter_animcmd_expression.agent.vtable as *const *const fn(&mut FighterAnimcmdExpressionShareDonkey));
        let dtor: unsafe extern "C" fn(&mut FighterAnimcmdExpressionShareDonkey) = std::mem::transmute(dtor_ptr);
        vtable::destroy(fighter.fighter_animcmd_expression.agent.vtable as *mut *mut skyline::libc::c_void, FighterAnimcmdExpressionShareDonkey::VTABLE_LENGTH);
        dtor(fighter);
    }

    unsafe extern "C" fn del_destroy(fighter: *mut FighterAnimcmdExpressionShareDonkey) {
        FighterAnimcmdExpressionShareDonkey::call_virt_dtor(&mut *fighter);
        std::alloc::dealloc(fighter as *mut u8, std::alloc::Layout::from_size_align(std::mem::size_of::<FighterAnimcmdExpressionShareDonkey>(), 1).unwrap());
    }

    pub fn new(obj: &mut BattleObject, boma: &mut BattleObjectModuleAccessor, state: &mut lua_State) -> FighterAnimcmdExpressionShareDonkey {
        unsafe {
            let mut ret = FighterAnimcmdExpressionShareDonkey { fighter_animcmd_expression: std::mem::zeroed(), reserved: [0; 0xC8 - 0x48] };
            ret.fighter_animcmd_expression.L2CFighterAnimcmdExpressionCommon(obj, boma, state);
            let new_vtable = vtable::make_new(ret.fighter_animcmd_expression.agent.vtable as *const *mut skyline::libc::c_void, FighterAnimcmdExpressionShareDonkey::VTABLE_LENGTH);
            ret.fighter_animcmd_expression.agent.vtable = new_vtable as u64;
            let vtable_slice = std::slice::from_raw_parts_mut(new_vtable, FighterAnimcmdExpressionShareDonkey::VTABLE_LENGTH);
            vtable_slice[1] = std::mem::transmute(FighterAnimcmdExpressionShareDonkey::del_destroy as *const fn());
            ret
        }
    }

    pub fn new_ptr(obj: &mut BattleObject, boma: &mut BattleObjectModuleAccessor, state: &mut lua_State) -> *mut FighterAnimcmdExpressionShareDonkey {
        unsafe {
            let mut ret = std::alloc::alloc(std::alloc::Layout::from_size_align(std::mem::size_of::<FighterAnimcmdExpressionShareDonkey>(), 1).unwrap()) as *mut FighterAnimcmdExpressionShareDonkey;
            (*ret).fighter_animcmd_expression.L2CFighterAnimcmdExpressionCommon(obj, boma, state);
            let new_vtable = vtable::make_new((*ret).fighter_animcmd_expression.agent.vtable as *const *mut skyline::libc::c_void, FighterAnimcmdExpressionShareDonkey::VTABLE_LENGTH);
            (*ret).fighter_animcmd_expression.agent.vtable = new_vtable as u64;
            let vtable_slice = std::slice::from_raw_parts_mut(new_vtable, FighterAnimcmdExpressionShareDonkey::VTABLE_LENGTH);
            vtable_slice[1] = std::mem::transmute(FighterAnimcmdExpressionShareDonkey::del_destroy as *const fn());
            ret
        }
    }
}

pub struct FighterAnimcmdExpressionDonkey {
    fighter_animcmd_expression: L2CFighterAnimcmdExpressionCommon,
    reserved: [u8; 0xC8 - 0x48]
}

impl FighterAnimcmdExpressionDonkey {
    const VTABLE_LENGTH: usize = 9;

    unsafe fn call_virt_dtor(fighter: &mut FighterAnimcmdExpressionDonkey) {
        let dtor_ptr = *(fighter.fighter_animcmd_expression.agent.vtable as *const *const fn(&mut FighterAnimcmdExpressionDonkey));
        let dtor: unsafe extern "C" fn(&mut FighterAnimcmdExpressionDonkey) = std::mem::transmute(dtor_ptr);
        vtable::destroy(fighter.fighter_animcmd_expression.agent.vtable as *mut *mut skyline::libc::c_void, FighterAnimcmdExpressionDonkey::VTABLE_LENGTH);
        dtor(fighter);
    }

    unsafe extern "C" fn del_destroy(fighter: *mut FighterAnimcmdExpressionDonkey) {
        FighterAnimcmdExpressionDonkey::call_virt_dtor(&mut *fighter);
        std::alloc::dealloc(fighter as *mut u8, std::alloc::Layout::from_size_align(std::mem::size_of::<FighterAnimcmdExpressionDonkey>(), 1).unwrap());
    }

    pub fn new(obj: &mut BattleObject, boma: &mut BattleObjectModuleAccessor, state: &mut lua_State) -> FighterAnimcmdExpressionDonkey {
        unsafe {
            let mut ret = FighterAnimcmdExpressionDonkey { fighter_animcmd_expression: std::mem::zeroed(), reserved: [0; 0xC8 - 0x48] };
            ret.fighter_animcmd_expression.L2CFighterAnimcmdExpressionCommon(obj, boma, state);
            let new_vtable = vtable::make_new(ret.fighter_animcmd_expression.agent.vtable as *const *mut skyline::libc::c_void, FighterAnimcmdExpressionDonkey::VTABLE_LENGTH);
            ret.fighter_animcmd_expression.agent.vtable = new_vtable as u64;
            let vtable_slice = std::slice::from_raw_parts_mut(new_vtable, FighterAnimcmdExpressionDonkey::VTABLE_LENGTH);
            vtable_slice[1] = std::mem::transmute(FighterAnimcmdExpressionDonkey::del_destroy as *const fn());
            ret
        }
    }

    pub fn new_ptr(obj: &mut BattleObject, boma: &mut BattleObjectModuleAccessor, state: &mut lua_State) -> *mut FighterAnimcmdExpressionDonkey {
        unsafe {
            let mut ret = std::alloc::alloc(std::alloc::Layout::from_size_align(std::mem::size_of::<FighterAnimcmdExpressionDonkey>(), 1).unwrap()) as *mut FighterAnimcmdExpressionDonkey;
            (*ret).fighter_animcmd_expression.L2CFighterAnimcmdExpressionCommon(obj, boma, state);
            let new_vtable = vtable::make_new((*ret).fighter_animcmd_expression.agent.vtable as *const *mut skyline::libc::c_void, FighterAnimcmdExpressionDonkey::VTABLE_LENGTH);
            (*ret).fighter_animcmd_expression.agent.vtable = new_vtable as u64;
            let vtable_slice = std::slice::from_raw_parts_mut(new_vtable, FighterAnimcmdExpressionDonkey::VTABLE_LENGTH);
            vtable_slice[1] = std::mem::transmute(FighterAnimcmdExpressionDonkey::del_destroy as *const fn());
            ret
        }
    }
}

pub struct FighterAnimcmdEffectShareDonkey {
    fighter_animcmd_effect: L2CFighterAnimcmdEffectCommon,
    reserved: [u8; 0xC8 - 0x48]
}

impl FighterAnimcmdEffectShareDonkey {
    const VTABLE_LENGTH: usize = 12;

    unsafe fn call_virt_dtor(fighter: &mut FighterAnimcmdEffectShareDonkey) {
        let dtor_ptr = *(fighter.fighter_animcmd_effect.agent.vtable as *const *const fn(&mut FighterAnimcmdEffectShareDonkey));
        let dtor: unsafe extern "C" fn(&mut FighterAnimcmdEffectShareDonkey) = std::mem::transmute(dtor_ptr);
        vtable::destroy(fighter.fighter_animcmd_effect.agent.vtable as *mut *mut skyline::libc::c_void, FighterAnimcmdEffectShareDonkey::VTABLE_LENGTH);
        dtor(fighter);
    }

    unsafe extern "C" fn del_destroy(fighter: *mut FighterAnimcmdEffectShareDonkey) {
        FighterAnimcmdEffectShareDonkey::call_virt_dtor(&mut *fighter);
        std::alloc::dealloc(fighter as *mut u8, std::alloc::Layout::from_size_align(std::mem::size_of::<FighterAnimcmdEffectShareDonkey>(), 1).unwrap());
    }

    pub fn new(obj: &mut BattleObject, boma: &mut BattleObjectModuleAccessor, state: &mut lua_State) -> FighterAnimcmdEffectShareDonkey {
        unsafe {
            let mut ret = FighterAnimcmdEffectShareDonkey { fighter_animcmd_effect: std::mem::zeroed(), reserved: [0; 0xC8 - 0x48] };
            ret.fighter_animcmd_effect.L2CFighterAnimcmdEffectCommon(obj, boma, state);
            let new_vtable = vtable::make_new(ret.fighter_animcmd_effect.agent.vtable as *const *mut skyline::libc::c_void, FighterAnimcmdEffectShareDonkey::VTABLE_LENGTH);
            ret.fighter_animcmd_effect.agent.vtable = new_vtable as u64;
            let vtable_slice = std::slice::from_raw_parts_mut(new_vtable, FighterAnimcmdEffectShareDonkey::VTABLE_LENGTH);
            vtable_slice[1] = std::mem::transmute(FighterAnimcmdEffectShareDonkey::del_destroy as *const fn());
            ret
        }
    }

    pub fn new_ptr(obj: &mut BattleObject, boma: &mut BattleObjectModuleAccessor, state: &mut lua_State) -> *mut FighterAnimcmdEffectShareDonkey {
        unsafe {
            let mut ret = std::alloc::alloc(std::alloc::Layout::from_size_align(std::mem::size_of::<FighterAnimcmdEffectShareDonkey>(), 1).unwrap()) as *mut FighterAnimcmdEffectShareDonkey;
            (*ret).fighter_animcmd_effect.L2CFighterAnimcmdEffectCommon(obj, boma, state);
            let new_vtable = vtable::make_new((*ret).fighter_animcmd_effect.agent.vtable as *const *mut skyline::libc::c_void, FighterAnimcmdEffectShareDonkey::VTABLE_LENGTH);
            (*ret).fighter_animcmd_effect.agent.vtable = new_vtable as u64;
            let vtable_slice = std::slice::from_raw_parts_mut(new_vtable, FighterAnimcmdEffectShareDonkey::VTABLE_LENGTH);
            vtable_slice[1] = std::mem::transmute(FighterAnimcmdEffectShareDonkey::del_destroy as *const fn());
            ret
        }
    }
}

pub struct FighterAnimcmdEffectDonkey {
    fighter_animcmd_effect: L2CFighterAnimcmdEffectCommon,
    reserved: [u8; 0xC8 - 0x48]
}

impl FighterAnimcmdEffectDonkey {
    const VTABLE_LENGTH: usize = 12;

    unsafe fn call_virt_dtor(fighter: &mut FighterAnimcmdEffectDonkey) {
        let dtor_ptr = *(fighter.fighter_animcmd_effect.agent.vtable as *const *const fn(&mut FighterAnimcmdEffectDonkey));
        let dtor: unsafe extern "C" fn(&mut FighterAnimcmdEffectDonkey) = std::mem::transmute(dtor_ptr);
        vtable::destroy(fighter.fighter_animcmd_effect.agent.vtable as *mut *mut skyline::libc::c_void, FighterAnimcmdEffectDonkey::VTABLE_LENGTH);
        dtor(fighter);
    }

    unsafe extern "C" fn del_destroy(fighter: *mut FighterAnimcmdEffectDonkey) {
        FighterAnimcmdEffectDonkey::call_virt_dtor(&mut *fighter);
        std::alloc::dealloc(fighter as *mut u8, std::alloc::Layout::from_size_align(std::mem::size_of::<FighterAnimcmdEffectDonkey>(), 1).unwrap());
    }

    pub fn new(obj: &mut BattleObject, boma: &mut BattleObjectModuleAccessor, state: &mut lua_State) -> FighterAnimcmdEffectDonkey {
        unsafe {
            let mut ret = FighterAnimcmdEffectDonkey { fighter_animcmd_effect: std::mem::zeroed(), reserved: [0; 0xC8 - 0x48] };
            ret.fighter_animcmd_effect.L2CFighterAnimcmdEffectCommon(obj, boma, state);
            let new_vtable = vtable::make_new(ret.fighter_animcmd_effect.agent.vtable as *const *mut skyline::libc::c_void, FighterAnimcmdEffectDonkey::VTABLE_LENGTH);
            ret.fighter_animcmd_effect.agent.vtable = new_vtable as u64;
            let vtable_slice = std::slice::from_raw_parts_mut(new_vtable, FighterAnimcmdEffectDonkey::VTABLE_LENGTH);
            vtable_slice[1] = std::mem::transmute(FighterAnimcmdEffectDonkey::del_destroy as *const fn());
            ret
        }
    }

    pub fn new_ptr(obj: &mut BattleObject, boma: &mut BattleObjectModuleAccessor, state: &mut lua_State) -> *mut FighterAnimcmdEffectDonkey {
        unsafe {
            let mut ret = std::alloc::alloc(std::alloc::Layout::from_size_align(std::mem::size_of::<FighterAnimcmdEffectDonkey>(), 1).unwrap()) as *mut FighterAnimcmdEffectDonkey;
            (*ret).fighter_animcmd_effect.L2CFighterAnimcmdEffectCommon(obj, boma, state);
            let new_vtable = vtable::make_new((*ret).fighter_animcmd_effect.agent.vtable as *const *mut skyline::libc::c_void, FighterAnimcmdEffectDonkey::VTABLE_LENGTH);
            (*ret).fighter_animcmd_effect.agent.vtable = new_vtable as u64;
            let vtable_slice = std::slice::from_raw_parts_mut(new_vtable, FighterAnimcmdEffectDonkey::VTABLE_LENGTH);
            vtable_slice[1] = std::mem::transmute(FighterAnimcmdEffectDonkey::del_destroy as *const fn());
            ret
        }
    }
}

pub struct FighterAIModeDonkey {
    fighter_ai_mode_base: L2CFighterAIModeBase,
    reserved: [u8; 0x900]
}

impl FighterAIModeDonkey {
    const VTABLE_LENGTH: usize = 47;

    unsafe fn call_virt_dtor(fighter: &mut FighterAIModeDonkey) {
        let dtor_ptr = *(fighter.fighter_ai_mode_base.fighter_base.agent_base.agent.vtable as *const *const fn(&mut FighterAIModeDonkey));
        let dtor: unsafe extern "C" fn(&mut FighterAIModeDonkey) = std::mem::transmute(dtor_ptr);
        vtable::destroy(fighter.fighter_ai_mode_base.fighter_base.agent_base.agent.vtable as *mut *mut skyline::libc::c_void, FighterAIModeDonkey::VTABLE_LENGTH);
        dtor(fighter);
    }

    unsafe extern "C" fn del_destroy(fighter: *mut FighterAIModeDonkey) {
        FighterAIModeDonkey::call_virt_dtor(&mut *fighter);
        std::alloc::dealloc(fighter as *mut u8, std::alloc::Layout::from_size_align(std::mem::size_of::<FighterAIModeDonkey>(), 1).unwrap());
    }

    pub fn new(obj: &mut BattleObject, boma: &mut BattleObjectModuleAccessor, state: &mut lua_State) -> FighterAIModeDonkey {
        unsafe {
            let mut ret = FighterAIModeDonkey { fighter_ai_mode_base: std::mem::zeroed(), reserved: [0; 0x900] };
            ret.fighter_ai_mode_base.L2CFighterAIModeBase(obj, boma, state);
            let new_vtable = vtable::make_new(ret.fighter_ai_mode_base.fighter_base.agent_base.agent.vtable as *const *mut skyline::libc::c_void, FighterAIModeDonkey::VTABLE_LENGTH);
            ret.fighter_ai_mode_base.fighter_base.agent_base.agent.vtable = new_vtable as u64;
            let vtable_slice = std::slice::from_raw_parts_mut(new_vtable, FighterAIModeDonkey::VTABLE_LENGTH);
            vtable_slice[1] = std::mem::transmute(FighterAIModeDonkey::del_destroy as *const fn());
            ret
        }
    }

    pub fn new_ptr(obj: &mut BattleObject, boma: &mut BattleObjectModuleAccessor, state: &mut lua_State) -> *mut FighterAIModeDonkey {
        unsafe {
            let mut ret = std::alloc::alloc(std::alloc::Layout::from_size_align(std::mem::size_of::<FighterAIModeDonkey>(), 1).unwrap()) as *mut FighterAIModeDonkey;
            (*ret).fighter_ai_mode_base.L2CFighterAIModeBase(obj, boma, state);
            let new_vtable = vtable::make_new((*ret).fighter_ai_mode_base.fighter_base.agent_base.agent.vtable as *const *mut skyline::libc::c_void, FighterAIModeDonkey::VTABLE_LENGTH);
            (*ret).fighter_ai_mode_base.fighter_base.agent_base.agent.vtable = new_vtable as u64;
            let vtable_slice = std::slice::from_raw_parts_mut(new_vtable, FighterAIModeDonkey::VTABLE_LENGTH);
            vtable_slice[1] = std::mem::transmute(FighterAIModeDonkey::del_destroy as *const fn());
            ret
        }
    }
}

pub struct FighterAIActionDonkey {
    fighter_ai_action_base: L2CFighterAIActionBase,
    reserved: [u8; 0x550]
}

impl FighterAIActionDonkey {
    const VTABLE_LENGTH: usize = 9;

    unsafe fn call_virt_dtor(fighter: &mut FighterAIActionDonkey) {
        let dtor_ptr = *(fighter.fighter_ai_action_base.fighter_base.agent_base.agent.vtable as *const *const fn(&mut FighterAIActionDonkey));
        let dtor: unsafe extern "C" fn(&mut FighterAIActionDonkey) = std::mem::transmute(dtor_ptr);
        vtable::destroy(fighter.fighter_ai_action_base.fighter_base.agent_base.agent.vtable as *mut *mut skyline::libc::c_void, FighterAIActionDonkey::VTABLE_LENGTH);
        dtor(fighter);
    }

    unsafe extern "C" fn del_destroy(fighter: *mut FighterAIActionDonkey) {
        FighterAIActionDonkey::call_virt_dtor(&mut *fighter);
        std::alloc::dealloc(fighter as *mut u8, std::alloc::Layout::from_size_align(std::mem::size_of::<FighterAIActionDonkey>(), 1).unwrap());
    }

    pub fn new(obj: &mut BattleObject, boma: &mut BattleObjectModuleAccessor, state: &mut lua_State) -> FighterAIActionDonkey {
        unsafe {
            let mut ret = FighterAIActionDonkey { fighter_ai_action_base: std::mem::zeroed(), reserved: [0; 0x550] };
            ret.fighter_ai_action_base.L2CFighterAIActionBase(obj, boma, state);
            let new_vtable = vtable::make_new(ret.fighter_ai_action_base.fighter_base.agent_base.agent.vtable as *const *mut skyline::libc::c_void, FighterAIActionDonkey::VTABLE_LENGTH);
            ret.fighter_ai_action_base.fighter_base.agent_base.agent.vtable = new_vtable as u64;
            let vtable_slice = std::slice::from_raw_parts_mut(new_vtable, FighterAIActionDonkey::VTABLE_LENGTH);
            vtable_slice[1] = std::mem::transmute(FighterAIActionDonkey::del_destroy as *const fn());
            ret
        }
    }

    pub fn new_ptr(obj: &mut BattleObject, boma: &mut BattleObjectModuleAccessor, state: &mut lua_State) -> *mut FighterAIActionDonkey {
        unsafe {
            let mut ret = std::alloc::alloc(std::alloc::Layout::from_size_align(std::mem::size_of::<FighterAIActionDonkey>(), 1).unwrap()) as *mut FighterAIActionDonkey;
            (*ret).fighter_ai_action_base.L2CFighterAIActionBase(obj, boma, state);
            let new_vtable = vtable::make_new((*ret).fighter_ai_action_base.fighter_base.agent_base.agent.vtable as *const *mut skyline::libc::c_void, FighterAIActionDonkey::VTABLE_LENGTH);
            (*ret).fighter_ai_action_base.fighter_base.agent_base.agent.vtable = new_vtable as u64;
            let vtable_slice = std::slice::from_raw_parts_mut(new_vtable, FighterAIActionDonkey::VTABLE_LENGTH);
            vtable_slice[1] = std::mem::transmute(FighterAIActionDonkey::del_destroy as *const fn());
            ret
        }
    }
}