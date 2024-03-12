use crate::game::{Language, SetupId, TileId, TileType};
use crate::list::Sort;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Page {
    Setup,
    List,
    Custom,
    Story,
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
    pub(crate) custom_tile_sets: Vec<usize>,
    pub(crate) setup_id: Option<SetupId>,
    pub(crate) story_chapter: usize,
    pub(crate) language: Language,
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
            custom_tile_sets: vec![1, 2, 3],
            setup_id: None,
            story_chapter: 0,
            language: Language::English,
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

    pub(crate) fn validate(mut self) -> Self {
        // check if the list is valid
        if !self.list_tiles.iter().all(|(i, _)| i.is_valid()) {
            self.list_tiles = Vec::new();
        }
        // check if a list is possible
        if self.page == Page::List && self.list_tiles.is_empty() {
            self.page = Page::Setup;
        }

        self
    }
}
