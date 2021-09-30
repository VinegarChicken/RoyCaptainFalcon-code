#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#![feature(asm)]

mod roy;
mod roy_effect;
mod roy_sound;


#[smashline::installer]
fn installAll(){
    roy::install();
    roy_effect::install();
    roy_sound::install();
}
#[skyline::main(name = "acmd_test")]
pub fn main() {
    installAll();
}
