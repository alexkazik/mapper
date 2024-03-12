use enum_tools::EnumTools;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumTools, Serialize, Deserialize)]
#[enum_tools(as_str)]
#[repr(u8)]
pub(crate) enum Language {
    English,
    German,
}

pub(crate) struct Setup {
    pub(crate) name: &'static str,
    pub(crate) tile_sets: &'static [usize],
    pub(crate) discovered: &'static [usize],
    pub(crate) stories: &'static [Story],
}

pub(crate) struct Story {
    pub language: Language,
    pub name: &'static str,
    pub chapters: &'static [Chapter],
}

pub(crate) struct Chapter {
    pub name: &'static str,
    pub paragraphs: &'static [&'static str],
}

include!(concat!(env!("OUT_DIR"), "/generated.rs"));
