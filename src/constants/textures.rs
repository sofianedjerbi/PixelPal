use crate::components::textures::*;
use phf::phf_map;


pub const WATER_MAP: TextureIDProbabilityPHF = TextureIDProbabilityPHF(
    phf_map! {
        0u32 => 0,
    }
);

pub const GRASS_MAP: TextureIDProbabilityPHF = TextureIDProbabilityPHF(
    phf_map! {
        0u32 => 55,  // At least 15 units
        15u32 => 56,  // At least 15 units
        30u32 => 57,  // At least 15 units
        45u32 => 58, // Adjusted proportionally
        50u32 => 59, // Adjusted proportionally
        55u32 => 60, // Adjusted proportionally
        60u32 => 66, // At least 15 units
        75u32 => 67, // At least 15 units
        90u32 => 68, // At least 15 units
        95u32 => 69, // Adjusted proportionally
        97u32 => 70, // Adjusted proportionally
        99u32 => 71, // Adjusted proportionally
        100u32 => 12, // 90%
    }
);

pub const DARKER_GRASS_MAP: TextureIDProbabilityPHF = TextureIDProbabilityPHF(
    phf_map! {
        0u32 => 55,  // At least 15 units
        15u32 => 56,  // At least 15 units
        30u32 => 57,  // At least 15 units
        45u32 => 58, // Adjusted proportionally
        50u32 => 59, // Adjusted proportionally
        55u32 => 60, // Adjusted proportionally
        60u32 => 66, // At least 15 units
        75u32 => 67, // At least 15 units
        90u32 => 68, // At least 15 units
        95u32 => 69, // Adjusted proportionally
        97u32 => 70, // Adjusted proportionally
        99u32 => 71, // Adjusted proportionally
        100u32 => 12, // 90%
    }
);

pub const SOIL_MAP: TextureIDProbabilityPHF = TextureIDProbabilityPHF(
    phf_map! {
        0u32 => 55,  // 14.89 (rounded to 15)
        15u32 => 56, // 14.89 (rounded to 15)
        30u32 => 57, // 14.89 (rounded to 15)
        45u32 => 58, // 2.96 (rounded to 3)
        48u32 => 59, // 4.46 (rounded to 4)
        52u32 => 66, // 14.89 (rounded to 15)
        67u32 => 67, // 14.89 (rounded to 15)
        82u32 => 68, // 14.89 (rounded to 15)
        97u32 => 69, // 2.96 (rounded to 3)
        100u32 => 70, // 4.46 (rounded to 4)
        104u32 => 12, // 90% of 1000 (keys 100 to 1000)
    }
);

pub const TEXTURE_RELIEF_IDS_MAP: TextureReliefIDsPHF = TextureReliefIDsPHF(
    phf_map! {
        0u32 => &GRASS_MAP,
        1u32 => &SOIL_MAP,
        2u32 => &WATER_MAP,
        3u32 => &SOIL_MAP,
        4u32 => &GRASS_MAP,
        5u32 => &GRASS_MAP,
        6u32 => &GRASS_MAP,
        7u32 => &DARKER_GRASS_MAP,
        8u32 => &DARKER_GRASS_MAP,
        9u32 => &DARKER_GRASS_MAP,
    }
);

const WATER: u32 = 0;
const SOIL: u32 = 1;
const GRASS_HILL: u32 = 4;
const DARKER_GRASS_HILL: u32 = 5;

// Top-of-water tilesets:
//const GRASS: u32 = 2;
//const DARKER_GRASS: u32 = 3;

const TILESET_SIZE: u32 = 77;

pub const WATER_LEVEL: u32 = 2;

pub const TEXTURE_ID_OFFSET_MAP: TextureIDOffsetPHF = TextureIDOffsetPHF(
    phf_map! {
        0u32 => TILESET_SIZE*GRASS_HILL,
        1u32 => TILESET_SIZE*SOIL,
        2u32 => TILESET_SIZE*WATER,
        3u32 => TILESET_SIZE*SOIL,
        4u32 => TILESET_SIZE*GRASS_HILL,
        5u32 => TILESET_SIZE*GRASS_HILL,
        6u32 => TILESET_SIZE*GRASS_HILL,
        7u32 => TILESET_SIZE*DARKER_GRASS_HILL,
        8u32 => TILESET_SIZE*DARKER_GRASS_HILL,
        9u32 => TILESET_SIZE*DARKER_GRASS_HILL,
    }
);

pub const TEXTURE_CORNER_IDS_MAP: TextureCornerIDsPHF = TextureCornerIDsPHF(
    // 000
    // 0_0 => 000_0_0_000
    // 000
    phf_map! {
        0b111_1_0_100u32 => 0,
        0b110_1_0_100u32 => 0,
        0b111_1_0_000u32 => 0,
        0b110_1_0_000u32 => 0,
        0b011_1_0_100u32 => 0,
        0b010_1_0_100u32 => 0,
        0b011_1_0_000u32 => 0,
        0b010_1_0_000u32 => 0,

        0b111_0_0_000u32 => 1,
        0b110_0_0_000u32 => 1,
        0b011_0_0_000u32 => 1,
        0b010_0_0_000u32 => 1,

        0b111_0_1_001u32 => 2,
        0b011_0_1_001u32 => 2,
        0b111_0_1_000u32 => 2,
        0b011_0_1_000u32 => 2,
        0b110_0_1_001u32 => 2,
        0b010_0_1_001u32 => 2,
        0b110_0_1_000u32 => 2,
        0b010_0_1_000u32 => 2,

        0b111_1_1_101u32 => 3,
        0b110_1_1_101u32 => 3,
        0b011_1_1_101u32 => 3,
        0b010_1_1_101u32 => 3,
        0b111_1_1_100u32 => 3,
        0b110_1_1_100u32 => 3,
        0b011_1_1_100u32 => 3,
        0b010_1_1_100u32 => 3,
        0b111_1_1_001u32 => 3,
        0b110_1_1_001u32 => 3,
        0b011_1_1_001u32 => 3,
        0b010_1_1_001u32 => 3,
        0b111_1_1_000u32 => 3,
        0b110_1_1_000u32 => 3,
        0b011_1_1_000u32 => 3,
        0b010_1_1_000u32 => 3,

        0b111_1_0_101u32 => 4,
        0b111_1_0_001u32 => 4,
        0b011_1_0_101u32 => 4,
        0b011_1_0_001u32 => 4,

        0b111_0_0_001u32 => 5,
        0b111_0_0_100u32 => 6,

        0b111_0_1_101u32 => 7,
        0b111_0_1_100u32 => 7,
        0b110_0_1_101u32 => 7,
        0b110_0_1_100u32 => 7,

        0b111_0_0_101u32 => 8,
        0b001_0_0_100u32 => 9,

        0b100_1_0_100u32 => 11,
        0b100_1_0_000u32 => 11,
        0b000_1_0_100u32 => 11,
        0b000_1_0_000u32 => 11,

        0b000_0_0_000u32 => 12,

        0b001_0_1_001u32 => 13,
        0b000_0_1_001u32 => 13,
        0b001_0_1_000u32 => 13,
        0b000_0_1_000u32 => 13,

        0b101_1_1_101u32 => 14,
        0b100_1_0_101u32 => 15,
        0b000_0_0_001u32 => 16,
        0b000_0_0_100u32 => 17,
        0b001_0_1_101u32 => 18,
        0b000_0_0_101u32 => 19,
        0b100_0_0_001u32 => 20,

        0b100_1_0_111u32 => 22,
        0b000_1_0_111u32 => 22,
        0b100_1_0_110u32 => 22,
        0b000_1_0_110u32 => 22,
        0b100_1_0_011u32 => 22,
        0b000_1_0_011u32 => 22,
        0b100_1_0_010u32 => 22,
        0b000_1_0_010u32 => 22,

        0b000_0_0_111u32 => 23,
        0b000_0_0_110u32 => 23,
        0b000_0_0_011u32 => 23,
        0b000_0_0_010u32 => 23,

        0b001_0_1_111u32 => 24,
        0b000_0_1_111u32 => 24,
        0b001_0_1_011u32 => 24,
        0b000_0_1_011u32 => 24,
        0b001_0_1_110u32 => 24,
        0b000_0_1_110u32 => 24,
        0b001_0_1_010u32 => 24,
        0b000_0_1_010u32 => 24,
        
        0b101_1_1_111u32 => 25,
        0b100_1_1_111u32 => 25,
        0b001_1_1_111u32 => 25,
        0b000_1_1_111u32 => 25,
        0b101_1_1_110u32 => 25,
        0b100_1_1_110u32 => 25,
        0b001_1_1_110u32 => 25,
        0b000_1_1_110u32 => 25,
        0b101_1_1_011u32 => 25,
        0b100_1_1_011u32 => 25,
        0b001_1_1_011u32 => 25,
        0b000_1_1_011u32 => 25,
        0b101_1_1_010u32 => 25,
        0b100_1_1_010u32 => 25,
        0b001_1_1_010u32 => 25,
        0b000_1_1_010u32 => 25,

        0b101_1_0_100u32 => 26,
        0b001_0_0_000u32 => 27,
        0b100_0_0_000u32 => 28,
        0b101_0_1_001u32 => 29,
        0b101_0_0_000u32 => 30,
        0b101_0_0_100u32 => 31,
        0b101_0_0_001u32 => 32,

        0b111_1_0_111u32 => 33,
        0b011_1_0_111u32 => 33,
        0b111_1_0_011u32 => 33,
        0b011_1_0_011u32 => 33,
        0b111_1_0_110u32 => 33,
        0b011_1_0_110u32 => 33,
        0b111_1_0_010u32 => 33,
        0b011_1_0_010u32 => 33,
        0b110_1_0_111u32 => 33,
        0b010_1_0_111u32 => 33,
        0b110_1_0_011u32 => 33,
        0b010_1_0_011u32 => 33,
        0b110_1_0_110u32 => 33,
        0b010_1_0_110u32 => 33,
        0b110_1_0_010u32 => 33,
        0b010_1_0_010u32 => 33,

        0b111_0_0_111u32 => 34,

        0b111_0_1_111u32 => 35,
        0b011_0_1_111u32 => 35,
        0b111_0_1_011u32 => 35,
        0b011_0_1_011u32 => 35,
        0b110_0_1_111u32 => 35,
        0b010_0_1_111u32 => 35,
        0b110_0_1_011u32 => 35,
        0b010_0_1_011u32 => 35,
        0b111_0_1_110u32 => 35,
        0b011_0_1_110u32 => 35,
        0b111_0_1_010u32 => 35,
        0b011_0_1_010u32 => 35,
        0b110_0_1_110u32 => 35,
        0b010_0_1_110u32 => 35,
        0b110_0_1_010u32 => 35,
        0b010_0_1_010u32 => 35,

        0b111_1_1_111u32 => 36,
        0b110_1_1_111u32 => 36,
        0b011_1_1_111u32 => 36,
        0b010_1_1_111u32 => 36,
        0b111_1_1_110u32 => 36,
        0b110_1_1_110u32 => 36,
        0b011_1_1_110u32 => 36,
        0b010_1_1_110u32 => 36,
        0b111_1_1_011u32 => 36,
        0b110_1_1_011u32 => 36,
        0b011_1_1_011u32 => 36,
        0b010_1_1_011u32 => 36,
        0b111_1_1_010u32 => 36,
        0b110_1_1_010u32 => 36,
        0b011_1_1_010u32 => 36,
        0b010_1_1_010u32 => 36,
        
        0b101_1_0_111u32 => 37,
        0b001_1_0_111u32 => 37,
        0b101_1_0_011u32 => 37,
        0b001_1_0_011u32 => 37,

        0b001_0_0_111u32 => 38,
        0b100_0_0_111u32 => 39,

        0b101_0_1_111u32 => 40,
        0b100_0_1_111u32 => 40,
        0b101_0_1_110u32 => 40,
        0b100_0_1_110u32 => 40,

        0b101_0_0_111u32 => 41,
        0b100_0_0_101u32 => 42,
        0b001_0_0_101u32 => 43,
        0b101_1_0_101u32 => 48,
        0b001_0_0_001u32 => 49,
        0b100_0_0_100u32 => 50,
        0b101_0_1_101u32 => 51,
        0b101_0_0_101u32 => 52,
    }
);

