pub const JOYPAD_ADDR: usize = 0xFF00;
pub const SERIAL_TRANSFER_DATA_ADDR: usize = 0xFF01;
pub const SERIAL_TRANSFER_CONTROL_ADDR: usize = 0xFF02;
pub const TIMER_DIV_ADDR: usize = 0xFF04;
pub const TIMER_COUNTER_ADDR: usize = 0xFF05;
pub const TIMER_MOD_ADDR: usize = 0xFF06;
pub const TIMER_CTRL_ADDR: usize = 0xFF07;
pub const INTERRUPT_FLAG_ADDR: usize = 0xFF0F;
pub const AUDIO_CH1_SWEEP_ADDR: usize = 0xFF10;
pub const AUDIO_CH1_LENGTH_ADDR: usize = 0xFF11;
pub const AUDIO_CH1_VOLUME_ADDR: usize = 0xFF12;
pub const AUDIO_CH1_WAV_LO_ADDR: usize = 0xFF13;
pub const AUDIO_CH1_WAV_HI_ADDR: usize = 0xFF14;
pub const AUDIO_CH2_LENGTH_ADDR: usize = 0xFF16;
pub const AUDIO_CH2_VOLUME_ADDR: usize = 0xFF17;
pub const AUDIO_CH2_WAV_LO_ADDR: usize = 0xFF18;
pub const AUDIO_CH2_WAV_HI_ADDR: usize = 0xFF19;
pub const AUDIO_CH3_DAC_ENABLE_ADDR: usize = 0xFF1A;
pub const AUDIO_CH3_LENGTH_ADDR: usize = 0xFF1B;
pub const AUDIO_CH3_OUTPUT_LVL_ADDR: usize = 0xFF1C;
pub const AUDIO_CH3_WAV_LO_ADDR: usize = 0xFF1D;
pub const AUDIO_CH3_WAV_HI_ADDR: usize = 0xFF1E;
pub const AUDIO_CH4_LENGTH_ADDR: usize = 0xFF20;
pub const AUDIO_CH4_VOLUME_ADDR: usize = 0xFF21;
pub const AUDIO_CH4_FREQ_ADDR: usize = 0xFF22;
pub const AUDIO_CH4_CTRL_ADDR: usize = 0xFF23;
pub const AUDIO_WAV_PATTERN_RAM_START_ADDR: usize = 0xFF30;
pub const AUDIO_GLOBAL_CTRL_ADDR: usize = 0xFF26;
pub const AUDIO_GLOBAL_PANNING_ADDR: usize = 0xFF25;
pub const AUDIO_GLOBAL_VOLUME_ADDR: usize = 0xFF24;
pub const LCD_CONTROL_ADDR: usize = 0xFF40;
pub const LCD_STAT_ADDR: usize = 0xFF41;
pub const LCD_SCY_ADDR: usize = 0xFF42;
pub const LCD_SCX_ADDR: usize = 0xFF43;
pub const LCD_LY_ADDR: usize = 0xFF44;
pub const LCD_LYC_ADDR: usize = 0xFF45;
pub const LCD_WINY_ADDR: usize = 0xFF4A;
pub const LCD_WINX_ADDR: usize = 0xFF4B;
pub const LCD_PALETTE_ADDR: usize = 0xFF47;
pub const BOOT_ROM_DISABLE_ADDR: usize = 0xFF50;

/// IO Registers that have side effects when read/written to
#[derive(Debug, Copy, Clone)]
enum IORegister {
    TimerDiv(u8),
    Default(u8),
}

#[derive(Debug, Copy, Clone)]
pub struct IORegisters {
    registers: [IORegister; 0x80],
}

impl IORegisters {
    pub fn new() -> Self {
        let mut registers = [IORegister::Default(0x00); 0x80];
        for (idx, register) in registers.iter_mut().enumerate() {
            *register = IORegister::new(idx + 0xFF00);
        }

        Self { registers }
    }

    pub fn post_boot_rom_state() -> Self {
        let offset: usize = 0xFF00;
        let mut r = IORegisters::new();

        r.registers[BOOT_ROM_DISABLE_ADDR - offset].write(0x01);
        r.registers[JOYPAD_ADDR - offset] = IORegister::Default(0xCF);
        r.registers[SERIAL_TRANSFER_DATA_ADDR - offset] = IORegister::Default(0x00);
        r.registers[SERIAL_TRANSFER_CONTROL_ADDR - offset] = IORegister::Default(0x7E);
        r.registers[TIMER_DIV_ADDR - offset] = IORegister::TimerDiv(0x00);
        r.registers[TIMER_COUNTER_ADDR - offset] = IORegister::Default(0x00);
        r.registers[TIMER_MOD_ADDR - offset] = IORegister::Default(0x00);
        r.registers[TIMER_CTRL_ADDR - offset] = IORegister::Default(0xF8);
        r.registers[0xFF0F - offset] = IORegister::Default(0xE1);
        r.registers[AUDIO_CH1_SWEEP_ADDR - offset] = IORegister::Default(0x80);
        r.registers[AUDIO_CH1_LENGTH_ADDR - offset] = IORegister::Default(0xBF);
        r.registers[AUDIO_CH1_VOLUME_ADDR - offset] = IORegister::Default(0xF3);
        r.registers[AUDIO_CH1_WAV_LO_ADDR - offset] = IORegister::Default(0xFF);
        r.registers[AUDIO_CH1_WAV_HI_ADDR - offset] = IORegister::Default(0xBF);
        r.registers[AUDIO_CH2_LENGTH_ADDR - offset] = IORegister::Default(0x3F);
        r.registers[AUDIO_CH2_VOLUME_ADDR - offset] = IORegister::Default(0x00);
        r.registers[AUDIO_CH2_WAV_LO_ADDR - offset] = IORegister::Default(0xFF);
        r.registers[AUDIO_CH2_WAV_HI_ADDR - offset] = IORegister::Default(0xBF);
        r.registers[AUDIO_CH3_DAC_ENABLE_ADDR - offset] = IORegister::Default(0x7F);
        r.registers[AUDIO_CH3_LENGTH_ADDR - offset] = IORegister::Default(0xFF);
        r.registers[AUDIO_CH3_OUTPUT_LVL_ADDR - offset] = IORegister::Default(0x9F);
        r.registers[AUDIO_CH3_WAV_LO_ADDR - offset] = IORegister::Default(0xFF);
        r.registers[AUDIO_CH3_WAV_HI_ADDR - offset] = IORegister::Default(0xBF);
        r.registers[AUDIO_CH4_LENGTH_ADDR - offset] = IORegister::Default(0xFF);
        r.registers[AUDIO_CH4_VOLUME_ADDR - offset] = IORegister::Default(0x00);
        r.registers[AUDIO_CH4_FREQ_ADDR - offset] = IORegister::Default(0x00);
        r.registers[AUDIO_CH4_CTRL_ADDR - offset] = IORegister::Default(0xBF);
        r.registers[AUDIO_GLOBAL_VOLUME_ADDR - offset] = IORegister::Default(0x77);
        r.registers[AUDIO_GLOBAL_PANNING_ADDR - offset] = IORegister::Default(0xF3);
        r.registers[AUDIO_GLOBAL_CTRL_ADDR - offset] = IORegister::Default(0xF1);
        r.registers[LCD_CONTROL_ADDR - offset] = IORegister::Default(0x91);
        r.registers[LCD_STAT_ADDR - offset] = IORegister::Default(0x81);
        r.registers[LCD_SCY_ADDR - offset] = IORegister::Default(0x00);
        r.registers[LCD_SCX_ADDR - offset] = IORegister::Default(0x00);
        r.registers[LCD_LY_ADDR - offset] = IORegister::Default(0x91);
        r.registers[LCD_LYC_ADDR - offset] = IORegister::Default(0x00);
        r.registers[0xFF46 - offset] = IORegister::Default(0xFF);
        r.registers[LCD_PALETTE_ADDR - offset] = IORegister::Default(0xFC);
        r.registers[0xFF48 - offset] = IORegister::Default(0x00);
        r.registers[0xFF49 - offset] = IORegister::Default(0x00);
        r.registers[LCD_WINY_ADDR - offset] = IORegister::Default(0x00);
        r.registers[LCD_WINX_ADDR - offset] = IORegister::Default(0x00);
        r.registers[0xFF4D - offset] = IORegister::Default(0xFF);
        r.registers[0xFF4F - offset] = IORegister::Default(0xFF);
        r.registers[0xFF51 - offset] = IORegister::Default(0xFF);
        r.registers[0xFF52 - offset] = IORegister::Default(0xFF);
        r.registers[0xFF53 - offset] = IORegister::Default(0xFF);
        r.registers[0xFF54 - offset] = IORegister::Default(0xFF);
        r.registers[0xFF55 - offset] = IORegister::Default(0xFF);
        r.registers[0xFF56 - offset] = IORegister::Default(0xFF);
        r.registers[0xFF68 - offset] = IORegister::Default(0xFF);
        r.registers[0xFF69 - offset] = IORegister::Default(0xFF);
        r.registers[0xFF6A - offset] = IORegister::Default(0xFF);
        r.registers[0xFF6B - offset] = IORegister::Default(0xFF);
        r.registers[0xFF70 - offset] = IORegister::Default(0xFF);

        return r;
    }

    pub fn read(&self, addr: usize) -> u8 {
        return self.registers[addr].read();
    }

    pub fn write(&mut self, addr: usize, val: u8) {
        self.registers[addr].write(val);
    }

    pub fn tick_timer_divider(&mut self) {
        self.registers[TIMER_DIV_ADDR - 0xFF00].tick();
    }
}

impl Default for IORegisters {
    fn default() -> Self {
        Self::new()
    }
}

impl IORegister {
    fn new(addr: usize) -> Self {
        match addr {
            TIMER_DIV_ADDR => IORegister::TimerDiv(0x00),
            _ => IORegister::Default(0x00),
        }
    }

    fn read(&self) -> u8 {
        match self {
            IORegister::TimerDiv(val) => *val,
            IORegister::Default(val) => *val,
        }
    }

    fn write(&mut self, val: u8) {
        match self {
            // Writing to TimerDiv resets it to 0, no matter the value.
            IORegister::TimerDiv(_) => *self = IORegister::TimerDiv(0x00),
            IORegister::Default(_) => *self = IORegister::Default(val),
        }
    }

    fn tick(&mut self) {
        match self {
            IORegister::TimerDiv(val) => *self = IORegister::TimerDiv(val.wrapping_add(1)),
            _ => panic!("attempted to tick a non-timer io register"),
        }
    }
}
