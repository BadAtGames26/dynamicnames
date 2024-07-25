#![feature(lazy_cell, ptr_sub_ptr)]
use engage::{gamedata::{unit::Unit, JobData}, mess::Mess};
use unity::prelude::*;

#[unity::hook("App", "Unit", "GetJobName")]
pub fn unit_getjobname(this: &Unit, method_info: OptionalMethod) -> &'static Il2CppString {
    let jobname = call_original!(this, method_info);
    let jid = this.job.jid.get_string().unwrap();
    match jid.as_str() {
        "JID_パラディン" | "JID_ジェネラル" => {
            let weaponmask = this.weapon_mask.value;
            println!("JID: {}, WeaponMask: {}", jid, weaponmask);
            match weaponmask {
                2 => unsafe {
                    string_concat(Mess::get("MID_H_INFO_WLV_Sword"), Mess::get("MID_Space"), jobname, None)
                },
                4 => unsafe {
                    string_concat(Mess::get("MID_H_INFO_WLV_Lance"), Mess::get("MID_Space"), jobname, None)
                },
                8 => unsafe {
                    string_concat(Mess::get("MID_H_INFO_WLV_Axe"), Mess::get("MID_Space"), jobname, None)
                },
                _ => jobname,
            }
        },
        _ => jobname
    }
}

#[unity::from_offset("System", "String", "Concat")]
pub fn string_concat(str0: &Il2CppString, str1: &Il2CppString, str2: &Il2CppString, method_info: OptionalMethod) -> &'static Il2CppString;

#[skyline::main(name = "dynnam")]
pub fn main() {
    std::panic::set_hook(Box::new(|info| {
        let location = info.location().unwrap();

        let msg = match info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => {
                match info.payload().downcast_ref::<String>() {
                    Some(s) => &s[..],
                    None => "Box<Any>",
                }
            },
        };


        let err_msg = format!(
            "Dynamic Names plugin has panicked at '{}' with the following message:\n{}\0",
            location,
            msg
        );

        skyline::error::show_error(
            42069,
            "Dynamic Names plugin has panicked! Please open the details and send a screenshot to the developer, then close the game.\n\0",
            err_msg.as_str(),
        );
    }));
    skyline::install_hook!(unit_getjobname);
}
