use crate::game::{TileId, TileType};
use crate::list::Sort;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Page {
    Setup,
    List,
}

#[derive(Serialize, Deserialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub(crate) struct State {
    pub(crate) page: Page,
    pub(crate) list_tiles: Vec<(TileId, bool)>,
    pub(crate) list_show_discovered: bool,
    pub(crate) list_sort: Sort,
    pub(crate) list_the: bool,
    pub(crate) list_tile_set: bool,
}

impl Default for State {
    fn default() -> Self {
        State {
            page: Page::Setup,
            list_tiles: Vec::new(),
            list_show_discovered: false,
            list_sort: Sort::Name,
            list_the: true,
            list_tile_set: false,
        }
    }
}

impl State {
    pub(crate) fn sort(&mut self) {
        self.list_tiles.sort_by_cached_key(|(id, _)| {
            let tile = id.get();
            (
                tile.color,
                match self.list_sort {
                    Sort::Name => TileType::Creature,
                    Sort::Type => tile.tile_type,
                },
                tile.name(self.list_the).0,
            )
        });
    }
}
