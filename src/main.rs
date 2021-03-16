#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate enum_primitive;
extern crate num;

extern crate core_affinity;
extern crate num_format;

mod components;
mod core;
mod engine;
mod input;
mod moves;
mod perft;
mod utils;

fn main() {
    // Pin current thread to a core
    let core_ids = core_affinity::get_core_ids().unwrap();
    core_affinity::set_for_current(core_ids[0]);
}
