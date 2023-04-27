use crate::game::data::SETUPS;
use crate::game::tile::{Tile, TileId};
use std::iter::{Enumerate, Map};
use std::slice::Iter;

#[derive(Clone, Copy)]
#[repr(transparent)]
pub(crate) struct SetupId(usize);

impl SetupId {
    pub(crate) fn get(&self) -> &Setup {
        &SETUPS[self.0]
    }
}

pub(crate) struct Setup {
    pub(crate) name: &'static str,
    pub(in crate::game) tile_sets: &'static [usize],
    pub(in crate::game) discovered: &'static [usize],
}

type SetupIter = Map<Enumerate<Iter<'static, Setup>>, fn((usize, &Setup)) -> (SetupId, &Setup)>;

impl Setup {
    pub(crate) fn all() -> SetupIter {
        SETUPS.iter().enumerate().map(|(i, s)| (SetupId(i), s))
    }

    pub(crate) fn tiles(&self) -> Vec<(TileId, bool)> {
        Tile::generate(self.tile_sets, self.discovered)
    }
}
