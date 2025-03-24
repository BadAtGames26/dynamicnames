#![feature(ptr_sub_ptr)]
use engage::{gamedata::{unit::{Gender, Unit}, JobData, WeaponMask}, mess::Mess};
use unity::prelude::*;

#[unity::class("App", "ClassChange_ChangeJobData")]
pub struct ChangeJobData {
    pub job: &'static mut JobData,
    pub jobweaponmask: &'static mut WeaponMask,

}
#[unity::class("App", "ClassChangeJobMenuItemContent")]
pub struct ClassChangeJobMenuItemContent {
    _pad: [u8; 0x40],
    pub title: &'static u64,
}

#[unity::hook("App", "ClassChangeJobMenuItemContent", "SetJobText")]
pub fn setjobtext(this: &ClassChangeJobMenuItemContent, unit: &Unit, changejob: &ChangeJobData, method_info: OptionalMethod) {
    call_original!(this, unit, changejob, method_info);

    // We have unit and job here, but changing the classs here would cause the unit to change class so we just take the data we need.
    let job = &changejob.job;
    let weapon_mask = changejob.jobweaponmask.value;
    let gender = &unit.get_gender();

    let name = get_job_name(job, weapon_mask, gender);

    // Changing the text on each class in the reclass list to properly display
    trysettext(this.title, name);
}

#[unity::hook("App", "Unit", "GetJobName")]
pub fn unit_getjobname(this: &Unit, _method_info: OptionalMethod) -> &'static Il2CppString {
    get_job_name(this.job, this.weapon_mask.value, &this.get_gender())
}

pub fn get_job_name(job: &JobData, weapon_mask: i32, gender: &Gender) -> &'static Il2CppString {
    let job_name = job.name.to_string();

    // New MJIDs need a _G at the end to be able to have a gender name
    let new_name = if job_name.ends_with("_G") {
        get_gender_name(job, gender)
    } else if job.weapons.iter().any(|&x| x == 2) { // Check for any weapon having a value of 2 (ex. Paladins and Generals only have 2 for sword, lance and axes)
        get_weapon_name(job, weapon_mask)
    } else {
        // If theres nothing we want to change just return the original name
        return Mess::get(job_name);
    };

    // Checking to see if the new entry actually exist so we don't return an empty string
    if isexist(&new_name) {
        Mess::get(new_name)
    } else {
        //println!("{} does not exist.", new_name);
        Mess::get(job_name)
    }
}

pub fn get_gender_name(job: &JobData, gender: &Gender) -> String {
    let job_name = job.name.to_string();

    match gender {
        Gender::Male => job_name + "M",
        Gender::Female => job_name + "F",
        _ => job_name
    }
}

pub fn get_weapon_name(job: &JobData, weapon_mask: i32) -> String {
    let job_name = job.name.to_string();
    let job_weapons = &job.weapons;  

    // Checking if the mask contains a weapon type and that weapon isn't one thats always allowed (ex. Hero always has swords but not lances or axes so we don't need the sword name.)
    match weapon_mask {
        weapon_mask if weapon_mask&2 == 2 && job_weapons[1] != 1 => {
            job_name + "_Sword"
        },
        weapon_mask if weapon_mask&4 == 4 && job_weapons[2] != 1 => {
            job_name + "_Lance"
        },
        weapon_mask if weapon_mask&8 == 8 && job_weapons[3] != 1 => {
            job_name + "_Axe"
        },
        weapon_mask if weapon_mask&16 == 16 && job_weapons[4] != 1 => {
            job_name + "_Bow"
        },
        weapon_mask if weapon_mask&32 == 32 && job_weapons[5] != 1 => {
            job_name + "_Dagger"
        },
        weapon_mask if weapon_mask&64 == 64 && job_weapons[6] != 1 => {
            job_name + "_Magic"
        },
        weapon_mask if weapon_mask&128 == 128 && job_weapons[7] != 1 => {
            job_name + "_Rod"
        },
        weapon_mask if weapon_mask&256 == 256 && job_weapons[8] != 1 => {
            job_name + "_Fist"
        },
        weapon_mask if weapon_mask&512 == 512 && job_weapons[9] != 1 => {
            job_name + "_Special"
        },
        _ => {
            job_name
        }
    }
}

pub fn isexist<'a>(label: impl Into<&'a Il2CppString>) -> bool {
    unsafe { mess_isexist(label.into(), None) }
}

pub fn trysettext(tmp: &u64, string: &Il2CppString) {
    unsafe { infoutil_trysettext(tmp, string, None) }
}

#[unity::from_offset("App", "Mess", "IsExist")]
pub fn mess_isexist(label: &Il2CppString, method_info: OptionalMethod) -> bool;

#[unity::from_offset("App", "InfoUtil", "TrySetText")]
pub fn infoutil_trysettext(tmp: &u64, str: &Il2CppString, method_info: OptionalMethod);

#[skyline::main(name = "dynname")]
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
    skyline::install_hooks!(unit_getjobname, setjobtext);
}
