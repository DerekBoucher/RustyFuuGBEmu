use std::sync::{self, Arc};

use crate::{interrupt, memory::io_registers};
const TIMER_CONTROL_ENABLED_MASK: u8 = 1 << 2;

#[derive(Debug)]
pub struct Timers {
    system_clock: u16,

    prev_bit_7: bool,
    prev_bit_5: bool,
    prev_bit_3: bool,
    prev_bit_1: bool,

    current_bit_7: bool,
    current_bit_5: bool,
    current_bit_3: bool,
    current_bit_1: bool,

    interrupt_pending: bool,

    tima: u8, // Timer
    tma: u8,  // Timer modulo
    tac: u8,  // Timer control register
}

impl Timers {
    pub fn new() -> Timers {
        Timers {
            system_clock: 0,

            prev_bit_7: false,
            prev_bit_5: false,
            prev_bit_3: false,
            prev_bit_1: false,

            current_bit_7: false,
            current_bit_5: false,
            current_bit_3: false,
            current_bit_1: false,

            interrupt_pending: false,
            tima: 0,
            tma: 0,
            tac: 0,
        }
    }

    pub fn read(&self, addr: usize) -> u8 {
        match addr {
            io_registers::TIMER_DIV_ADDR => (self.system_clock >> 8) as u8,
            io_registers::TIMER_COUNTER_ADDR => self.tima,
            io_registers::TIMER_MOD_ADDR => self.tma,
            io_registers::TIMER_CTRL_ADDR => self.tac & 0b111,
            _ => panic!("Invalid timer register read"),
        }
    }

    pub fn write(&mut self, addr: usize, value: u8) {
        match addr {
            io_registers::TIMER_DIV_ADDR => self.reset_sys_clock(),
            io_registers::TIMER_COUNTER_ADDR => self.tima = value,
            io_registers::TIMER_MOD_ADDR => self.tma = value,
            io_registers::TIMER_CTRL_ADDR => self.tac = value & 0b111,
            _ => panic!("Invalid timer register write"),
        }
    }

    pub fn set_post_boot_rom_state(&mut self) {
        self.system_clock = 0xABCC;
        self.current_bit_7 = self.system_clock & (1 << 7) > 0;
        self.current_bit_5 = self.system_clock & (1 << 5) > 0;
        self.current_bit_3 = self.system_clock & (1 << 3) > 0;
        self.current_bit_1 = self.system_clock & (1 << 1) > 0;

        let previous_system_clock = self.system_clock.wrapping_sub(1);
        self.prev_bit_7 = previous_system_clock & (1 << 7) > 0;
        self.prev_bit_5 = previous_system_clock & (1 << 5) > 0;
        self.prev_bit_3 = previous_system_clock & (1 << 3) > 0;
        self.prev_bit_1 = previous_system_clock & (1 << 1) > 0;

        self.interrupt_pending = false;
        self.tac = 0xF8;
        self.tima = 0x00;
        self.tma = 0x00;
    }

    pub fn reset(&mut self) {
        *self = Timers::new();
    }

    pub fn reset_sys_clock(&mut self) {
        self.system_clock = 0;
        self.current_bit_7 = false;
        self.current_bit_5 = false;
        self.current_bit_3 = false;
        self.current_bit_1 = false;
    }

    pub fn step(&mut self, interrupt_bus: &Arc<sync::Mutex<crate::interrupt::Bus>>) {
        if self.interrupt_pending {
            self.interrupt_pending = false;
            self.tima = self.tma;
            interrupt_bus
                .lock()
                .unwrap()
                .request_interrupt(interrupt::Interrupt::TimerOverflow);
        }

        self.system_clock = self.system_clock.wrapping_add(1);

        for _ in 0..4 {
            // Aka a machine cycle
            self.current_bit_7 = self.system_clock & (1 << 7) > 0;
            self.current_bit_5 = self.system_clock & (1 << 5) > 0;
            self.current_bit_3 = self.system_clock & (1 << 3) > 0;
            self.current_bit_1 = self.system_clock & (1 << 1) > 0;

            if self.tac & TIMER_CONTROL_ENABLED_MASK > 0 {
                let (current_bit_to_use, prev_bit_to_use) = match self.tac & 0b11 {
                    0 => (self.current_bit_7, self.prev_bit_7),
                    3 => (self.current_bit_5, self.prev_bit_5),
                    2 => (self.current_bit_3, self.prev_bit_3),
                    1 => (self.current_bit_1, self.prev_bit_1),
                    _ => panic!("Invalid timer control register value"),
                };

                // Falling edge detection
                if prev_bit_to_use && !current_bit_to_use {
                    if self.tima == 0xFF {
                        // Both the interrupt + the TIMA reload are delayed 1 machine cycle
                        // In the meantime, TIMA is resetted with 0
                        self.interrupt_pending = true;
                    }

                    self.tima = self.tima.wrapping_add(1);
                }

                self.prev_bit_7 = self.current_bit_7;
                self.prev_bit_5 = self.current_bit_5;
                self.prev_bit_3 = self.current_bit_3;
                self.prev_bit_1 = self.current_bit_1;
            }
        }
    }
}
