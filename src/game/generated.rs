use std::env;

pub(crate) struct Setup {
    pub(crate) name: &'static str,
    pub(crate) tile_sets: &'static [usize],
    pub(crate) discovered: &'static [usize],
}

include!(concat!(env!("OUT_DIR"), "/generated.rs"));
