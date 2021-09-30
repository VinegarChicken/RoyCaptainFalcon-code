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



#[acmd_script( agent = "roy", script = "effect_attackdash", category = ACMD_EFFECT )]
unsafe fn effect_roy_attackdash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=7)
        if(is_excute){
            EFFECT_FOLLOW(hash40("sys_damage_fire"), hash40("top"), 0, 8, 12, 0, 0, 0, 0.85, true)
            LANDING_EFFECT(hash40("sys_atk_smoke"), hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
        }
        frame(Frame=13)
        if(is_excute){
            EFFECT_FOLLOW(hash40("sys_damage_fire"), hash40("top"), 0, 8, 12, 0, 0, 0, 0.85, true)
        }
        frame(Frame=28)
        if(is_excute){
            FOOT_EFFECT(hash40("null"), hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
        }
    });
}


#[acmd_script( agent = "roy", script = "effect_attacks3", category = ACMD_EFFECT )]
unsafe fn effect_roy_attacks3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=7)
        if(is_excute){
                EFFECT(hash40("sys_attack_arc"), hash40("top"), 0, 10, 5.5, 100, -100, -112, 1, 0, 0, 0, 0, 0, 0, true)
                LAST_EFFECT_SET_COLOR(0.90, 0.3, 0.0)
        }
        frame(Frame=11)
        if(is_excute){
            FOOT_EFFECT(hash40("sys_run_smoke"), hash40("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false)
        }
    });
}

#[acmd_script( agent = "roy", script = "effect_attackhi3", category = ACMD_EFFECT )]
unsafe fn effect_roy_attackhi3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=11)
        if(is_excute){
            FOOT_EFFECT(hash40("sys_run_smoke"), hash40("top"), 0, 0, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, false)
        }
        frame(Frame=18)
        if(is_excute){
            EFFECT_ALPHA(hash40("sys_attack_arc_b"), hash40("top"), 0, 12.5, 2, 11, -40, -84, 1.2, 0, 0, 0, 0, 0, 0, true, 0.65)
            LAST_EFFECT_SET_COLOR(0.90, 0.3, 0.0)
        }
        frame(Frame=21)
        if(is_excute){
            FOOT_EFFECT(hash40("sys_down_smoke"), hash40("top"), 11, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false)
        }
    });
}


#[acmd_script( agent = "roy", script = "effect_attacks4", category = ACMD_EFFECT )]
unsafe fn effect_roy_attacks4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {

    });

}



#[acmd_script( agent = "roy", script = "effect_attacklw4", category = ACMD_EFFECT )]
unsafe fn effect_roy_attacklw4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=26)
        if(is_excute){
            EFFECT(hash40("sys_smash_flash"), hash40("top"), 4, 16, 8, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true)
        }
        frame(Frame=28)
        if(is_excute){
            LANDING_EFFECT(hash40("sys_atk_smoke"), hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
        }
        frame(Frame=29)
        if(is_excute){
            AFTER_IMAGE4_ON_arg29(hash40("tex_roy_sword1"), hash40("tex_roy_sword2"), 5, hash40("sword1"), 0, 0, -0.8, hash40("sword1"), -0.0, -0.0, 14.5, true, hash40("roy_sword"), hash40("sword1"), 0, 0, 0, 0, 0, 0, 1, 0, EFFECT_AXIS_X, 0, TRAIL_BLEND_ALPHA, 101, TRAIL_CULL_NONE, 1.4, 0.2)
        }
        frame(Frame=37)
        if(is_excute){
            AFTER_IMAGE_OFF(6)
        }
    });
}


#[acmd_script( agent = "roy", script = "effect_attackhi4", category = ACMD_EFFECT )]
unsafe fn effect_roy_attackhi4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=2)
        if(is_excute){
            EFFECT(hash40("sys_smash_flash"), hash40("sword1"), -0.0, -0.0, 10, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true)
        }
        frame(Frame=7)
        if(is_excute){
            EFFECT_FOLLOW(hash40("roy_sword"), hash40("sword1"), 0, 0, 0, 0, 0, 0, 1, true)
            EFFECT_FOLLOW(hash40("roy_smash_fire"), hash40("sword1"), 0, 0, 0, 0, 0, 0, 0.9, true)
        }
        frame(Frame=13)
        if(is_excute){
            LANDING_EFFECT(hash40("sys_v_smoke_a"), hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
        }
        frame(Frame=19)
        if(is_excute){
            //EFFECT(hash40("roy_smash_bomb"), hash40("sword1"), 0, 0, 7, 90, 0, 0, 0.48, 0, 0, 0, 0, 0, 0, true)
            LAST_EFFECT_SET_RATE(1.25)
        }
        frame(Frame=35)
        if(is_excute){
            EFFECT_OFF_KIND(hash40("roy_sword"), false, false)
            EFFECT_OFF_KIND(hash40("roy_smash_bomb"), true, true)
        }
        frame(Frame=41)
        if(is_excute){
            EFFECT_FOLLOW(hash40("captain_smash_arc"), hash40("top"), 0, 17, 1.5, 73, -15, 165, 1.05, true)
        }
        frame(Frame=48)
        if(is_excute){
            LANDING_EFFECT(hash40("sys_down_smoke"), hash40("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false)
        }
    });
}


#[acmd_script( agent = "roy", script = "effect_attackairn", category = ACMD_EFFECT )]
unsafe fn effect_roy_attackairn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=6)
        if(is_excute){
            EFFECT_FOLLOW_ALPHA(hash40("sys_attack_arc"), hash40("top"), 0, 2.5, 2, -148, -141, -3, 1.1, true, 0.7)
            LAST_EFFECT_SET_RATE(1.5)
        }
        frame(Frame=15)
        if(is_excute){
            AFTER_IMAGE4_ON_arg29(hash40("tex_roy_sword1"), hash40("tex_roy_sword2"), 10, hash40("sword1"), 0, 0, -0.8, hash40("sword1"), -0.0, -0.0, 14.5, true, hash40("roy_sword"), hash40("sword1"), 0, 0, 0, 0, 0, 0, 1, 0, EFFECT_AXIS_X, 0, TRAIL_BLEND_ALPHA, 101, TRAIL_CULL_NONE, 1.3, 0.2)
            EFFECT_FOLLOW(hash40("roy_sword_light"), hash40("sword1"), 0, 0, 10.5, 0, 0, 0, 1, true)
            LAST_EFFECT_SET_ALPHA(0.6)
        }
        frame(Frame=23)
        if(is_excute){
            AFTER_IMAGE_OFF(4)
            EFFECT_OFF_KIND(hash40("roy_sword_light"), false, true)
        }
    });
}


#[acmd_script( agent = "roy", script = "effect_attackairf", category = ACMD_EFFECT )]
unsafe fn effect_roy_attackairf(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=7)
        if(is_excute){
            EFFECT_FOLLOW(hash40("sys_smash_flash"), hash40("top"), 0, 10, 8, -20, 0, 0, 0.7, true)
        }
        frame(Frame=14)
        if(is_excute){
            EFFECT_FOLLOW(hash40("sys_attack_impact"), hash40("top"), 2, 7, 6, 0, 0, 0, 1.3, true)
            EFFECT_FOLLOW_ALPHA(hash40("captain_at_thrust"), hash40("top"), 0, 5, 0, -20, 10, 0, 0.85, true, 0.5)
            LAST_EFFECT_SET_RATE(1)
        }
    });
}

#[acmd_script( agent = "roy", script = "effect_attackairb", category = ACMD_EFFECT )]
unsafe fn effect_roy_attackairb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=9)
        if(is_excute){
            EFFECT_FOLLOW_ALPHA(hash40("sys_attack_line"), hash40("top"), 5.5, 17.5, 4, -213, 10, 0, 1.2, true, 0.8)
            LAST_EFFECT_SET_COLOR(0.90, 0.3, 0.0)
        }
        frame(Frame=11)
        if(is_excute){
            EFFECT_FOLLOW_ALPHA(hash40("sys_attack_impact"), hash40("top"), 2, 6, -15, 0, 0, 0, 1.15, true, 0.5)
            LAST_EFFECT_SET_COLOR(0.90, 0.3, 0.0)
        }
        frame(Frame=16)
        if(is_excute){
            AFTER_IMAGE4_ON_arg29(hash40("tex_roy_sword1"), hash40("tex_roy_sword2"), 7, hash40("sword1"), 0, 0, -0.8, hash40("sword1"), -0.0, -0.0, 14.5, true, hash40("roy_sword"), hash40("sword1"), 0, 0, 0, 0, 0, 0, 1, 0, EFFECT_AXIS_X, 0, TRAIL_BLEND_ALPHA, 101, TRAIL_CULL_NONE, 1.3, 0.2)
        }
        frame(Frame=21)
        if(is_excute){
            AFTER_IMAGE_OFF(2)
        }
    });
}

#[acmd_script( agent = "roy", script = "effect_attackairhi", category = ACMD_EFFECT )]
unsafe fn effect_roy_attackairhi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=7)
        if(is_excute){
            EFFECT_FOLLOW_ALPHA(hash40("sys_attack_arc_d"), hash40("top"), 0, 14, 3, 0, 100, 90, 1.2, true, 0.75)
            LAST_EFFECT_SET_COLOR(0.90, 0.3, 0.0)
            LAST_EFFECT_SET_RATE(0.8)
        }
    });
}

#[acmd_script( agent = "roy", script = "effect_attackairlw", category = ACMD_EFFECT )]
unsafe fn effect_roy_attackairlw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=3)
        if(is_excute){
            EFFECT(hash40("sys_smash_flash"), hash40("toer"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true)
        }
        frame(Frame=15)
        if(is_excute){
            EFFECT_FOLLOW_ALPHA(hash40("roy_attack_fire"), hash40("top"), 0, 15, 0, 90, 0, 0, 1.1, true, 0.75)
            EFFECT_FOLLOW_ALPHA(hash40("roy_sttack_fire"), hash40("top"), 2, 15, 2, 90, 0, 0, 1.1, true, 0.75)
            EFFECT_FOLLOW(hash40("sys_attack_speedline"), hash40("top"), 0, -4, 0, -90, 0, 0, 1.5, true)
        }
        frame(Frame=16)
        if(is_excute){
            //EFFECT_FOLLOW(hash40("roy_smash_bomb"), hash40("top"), 0, -5, 1, 0, 0, 0, 0.48, true)
            EFFECT_FOLLOW(hash40("roy_eruption_max"), hash40("top"), 0, -5, 1, 0, 0, 0, 0.48, true)
            LAST_EFFECT_SET_RATE(0.7)
            EFFECT_FOLLOW(hash40("roy_eruption_bomb_main"), hash40("top"), 0, -5, 1, 0, 0, 0, 0.48, true)
            LAST_EFFECT_SET_RATE(0.7)
            EFFECT_FOLLOW(hash40("roy_eruption_bomb_start_max"), hash40("top"), 0, -5, 1, 0, 0, 0, 0.48, true)
            LAST_EFFECT_SET_RATE(0.7)
            //EFFECT(hash40("roy_smash_bomb"), hash40("toer"), 0, -5, 1, 0, 0, 0, 0.48, 0, 0, 0, 0, 0, 0, true)
        }
        frame(Frame=19)
        if(is_excute){
            EFFECT_OFF_KIND(hash40("sys_attack_line"), true, true)
            //EFFECT_OFF_KIND(hash40("roy_smash_bomb"), true, true)
        }
        frame(Frame=30)
        if(is_execute){
            //EFFECT_OFF_KIND(hash40("roy_smash_bomb"), true, true)
            EFFECT_OFF_KIND(hash40("roy_eruption_max"), true, true)
            EFFECT_OFF_KIND(hash40("roy_eruption_bomb_main"), true, true)
            EFFECT_OFF_KIND(hash40("roy_eruption_bomb_start_max"), true, true)
        }
    });
}

#[acmd_script( agent = "roy", script = "effect_speciallwhit", category = ACMD_EFFECT )]
unsafe fn effect_roy_speciallwhit(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=1)
        if(is_excute){
            LANDING_EFFECT(hash40("sys_v_smoke_b"), hash40("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false)
        }
        frame(Frame=6)
        if(is_excute){
            //EFFECT_FOLLOW(hash40("roy_erupution_hold"), hash40("bust"), 0, 0, 0, 0, 0, 0, 1, true)
        }
        frame(Frame=26)
        if(is_excute){
            //EFFECT_FOLLOW(hash40("roy_erupution_hold"), hash40("haver"), 0, 0, 0, 3.119, -0.79, -0.543, 1, true)
        }
        frame(Frame=33)
        for(4 Iterations){
            if(is_excute){
                FLASH(1, 1, 0.392, 0.392)
            }
            wait(Frames=1)
            if(is_excute){
                FLASH(1, 0.392, 0, 0.353)
            }
            wait(Frames=1)
            if(is_excute){
                COL_NORMAL()
            }
            wait(Frames=1)
        }
        frame(Frame=51)
        if(is_excute){
            LANDING_EFFECT(hash40("sys_h_smoke_b"), hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
            EFFECT(hash40("roy_eruption_max"), hash40("top"), 0, 0, 15, 0, 0, 0, 2.1, 0, 0, 0, 0, 0, 0, true)
            LAST_EFFECT_SET_RATE(0.7)
            EFFECT(hash40("roy_eruption_bomb_main"), hash40("top"), 0, 0, 15, 0, 0, 0, 1.6, 0, 0, 0, 0, 0, 0, true)
            LAST_EFFECT_SET_RATE(0.7)
            EFFECT(hash40("roy_eruption_bomb_start_max"), hash40("top"), 0, 0, 15, 0, 0, 0, 1.6, 0, 0, 0, 0, 0, 0, true)
            LAST_EFFECT_SET_RATE(0.7)
        }
        frame(Frame=51)
        if(is_excute){
            EFFECT_OFF_KIND(hash40("roy_erupution_hold"), false, true)
        }
    });
}

#[acmd_script( agent = "roy", script = "effect_specialairlwhit", category = ACMD_EFFECT )]
unsafe fn effect_roy_specialairlwhit(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=1)
        if(is_excute){
            LANDING_EFFECT(hash40("sys_v_smoke_b"), hash40("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false)
        }
        frame(Frame=6)
        if(is_excute){
            //EFFECT_FOLLOW(hash40("roy_erupution_hold"), hash40("bust"), 0, 0, 0, 0, 0, 0, 1, true)
        }
        frame(Frame=26)
        if(is_excute){
            //EFFECT_FOLLOW(hash40("roy_erupution_hold"), hash40("haver"), 0, 0, 0, 3.119, -0.79, -0.543, 1, true)
        }
        frame(Frame=33)
        for(4 Iterations){
            if(is_excute){
                FLASH(1, 1, 0.392, 0.392)
            }
            wait(Frames=1)
            if(is_excute){
                FLASH(1, 0.392, 0, 0.353)
            }
            wait(Frames=1)
            if(is_excute){
                COL_NORMAL()
            }
            wait(Frames=1)
        }
        frame(Frame=51)
        if(is_excute){
            LANDING_EFFECT(hash40("sys_h_smoke_b"), hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
            EFFECT(hash40("roy_eruption_max"), hash40("top"), 0, 0, 15, 0, 0, 0, 2.1, 0, 0, 0, 0, 0, 0, true)
            LAST_EFFECT_SET_RATE(0.7)
            EFFECT(hash40("roy_eruption_bomb_main"), hash40("top"), 0, 0, 15, 0, 0, 0, 1.6, 0, 0, 0, 0, 0, 0, true)
            LAST_EFFECT_SET_RATE(0.7)
            EFFECT(hash40("roy_eruption_bomb_start_max"), hash40("top"), 0, 0, 15, 0, 0, 0, 1.6, 0, 0, 0, 0, 0, 0, true)
            LAST_EFFECT_SET_RATE(0.7)
        }
        frame(Frame=51)
        if(is_excute){
            EFFECT_OFF_KIND(hash40("roy_erupution_hold"), false, true)
        }
    });
}


#[acmd_script( agent = "roy", script = "effect_specialnendmax", category = ACMD_EFFECT )]
unsafe fn effect_roy_specialn_end_max(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=1)
        if(is_excute){
            LANDING_EFFECT(hash40("sys_v_smoke_b"), hash40("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false)
        }
        frame(Frame=6)
        if(is_excute){
            //EFFECT_FOLLOW(hash40("roy_erupution_hold"), hash40("bust"), 0, 0, 0, 0, 0, 0, 1, true)
        }
        frame(Frame=26)
        if(is_excute){
            //EFFECT_FOLLOW(hash40("roy_erupution_hold"), hash40("haver"), 0, 0, 0, 3.119, -0.79, -0.543, 1, true)
        }
        frame(Frame=33)
        for(4 Iterations){
            if(is_excute){
                FLASH(1, 1, 0.392, 0.392)
            }
            wait(Frames=1)
            if(is_excute){
                FLASH(1, 0.392, 0, 0.353)
            }
            wait(Frames=1)
            if(is_excute){
                COL_NORMAL()
            }
            wait(Frames=1)
        }
        frame(Frame=51)
        if(is_excute){
            LANDING_EFFECT(hash40("sys_h_smoke_b"), hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
            EFFECT(hash40("roy_eruption_max"), hash40("top"), 0, 0, 15, 0, 0, 0, 2.1, 0, 0, 0, 0, 0, 0, true)
            LAST_EFFECT_SET_RATE(0.7)
            EFFECT(hash40("roy_eruption_bomb_main"), hash40("top"), 0, 0, 15, 0, 0, 0, 1.6, 0, 0, 0, 0, 0, 0, true)
            LAST_EFFECT_SET_RATE(0.7)
            EFFECT(hash40("roy_eruption_bomb_start_max"), hash40("top"), 0, 0, 15, 0, 0, 0, 1.6, 0, 0, 0, 0, 0, 0, true)
            LAST_EFFECT_SET_RATE(0.7)
        }
        frame(Frame=51)
        if(is_excute){
            EFFECT_OFF_KIND(hash40("roy_erupution_hold"), false, true)
        }
    });
}

#[acmd_script( agent = "roy", script = "effect_specialairnendmax", category = ACMD_EFFECT )]
unsafe fn effect_roy_specialairn_end_max(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=1)
        if(is_excute){
            LANDING_EFFECT(hash40("sys_v_smoke_b"), hash40("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false)
        }
        frame(Frame=6)
        if(is_excute){
            //EFFECT_FOLLOW(hash40("roy_erupution_hold"), hash40("bust"), 0, 0, 0, 0, 0, 0, 1, true)
        }
        frame(Frame=26)
        if(is_excute){
            //EFFECT_FOLLOW(hash40("roy_erupution_hold"), hash40("haver"), 0, 0, 0, 3.119, -0.79, -0.543, 1, true)
        }
        frame(Frame=33)
        for(4 Iterations){
            if(is_excute){
                FLASH(1, 1, 0.392, 0.392)
            }
            wait(Frames=1)
            if(is_excute){
                FLASH(1, 0.392, 0, 0.353)
            }
            wait(Frames=1)
            if(is_excute){
                COL_NORMAL()
            }
            wait(Frames=1)
        }
        frame(Frame=51)
        if(is_excute){
            LANDING_EFFECT(hash40("sys_h_smoke_b"), hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
            EFFECT(hash40("roy_eruption_max"), hash40("top"), 0, 0, 15, 0, 0, 0, 2.1, 0, 0, 0, 0, 0, 0, true)
            LAST_EFFECT_SET_RATE(0.7)
            EFFECT(hash40("roy_eruption_bomb_main"), hash40("top"), 0, 0, 15, 0, 0, 0, 1.6, 0, 0, 0, 0, 0, 0, true)
            LAST_EFFECT_SET_RATE(0.7)
            EFFECT(hash40("roy_eruption_bomb_start_max"), hash40("top"), 0, 0, 15, 0, 0, 0, 1.6, 0, 0, 0, 0, 0, 0, true)
            LAST_EFFECT_SET_RATE(0.7)
        }
        frame(Frame=51)
        if(is_excute){
            EFFECT_OFF_KIND(hash40("roy_erupution_hold"), false, true)
        }
    });
}

#[acmd_script( agent = "roy", script = "effect_throwb", category = ACMD_EFFECT )]
unsafe fn effect_roy_throwb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {
        frame(Frame=11)
        if(is_excute){
            EFFECT(hash40("sys_attack_line"), hash40("top"), 0, 8, 2, -140, 0, 0, 1, 0, 0, 0, 0, 0, 0, true)
        }
        frame(Frame=12)
        if(is_excute){
            LANDING_EFFECT(hash40("sys_atk_smoke"), hash40("top"), 2, 0, 0, 0, 180, 0, 0.5, 0, 0, 0, 0, 0, 0, false)
        }
        frame(Frame=20)
        if(is_excute){
            EFFECT(hash40("sys_smash_flash_s"), hash40("throw"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true)
        }
    });
}


#[acmd_script( agent = "roy", script = "effect_throwlw", category = ACMD_EFFECT )]
unsafe fn effect_roy_throwlw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state, {

    });
}

pub fn install() {
    smashline::install_acmd_scripts!(
        effect_roy_attackdash,
        effect_roy_attacks3,
        effect_roy_attackhi3,
        //effect_roy_attacks4,
        effect_roy_attacklw4,
        effect_roy_attackhi4,
        effect_roy_attackairn,
        //effect_roy_attackairb,
        effect_roy_attackairf,
        effect_roy_attackairhi,
        effect_roy_attackairlw,
        effect_roy_throwb,
        effect_roy_throwlw,
        effect_roy_speciallwhit,
        effect_roy_specialairlwhit,
        effect_roy_specialn_end_max,
        effect_roy_specialairn_end_max

    );
}
