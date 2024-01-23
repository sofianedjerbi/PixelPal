use phf::phf_map;

use crate::components::ui::DialogueSizePHF;

pub const DIALOGUE_MAX_CHARACTERS: usize = 20;
pub const DIALOGUE_SIZE_PHF: DialogueSizePHF = DialogueSizePHF(phf_map!(
    0u8 => 0,
    5u8 => 2,
    10u8 => 3,
    15u8 => 4,
));
