use crate::game::generated::{Setup, SETUPS};
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

type SetupIter = Map<Enumerate<Iter<'static, Setup>>, fn((usize, &Setup)) -> (SetupId, &Setup)>;

impl Setup {
    pub(crate) fn all() -> SetupIter {
        SETUPS.iter().enumerate().map(|(i, s)| (SetupId(i), s))
    }

    pub(crate) fn tiles(&self) -> Vec<(TileId, bool)> {
        Tile::generate(self.tile_sets, self.discovered)
    }
}
