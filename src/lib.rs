#![feature(ptr_sub_ptr)]
use crate::jobname::unit_getjobname_skyline_internal_install_hook;
use crate::classchange::setjobtext_skyline_internal_install_hook;

mod classchange;
mod jobname;
mod utils;

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
