#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(deref_nullptr)]

use std::env;

// include!("bindings.rs");

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
