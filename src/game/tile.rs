use crate::game::data::{TileType, NUM_TILES, TILES, TILE_COLOR_CLASS};

#[derive(PartialEq, Eq, Clone, Copy, serde::Deserialize, serde::Serialize)]
#[serde(transparent)]
#[repr(transparent)]
pub(crate) struct TileId(pub(in crate::game) usize);

impl TileId {
    pub(crate) fn get(&self) -> &Tile {
        &TILES[self.0]
    }

    #[inline]
    pub(crate) fn is_valid(self) -> bool {
        self.0 < NUM_TILES
    }
}

pub(crate) struct Tile {
    #[allow(clippy::struct_field_names)]
    pub(crate) tile_set: usize,
    pub(crate) color: usize,
    pub(crate) name: &'static str,
    #[allow(clippy::struct_field_names)]
    pub(crate) tile_type: TileType,
}

impl Tile {
    pub(crate) fn color_class(&self) -> &'static str {
        TILE_COLOR_CLASS[self.color]
    }

    pub(crate) fn generate(tile_sets: &[usize], discovered: &[usize]) -> Vec<(TileId, bool)> {
        TILES
            .iter()
            .enumerate()
            .filter_map(|(id, t)| {
                if tile_sets.contains(&t.tile_set) {
                    Some((TileId(id), discovered.contains(&id)))
                } else {
                    None
                }
            })
            .collect()
    }

    pub(crate) fn name(&self, settings_the: bool) -> (&'static str, &'static str) {
        if settings_the && self.name.starts_with("The ") {
            (&self.name[4..], ", The")
        } else {
            (self.name, "")
        }
    }
}

pub(in crate::game) const fn t(
    tile_set: usize,
    color: usize,
    name: &'static str,
    tile_type: TileType,
) -> Tile {
    Tile {
        tile_set,
        color,
        name,
        tile_type,
    }
}
