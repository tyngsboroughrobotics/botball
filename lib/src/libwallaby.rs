mod lib {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(unused)]
    #![allow(clippy::all)]

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

use lazy_static::lazy_static;
pub use lib::Libwallaby;

lazy_static! {
    static ref LIBWALLABY: Libwallaby =
        unsafe { Libwallaby::new("libwallaby.so").expect("Could not load libwallaby") };
}

pub fn libwallaby() -> &'static Libwallaby {
    &*LIBWALLABY
}
