use crate::memory::io_registers;
use crate::{cpu, events, memory};
const TIMER_CONTROL_ENABLED_MASK: u8 = 1 << 2;

#[derive(Debug)]
pub struct Timers {
    system_clock: u16,

    /// Div Timer Reset event subscriber
    div_reset_subscriber: crossbeam::channel::Receiver<()>,

    prev_bit_1: bool,
    prev_bit_7: bool,
    prev_bit_5: bool,
    prev_bit_3: bool,

    current_bit_1: bool,
    current_bit_7: bool,
    current_bit_5: bool,
    current_bit_3: bool,

    pending_interrupt: bool,
}

impl Timers {
    pub fn new() -> Timers {
        let recv = events::subscribe(events::Event::TimerDivWrite);

        Timers {
            div_reset_subscriber: recv,
            system_clock: 0,

            prev_bit_1: false,
            prev_bit_7: false,
            prev_bit_5: false,
            prev_bit_3: false,

            current_bit_1: false,
            current_bit_7: false,
            current_bit_5: false,
            current_bit_3: false,

            pending_interrupt: false,
        }
    }

    pub fn reset(&mut self) {
        self.system_clock = 0;
        self.prev_bit_1 = false;
        self.prev_bit_7 = false;
        self.prev_bit_5 = false;
        self.prev_bit_3 = false;
        self.current_bit_1 = false;
        self.current_bit_7 = false;
        self.current_bit_5 = false;
        self.current_bit_3 = false;
        self.pending_interrupt = false;
    }

    pub fn update(&mut self, cycles: u32, memory: &mut memory::Memory, cpu: &mut cpu::LR35902) {
        match self.div_reset_subscriber.try_recv() {
            Ok(_) => {
                self.system_clock = 0;
            }
            _ => {}
        }

        if self.pending_interrupt {
            cpu.request_interrupt(memory, cpu::Interrupt::TimerOverflow);
            self.pending_interrupt = false;
            let timer_mod = memory.dma_read(io_registers::TIMER_MOD_ADDR).unwrap();
            memory.dma_write(io_registers::TIMER_COUNTER_ADDR, timer_mod);
            log::trace!("Timer interrupt requested");
        }

        let timer_control_register = match memory.dma_read(io_registers::TIMER_CTRL_ADDR) {
            Some(val) => val,
            None => panic!("Timer control register not found"),
        };

        for _ in 0..cycles / 4 {
            self.system_clock = self.system_clock.wrapping_add(1) & 0xFFFF;

            self.current_bit_7 = self.system_clock & (1 << 7) > 0;
            self.current_bit_5 = self.system_clock & (1 << 5) > 0;
            self.current_bit_3 = self.system_clock & (1 << 3) > 0;
            self.current_bit_1 = self.system_clock & (1 << 1) > 0;

            // DIV is essentially the upper byte of the system clock
            memory.dma_write(io_registers::TIMER_DIV_ADDR, (self.system_clock >> 8) as u8);

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
                    let timer_register = memory.dma_read(io_registers::TIMER_COUNTER_ADDR).unwrap();

                    if timer_register == 0xFF {
                        self.pending_interrupt = true;
                    } else {
                        memory.dma_write(
                            io_registers::TIMER_COUNTER_ADDR,
                            timer_register.wrapping_add(1),
                        );
                    }
                }
            }
        }

        self.prev_bit_1 = self.current_bit_1;
        self.prev_bit_7 = self.current_bit_7;
        self.prev_bit_5 = self.current_bit_5;
        self.prev_bit_3 = self.current_bit_3;
    }
}
