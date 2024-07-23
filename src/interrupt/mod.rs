use crate::memory::io_registers;

#[derive(Debug)]
pub struct Bus {
    flag_bits: u8,
    enable_bits: u8,
}

pub enum Interrupt {
    VBlank,
    LcdStat,
    TimerOverflow,
    Serial,
    Joypad,
}

impl Bus {
    pub fn new() -> Bus {
        Bus {
            flag_bits: 0,
            enable_bits: 0,
        }
    }

    pub fn request_interrupt(&mut self, interrupt: Interrupt) {
        let interrupt_bit = match interrupt {
            Interrupt::VBlank => 0,
            Interrupt::LcdStat => 1,
            Interrupt::TimerOverflow => 2,
            Interrupt::Serial => 3,
            Interrupt::Joypad => 4,
        };

        self.flag_bits |= 1 << interrupt_bit;
    }

    pub fn get_highest_priority_interrupt(&mut self) -> Option<Interrupt> {
        for interrupt_bit in 0..5 {
            if (self.flag_bits & (1 << interrupt_bit)) != 0
                && (self.enable_bits & (1 << interrupt_bit)) != 0
            {
                return match interrupt_bit {
                    0 => Some(Interrupt::VBlank),
                    1 => Some(Interrupt::LcdStat),
                    2 => Some(Interrupt::TimerOverflow),
                    3 => Some(Interrupt::Serial),
                    4 => Some(Interrupt::Joypad),
                    _ => None,
                };
            }
        }

        None
    }

    pub fn clear_interrupt(&mut self, interrupt: Interrupt) {
        let interrupt_bit = match interrupt {
            Interrupt::VBlank => 0,
            Interrupt::LcdStat => 1,
            Interrupt::TimerOverflow => 2,
            Interrupt::Serial => 3,
            Interrupt::Joypad => 4,
        };

        self.flag_bits &= !(1 << interrupt_bit);
    }

    pub fn read(&self, addr: usize) -> u8 {
        match addr {
            io_registers::INTERRUPT_FLAG_REGISTER_ADDR => self.flag_bits,
            io_registers::INTERRUPT_ENABLE_REGISTER_ADDR => self.enable_bits,
            _ => panic!("Invalid interrupt register read"),
        }
    }

    pub fn write(&mut self, addr: usize, value: u8) {
        match addr {
            io_registers::INTERRUPT_FLAG_REGISTER_ADDR => self.flag_bits = value,
            io_registers::INTERRUPT_ENABLE_REGISTER_ADDR => self.enable_bits = value,
            _ => panic!("Invalid interrupt register write"),
        }
    }
}
