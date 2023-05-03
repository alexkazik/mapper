// Copyright Frank West
//
// He gave permission to use it as long as all uses are clearly marked as unofficial.

use crate::game::setup::Setup;
use crate::game::tile::{t, Tile};
use std::fmt::{Display, Formatter};

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

impl Display for TileType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TileType::BldgFarm => f.write_str("Building (Farm)"),
            TileType::BldgShrine => f.write_str("Building (Shrine)"),
            TileType::BldgStronghold => f.write_str("Building (Stronghold)"),
            TileType::BuffAttack => f.write_str("Buff (Attack)"),
            TileType::BuffHeal => f.write_str("Buff (Heal)"),
            TileType::BuffMove => f.write_str("Buff (Move)"),
            TileType::BuffRange => f.write_str("Buff (Range)"),
            TileType::Creature => f.write_str("Creature"),
            TileType::OccupiedCity => f.write_str("Occupied City"),
            TileType::Quest => f.write_str("Quest"),
            TileType::Shop => f.write_str("Shop"),
            TileType::Stable => f.write_str("Stable"),
            TileType::ResourceFish => f.write_str("Resource (Fish)"),
            TileType::ResourceLinen => f.write_str("Resource (Linen)"),
            TileType::ResourceOre => f.write_str("Resource (Ore)"),
            TileType::ResourceWood => f.write_str("Resource (Wood)"),
            TileType::Trap => f.write_str("Trap"),
        }
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

pub(super) const TILE_COLOR: [&str; 2] = ["#819f53", "#b6653f"];

pub(super) const SETUPS: &[Setup] = &[
    Setup {
        name: "Story 1",
        tile_sets: &[1, 2, 3],
        discovered: &[13, 3],
    },
    Setup {
        name: "Story 2",
        tile_sets: &[1, 4, 5],
        discovered: &[13, 1],
    },
    Setup {
        name: "Story 3",
        tile_sets: &[1, 6],
        discovered: &[7],
    },
    Setup {
        name: "Story 4",
        tile_sets: &[1, 3, 7],
        discovered: &[10, 18, 2],
    },
    Setup {
        name: "Story 5",
        tile_sets: &[1, 5, 8],
        discovered: &[5, 21],
    },
    Setup {
        name: "Story 6",
        tile_sets: &[1, 5, 9, 10, 11],
        discovered: &[35, 37, 8, 33, 38, 21],
    },
    Setup {
        name: "Story 7",
        tile_sets: &[1, 10, 11, 12, 13, 16],
        discovered: &[46, 7, 34, 35],
    },
    Setup {
        name: "Scenario: Carry Our Prayers (Normal)",
        tile_sets: &[1, 2, 7, 12],
        discovered: &[2, 40, 4, 30],
    },
    Setup {
        name: "Scenario: Desecrated (Normal)",
        tile_sets: &[1, 2, 3],
        discovered: &[5],
    },
    Setup {
        name: "Scenario: Memory Stone (Normal)",
        tile_sets: &[1, 2, 4, 12],
        discovered: &[7, 11, 8, 0],
    },
    Setup {
        name: "Scenario: Children of Lesser Men (Hard)",
        tile_sets: &[1, 2, 5],
        discovered: &[2, 23, 9, 8],
    },
    Setup {
        name: "Scenario: Lonely Nights (Hard)",
        tile_sets: &[1, 2, 5],
        discovered: &[23, 3, 8, 1],
    },
    Setup {
        name: "Scenario: Sworn (Hard)",
        tile_sets: &[1, 8, 9, 13],
        discovered: &[31, 42, 1, 3],
    },
    Setup {
        name: "Scenario: The Honour of Thieves (Hard)",
        tile_sets: &[1, 6],
        discovered: &[9, 24, 28, 26, 8],
    },
    Setup {
        name: "Scenario: A New Home (Very hard)",
        tile_sets: &[1, 6],
        discovered: &[3, 11, 26, 24, 1],
    },
    Setup {
        name: "Scenario: Hidden in Plain Sigh (Very hard)",
        tile_sets: &[1, 9, 11],
        discovered: &[33, 39, 38, 37],
    },
    Setup {
        name: "Scenario: Rumours (Very hard)",
        tile_sets: &[1, 6],
        discovered: &[27, 26, 24, 9],
    },
    Setup {
        name: "Scenario: The Creations of Vesh (Very hard)",
        tile_sets: &[1, 2, 14, 15],
        discovered: &[14, 3, 7, 11, 2],
    },
    Setup {
        name: "Scenario: The Scorched (Very hard)",
        tile_sets: &[1, 3, 10, 11, 14],
        discovered: &[2, 39, 16, 43, 9, 6, 3],
    },
];
