pub const MODE_MASK: u8 = (1 << 1) | (1 << 0);
pub const MAX_SCANLINE_COUNT: i32 = 456;

const STAT_LY_LYC_MASK: u8 = 1 << 2;
const MODE_0_INTERRUPT_MASK: u8 = 1 << 3;
const MODE_1_INTERRUPT_MASK: u8 = 1 << 4;
const MODE_2_INTERRUPT_MASK: u8 = 1 << 5;
const LY_LYC_INTERRUPT_MASK: u8 = 1 << 6;
const V_BLANK_BOUNDARY: u8 = 144;
const TRANSFER_PIXEL_BOUNDARY: i32 = MAX_SCANLINE_COUNT - 80;
const OAM_SEARCH_BOUNDARY: i32 = TRANSFER_PIXEL_BOUNDARY - 172;

enum StatMode {
    HBlank,
    VBlank,
    OAMSearch,
    PixelTransfer,
}

impl From<u8> for StatMode {
    fn from(value: u8) -> Self {
        match value {
            0x00 => StatMode::HBlank,
            0x01 => StatMode::VBlank,
            0x02 => StatMode::OAMSearch,
            0x03 => StatMode::PixelTransfer,
            _ => panic!("invalid stat mode: {}", value),
        }
    }
}

pub struct StatUpdater {
    current_stat: u8,
    new_stat: u8,
    requires_interrupt: bool,
    state_change_already_occured: bool,
}

impl StatUpdater {
    pub fn new(current_stat: u8) -> Self {
        Self {
            current_stat: current_stat,
            new_stat: current_stat & !MODE_MASK,
            requires_interrupt: false,
            state_change_already_occured: false,
        }
    }

    pub fn process_vblank(mut self, current_scanline: u8) -> Self {
        if current_scanline >= V_BLANK_BOUNDARY {
            self.state_change_already_occured = true;
            self.new_stat |= StatMode::VBlank as u8;

            if self.current_stat & MODE_1_INTERRUPT_MASK > 0 {
                self.requires_interrupt = true;
            }
        }

        return self;
    }

    pub fn process_oam_search(mut self, scanline_counter: i32) -> Self {
        if self.state_change_already_occured {
            return self;
        }

        if scanline_counter >= OAM_SEARCH_BOUNDARY {
            self.state_change_already_occured = true;
            self.new_stat |= StatMode::OAMSearch as u8;

            if self.current_stat & MODE_2_INTERRUPT_MASK > 0 {
                self.requires_interrupt = true;
            }
        }

        return self;
    }

    pub fn process_pixel_transfer(mut self, scanline_counter: i32) -> Self {
        if self.state_change_already_occured {
            return self;
        }

        if scanline_counter >= TRANSFER_PIXEL_BOUNDARY {
            self.state_change_already_occured = true;
            self.new_stat |= StatMode::PixelTransfer as u8;
        }

        return self;
    }

    pub fn process_hblank(mut self) -> Self {
        if self.state_change_already_occured {
            return self;
        }

        self.state_change_already_occured = true;
        self.new_stat |= StatMode::HBlank as u8;

        if self.current_stat & MODE_0_INTERRUPT_MASK > 0 {
            self.requires_interrupt = true;
        }

        return self;
    }

    pub fn process_ly_lyc(mut self, ly: u8, lyc: u8) -> Self {
        if ly == lyc {
            self.new_stat |= STAT_LY_LYC_MASK;
        } else {
            self.new_stat &= !STAT_LY_LYC_MASK;
        }

        if self.current_stat & LY_LYC_INTERRUPT_MASK > 0 {
            self.requires_interrupt = true;
        }

        return self;
    }

    pub fn build(&self) -> (u8, bool) {
        (self.new_stat, self.requires_interrupt)
    }
}
