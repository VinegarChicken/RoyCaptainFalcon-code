use smash::{
    *,
    hash40,
    phx::{Hash40, Vector3f},
    lib::lua_const::*,
    app::*,
    app::lua_bind::*,
    lua2cpp::{L2CFighterCommon, L2CAgentBase}
};
use smashline::*;


#[acmd_script( agent = "roy", script = "effect_speciallw", category = ACMD_SOUND )]
unsafe fn sound_roy_speciallw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {

    });
}

#[acmd_script( agent = "roy", script = "effect_specialairlw", category = ACMD_SOUND )]
unsafe fn sound_roy_specialairlw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {

    });
}


#[acmd_script( agent = "roy", script = "sound_specialnendmax", category = ACMD_SOUND )]
unsafe fn sound_roy_specialn_end_max(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=51)
        if(is_excute){
            STOP_SE(hash40("se_roy_special_n01"))
            PLAY_SE(hash40("se_roy_special_n05"))
            PLAY_SE(hash40("vc_roy_special_n03"))
        }
    });
}

#[acmd_script( agent = "roy", script = "sound_specialairnendmax", category = ACMD_SOUND )]
unsafe fn sound_roy_specialairn_end_max(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=51)
        if(is_excute){
            STOP_SE(hash40("se_roy_special_n01"))
            PLAY_SE(hash40("se_roy_special_n05"))
            PLAY_SE(hash40("vc_roy_special_n03"))
        }
    });
}

#[acmd_script( agent = "roy", script = "sound_speciallwhit", category = ACMD_SOUND )]
unsafe fn sound_roy_speciallwhit(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=1)
        if(is_excute){
            PLAY_SE(hash40("se_roy_special_l02"))
        }
        wait(Frames=51)
        if(is_excute){
            PLAY_SE(hash40("se_roy_special_l03"))
            PLAY_SEQUENCE(hash40("seq_roy_rnd_special_l"))
        }
    });
}

#[acmd_script( agent = "roy", script = "sound_specialairlwhit", category = ACMD_SOUND )]
unsafe fn sound_roy_specialairlwhit(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=1)
        if(is_excute){
            PLAY_SE(hash40("se_roy_special_l02"))
        }
        wait(Frames=51)
        if(is_excute){
            PLAY_SE(hash40("se_roy_special_l03"))
            PLAY_SEQUENCE(hash40("seq_roy_rnd_special_l"))
        }
    });
}

pub fn install() {
    smashline::install_acmd_scripts!(
        //sound_roy_speciallw,
        //sound_roy_specialairlw,
        sound_roy_specialn_end_max,
        sound_roy_specialairn_end_max,
        sound_roy_speciallwhit,
        sound_roy_specialairlwhit

    );
}
