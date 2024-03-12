// Copyright Frank West
//
// He gave permission to use it as long as all uses are clearly marked as unofficial.

pub(crate) struct RawSetup {
    pub(crate) name: &'static str,
    pub(crate) tile_sets: &'static [usize],
    pub(crate) discovered: &'static [usize],
}

pub(crate) const RAW_SETUPS: &[RawSetup] = &[
    RawSetup {
        name: "Story 1",
        tile_sets: &[1, 2, 3],
        discovered: &[13, 3],
    },
    RawSetup {
        name: "Story 2",
        tile_sets: &[1, 4, 5],
        discovered: &[13, 1],
    },
    RawSetup {
        name: "Story 3",
        tile_sets: &[1, 6],
        discovered: &[7],
    },
    RawSetup {
        name: "Story 4",
        tile_sets: &[1, 3, 7],
        discovered: &[10, 18, 2],
    },
    RawSetup {
        name: "Story 5",
        tile_sets: &[1, 5, 8],
        discovered: &[5, 21],
    },
    RawSetup {
        name: "Story 6",
        tile_sets: &[1, 5, 9, 10, 11],
        discovered: &[35, 37, 8, 33, 38, 21],
    },
    RawSetup {
        name: "Story 7",
        tile_sets: &[1, 10, 11, 12, 13, 16],
        discovered: &[46, 7, 34, 35],
    },
    RawSetup {
        name: "Scenario: Carry Our Prayers (Normal)",
        tile_sets: &[1, 2, 7, 12],
        discovered: &[2, 40, 4, 30],
    },
    RawSetup {
        name: "Scenario: Desecrated (Normal)",
        tile_sets: &[1, 2, 3],
        discovered: &[5],
    },
    RawSetup {
        name: "Scenario: Memory Stone (Normal)",
        tile_sets: &[1, 2, 4, 12],
        discovered: &[7, 11, 8, 0],
    },
    RawSetup {
        name: "Scenario: Children of Lesser Men (Hard)",
        tile_sets: &[1, 2, 5],
        discovered: &[2, 23, 9, 8],
    },
    RawSetup {
        name: "Scenario: Lonely Nights (Hard)",
        tile_sets: &[1, 2, 5],
        discovered: &[23, 3, 8, 1],
    },
    RawSetup {
        name: "Scenario: Sworn (Hard)",
        tile_sets: &[1, 8, 9, 13],
        discovered: &[31, 42, 1, 3],
    },
    RawSetup {
        name: "Scenario: The Honour of Thieves (Hard)",
        tile_sets: &[1, 6],
        discovered: &[9, 24, 28, 26, 8],
    },
    RawSetup {
        name: "Scenario: A New Home (Very hard)",
        tile_sets: &[1, 6],
        discovered: &[3, 11, 26, 24, 1],
    },
    RawSetup {
        name: "Scenario: Hidden in Plain Sigh (Very hard)",
        tile_sets: &[1, 9, 11],
        discovered: &[33, 39, 38, 37],
    },
    RawSetup {
        name: "Scenario: Rumours (Very hard)",
        tile_sets: &[1, 6],
        discovered: &[27, 26, 24, 9],
    },
    RawSetup {
        name: "Scenario: The Creations of Vesh (Very hard)",
        tile_sets: &[1, 2, 14, 15],
        discovered: &[14, 3, 7, 11, 2],
    },
    RawSetup {
        name: "Scenario: The Scorched (Very hard)",
        tile_sets: &[1, 3, 10, 11, 14],
        discovered: &[2, 39, 16, 43, 9, 6, 3],
    },
];
