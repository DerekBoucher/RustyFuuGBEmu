use crate::memory::Memory;
use std::sync::Arc;
use std::sync::Mutex;

const OAM_ADDR: usize = 0xFE00;

#[derive(Clone, Copy)]
pub struct Sprite {
    x_pos: u8,
    y_pos: u8,
    pub pattern_number: u8,
    attributes: u8,
}

impl Sprite {
    pub fn is_y_flipped(&self) -> bool {
        self.attributes & (1 << 6) > 0
    }

    pub fn is_x_flipped(&self) -> bool {
        self.attributes & (1 << 5) > 0
    }

    pub fn bg_has_priority_over_this(&self) -> bool {
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

pub fn process_from_memory(memory: &Arc<Mutex<Memory>>) -> [Sprite; 40] {
    let mut sprites = [Sprite {
        x_pos: 0,
        y_pos: 0,
        pattern_number: 0,
        attributes: 0,
    }; 40];

    for i in (0..40).rev() {
        let transformed_idx = i as usize * 4;

        sprites[i as usize].y_pos = memory
            .lock()
            .unwrap()
            .dma_read(OAM_ADDR + transformed_idx)
            .unwrap()
            .wrapping_sub(16);
        sprites[i as usize].x_pos = memory
            .lock()
            .unwrap()
            .dma_read(OAM_ADDR + transformed_idx + 1)
            .unwrap()
            .wrapping_sub(8);
        sprites[i as usize].pattern_number = memory
            .lock()
            .unwrap()
            .dma_read(OAM_ADDR + transformed_idx + 2)
            .unwrap();
        sprites[i as usize].attributes = memory
            .lock()
            .unwrap()
            .dma_read(OAM_ADDR + transformed_idx + 3)
            .unwrap();
    }

    return sprites;
}
