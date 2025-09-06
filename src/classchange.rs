use engage::gamedata::{unit::Unit, JobData, WeaponMask};
use unity::prelude::*;
use crate::utils::trysettext;
use crate::jobname::get_job_name;

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