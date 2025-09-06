use unity::prelude::*;

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