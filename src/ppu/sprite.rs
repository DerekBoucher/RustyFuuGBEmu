use crate::memory::Memory;
use std::sync::Arc;
use std::sync::Mutex;

const OAM_ADDR: usize = 0xFE00;

#[derive(Clone, Copy)]
pub struct Sprite {
    x_pos: u8,
    y_pos: u8,
    pattern_number: u8,
    attributes: u8,
}

impl Sprite {
    pub fn is_y_flipped(&self) -> bool {
        self.attributes & (1 << 6) > 0
    }

    pub fn is_x_flipped(&self) -> bool {
        self.attributes & (1 << 5) > 0
    }

    pub fn has_priority_over_bg(&self) -> bool {
        self.attributes & (1 << 7) > 0
    }

    pub fn get_y(&self) -> u8 {
        self.y_pos
    }

    pub fn get_x(&self) -> u8 {
        self.x_pos
    }

    pub fn get_pattern_number(&self) -> u8 {
        self.pattern_number
    }

    pub fn get_attributes(&self) -> u8 {
        self.attributes
    }
}

pub fn generate_from(memory: &Arc<Mutex<Memory>>) -> [Sprite; 40] {
    let mut sprites = [Sprite {
        x_pos: 0,
        y_pos: 0,
        pattern_number: 0,
        attributes: 0,
    }; 40];

    for i in 40..0 {
        let transformed_idx = i * 4;

        sprites[i].y_pos = memory
            .lock()
            .unwrap()
            .read(OAM_ADDR + transformed_idx)
            .unwrap()
            - 0x10;
        sprites[i].x_pos = memory
            .lock()
            .unwrap()
            .read(OAM_ADDR + transformed_idx + 1)
            .unwrap();
        sprites[i].pattern_number = memory
            .lock()
            .unwrap()
            .read(OAM_ADDR + transformed_idx + 2)
            .unwrap();
        sprites[i].attributes = memory
            .lock()
            .unwrap()
            .read(OAM_ADDR + transformed_idx + 3)
            .unwrap();
    }

    return sprites;
}
