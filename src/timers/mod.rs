use crate::memory::io_registers;
use crate::{cpu, memory};
const TIMER_CONTROL_ENABLED_MASK: u8 = 1 << 2;

#[derive(Debug)]
pub struct Timers {
    system_clock: u16,

    prev_bit_1: bool,
    prev_bit_7: bool,
    prev_bit_5: bool,
    prev_bit_3: bool,

    current_bit_1: bool,
    current_bit_7: bool,
    current_bit_5: bool,
    current_bit_3: bool,
}

impl Timers {
    pub fn new() -> Timers {
        Timers {
            system_clock: 0,

            prev_bit_1: false,
            prev_bit_7: false,
            prev_bit_5: false,
            prev_bit_3: false,

            current_bit_1: false,
            current_bit_7: false,
            current_bit_5: false,
            current_bit_3: false,
        }
    }

    pub fn set_post_bootrom_state(&mut self) {
        self.system_clock = 0xABCC;
        self.current_bit_7 = self.system_clock & (1 << 7) > 0;
        self.current_bit_5 = self.system_clock & (1 << 5) > 0;
        self.current_bit_3 = self.system_clock & (1 << 3) > 0;
        self.current_bit_1 = self.system_clock & (1 << 1) > 0;

        let previous_system_clock = self.system_clock.wrapping_sub(1);
        self.prev_bit_1 = previous_system_clock & (1 << 1) > 0;
        self.prev_bit_7 = previous_system_clock & (1 << 7) > 0;
        self.prev_bit_5 = previous_system_clock & (1 << 5) > 0;
        self.prev_bit_3 = previous_system_clock & (1 << 3) > 0;
    }

    pub fn reset(&mut self) {
        *self = Timers::new();
    }

    pub fn reset_sys_clock(&mut self) {
        self.system_clock = 0;
    }

    pub fn increment(&mut self, memory: &mut memory::Memory, cpu: &mut cpu::LR35902) {
        if cpu.is_stopped() {
            return;
        }

        let timer_control_register = match memory.dma_read(io_registers::TIMER_CTRL_ADDR) {
            Some(val) => val,
            None => panic!("Timer control register not found"),
        };

        self.system_clock = self.system_clock.wrapping_add(1);

        self.current_bit_7 = self.system_clock & (1 << 7) > 0;
        self.current_bit_5 = self.system_clock & (1 << 5) > 0;
        self.current_bit_3 = self.system_clock & (1 << 3) > 0;
        self.current_bit_1 = self.system_clock & (1 << 1) > 0;

        // DIV is essentially the upper byte of the system clock
        memory.dma_write(io_registers::TIMER_DIV_ADDR, (self.system_clock >> 8) as u8);

        let _sample = memory.dma_read(io_registers::TIMER_DIV_ADDR).unwrap();

        if timer_control_register & TIMER_CONTROL_ENABLED_MASK > 0 {
            let (current_bit_to_use, prev_bit_to_use) = match timer_control_register & 0x03 {
                0 => (self.current_bit_7, self.prev_bit_7),
                3 => (self.current_bit_5, self.prev_bit_5),
                2 => (self.current_bit_3, self.prev_bit_3),
                1 => (self.current_bit_1, self.prev_bit_1),
                _ => panic!("Invalid timer control register value"),
            };

            // Falling edge detection
            if prev_bit_to_use && !current_bit_to_use {
                let timer_register = memory
                    .dma_read(io_registers::TIMER_COUNTER_ADDR)
                    .unwrap()
                    .wrapping_add(1);

                if timer_register == 0xFF {
                    cpu.request_interrupt(memory, cpu::Interrupt::TimerOverflow);
                    let timer_mod = memory.dma_read(io_registers::TIMER_MOD_ADDR).unwrap();
                    memory.dma_write(io_registers::TIMER_COUNTER_ADDR, timer_mod);
                    log::trace!("Timer interrupt requested");
                } else {
                    memory.dma_write(io_registers::TIMER_COUNTER_ADDR, timer_register);
                }
            }
        }

        self.prev_bit_1 = self.current_bit_1;
        self.prev_bit_7 = self.current_bit_7;
        self.prev_bit_5 = self.current_bit_5;
        self.prev_bit_3 = self.current_bit_3;
    }
}
