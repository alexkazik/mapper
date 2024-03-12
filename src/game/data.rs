// Copyright Frank West
//
// He gave permission to use it as long as all uses are clearly marked as unofficial.

use crate::game::tile::{t, Tile};
use yew::virtual_dom::{VNode, VText};
use yew::{AttrValue, Html, ToHtml};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum TileType {
    BldgFarm,
    BldgShrine,
    BldgStronghold,
    BuffAttack,
    BuffHeal,
    BuffMove,
    BuffRange,
    Creature,
    OccupiedCity,
    Quest,
    Shop,
    Stable,
    Trap,
    ResourceFish,
    ResourceLinen,
    ResourceOre,
    ResourceWood,
}

impl ToHtml for TileType {
    fn to_html(&self) -> Html {
        VNode::VText(VText {
            text: AttrValue::Static(match self {
                TileType::BldgFarm => "Building (Farm)",
                TileType::BldgShrine => "Building (Shrine)",
                TileType::BldgStronghold => "Building (Stronghold)",
                TileType::BuffAttack => "Buff (Attack)",
                TileType::BuffHeal => "Buff (Heal)",
                TileType::BuffMove => "Buff (Move)",
                TileType::BuffRange => "Buff (Range)",
                TileType::Creature => "Creature",
                TileType::OccupiedCity => "Occupied City",
                TileType::Quest => "Quest",
                TileType::Shop => "Shop",
                TileType::Stable => "Stable",
                TileType::ResourceFish => "Resource (Fish)",
                TileType::ResourceLinen => "Resource (Linen)",
                TileType::ResourceOre => "Resource (Ore)",
                TileType::ResourceWood => "Resource (Wood)",
                TileType::Trap => "Trap",
            }),
        })
    }
}

pub(super) const NUM_TILES: usize = 47;
pub(super) const TILES: [Tile; NUM_TILES] = [
    //  0
    t(1, 0, "Cogshead Works", TileType::ResourceOre),
    t(1, 0, "Melian Forest", TileType::ResourceWood),
    t(1, 0, "Moon Lake", TileType::ResourceFish),
    t(1, 0, "Oak Wood", TileType::ResourceWood),
    t(1, 0, "Quietsea", TileType::ResourceFish),
    //  5
    t(1, 0, "The Astrographer's Tower", TileType::Creature),
    t(1, 0, "The Cave of T'Lokk", TileType::Creature),
    t(1, 0, "The Granite Halls", TileType::ResourceOre),
    t(1, 0, "The Hyzicki Caravans", TileType::Shop),
    t(1, 0, "The Inn of Lost Hope", TileType::Quest),
    // 10
    t(1, 1, "Crowfeather Market", TileType::Shop),
    t(1, 1, "Menash Greenlands", TileType::ResourceLinen),
    t(1, 1, "Sterncastle Paddocks", TileType::Stable),
    t(1, 1, "The Tower of the North Wind", TileType::Creature),
    t(2, 0, "Olstan's Way", TileType::Quest),
    // 15
    t(2, 0, "Vestan's Curse", TileType::Creature),
    t(3, 0, "Cripplewood Canyons", TileType::Creature),
    t(3, 1, "Darkwood Trees", TileType::Creature),
    t(3, 1, "Howling Heath", TileType::Creature),
    t(4, 0, "Freegrove", TileType::Quest),
    // 20
    t(4, 1, "High Meadows", TileType::ResourceLinen),
    t(5, 1, "The Diamond Halls", TileType::ResourceOre),
    t(5, 1, "Undine's Wash", TileType::ResourceFish),
    t(5, 1, "Wolfswood", TileType::ResourceWood),
    t(6, 0, "Pit", TileType::Trap),
    // 25
    t(6, 0, "Steward Rancor's Fort", TileType::BldgStronghold),
    t(6, 1, "Pit", TileType::Trap),
    t(6, 1, "The Herald's Gate", TileType::BldgStronghold),
    t(6, 1, "The Marred Hand", TileType::Quest),
    t(7, 0, "Boswitch Grounds", TileType::Stable),
    // 30
    t(7, 0, "The House of the Eastern Wind", TileType::BldgShrine),
    t(8, 0, "Ged's Farm", TileType::BldgFarm),
    t(8, 1, "The Old Clocktower Ruins", TileType::Quest),
    t(9, 0, "The Telmarren Grove", TileType::BuffAttack),
    t(10, 1, "Gibbet Cross", TileType::Creature),
    // 35
    t(10, 1, "Ravenswood Town", TileType::Creature),
    t(11, 0, "Giant's Foot Peak", TileType::BuffRange),
    t(11, 0, "Greenacre Stables", TileType::Stable),
    t(11, 0, "The Sappers Way", TileType::BuffMove),
    t(11, 1, "Dragon's Ridge", TileType::BuffRange),
    // 40
    t(12, 1, "The Shrine of the Hidden Sun", TileType::BldgShrine),
    t(13, 0, "The Pools of Aranwyn", TileType::BuffHeal),
    t(13, 1, "Blackthorn Manor", TileType::BldgFarm),
    t(14, 0, "Wayland's Crossroads", TileType::Quest),
    t(15, 0, "Pit", TileType::Trap),
    // 45
    t(15, 1, "Pit", TileType::Trap),
    t(16, 1, "Cloud Keep", TileType::OccupiedCity),
];

pub(super) const TILE_COLOR_CLASS: [&str; 2] = ["tile_color_green", "tile_color_red"];
