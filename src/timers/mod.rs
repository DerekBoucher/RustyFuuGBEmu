use crate::memory::io_registers;
use crate::{cpu, memory};
const TIMER_CONTROL_ENABLED_MASK: u8 = 1 << 2;
const DIV_TIMER_FREQUENCY: u32 = 16384;
const CYCLES_PER_DIVIDER_TICK: i32 = (cpu::CPU_FREQUENCY / DIV_TIMER_FREQUENCY) as i32;
const TIMER_FREQUENCY_00: u32 = 4096;
const TIMER_FREQUENCY_01: u32 = 262144;
const TIMER_FREQUENCY_10: u32 = 65536;
const TIMER_FREQUENCY_11: u32 = 16384;

#[derive(Debug)]
pub struct Timers {
    /// Counter used to keep track of when the divider register timer
    /// should be incremented. The divider register is incremented whenever this
    /// counter reaches 256.
    divider_tick_counter: i32,

    /// Similar to the divider register tick counter, this counter is used to determine when
    /// to increment the timer register. The timer register is incremented whenever this ticker
    /// is less than or equal to 0.
    timer_tick_counter: i32,
}

impl Timers {
    pub fn new() -> Timers {
        Timers {
            divider_tick_counter: 0,
            timer_tick_counter: 0,
        }
    }

    pub fn reset(&mut self) {
        *self = Timers::new();
    }

    pub fn update<'a>(&mut self, cycles: u32, memory: &mut memory::Memory, cpu: &mut cpu::LR35902) {
        let timer_control_register = match memory.dma_read(io_registers::TIMER_CTRL_ADDR) {
            Some(val) => val,
            None => panic!("Timer control register not found"),
        };

        self.divider_tick_counter += cycles as i32;
        while self.divider_tick_counter >= CYCLES_PER_DIVIDER_TICK {
            self.divider_tick_counter -= CYCLES_PER_DIVIDER_TICK;
            let incremented_div_timer = memory
                .dma_read(io_registers::TIMER_DIV_ADDR)
                .unwrap()
                .wrapping_add(1);
            memory.dma_write(io_registers::TIMER_DIV_ADDR, incremented_div_timer);
            log::trace!(
                "Divider register incremented to {:X}",
                incremented_div_timer
            );
        }

        if timer_control_register & TIMER_CONTROL_ENABLED_MASK > 0 {
            self.timer_tick_counter -= cycles as i32;

            let timer_frequency = match timer_control_register & 0x03 {
                0 => TIMER_FREQUENCY_00,
                1 => TIMER_FREQUENCY_01,
                2 => TIMER_FREQUENCY_10,
                3 => TIMER_FREQUENCY_11,
                _ => panic!("Invalid timer control register value"),
            };

            while self.timer_tick_counter >= timer_frequency as i32 {
                self.timer_tick_counter -= timer_frequency as i32;

                let timer_register = memory
                    .dma_read(io_registers::TIMER_COUNTER_ADDR)
                    .unwrap()
                    .wrapping_add(1);

                if timer_register == 0x00 {
                    let timer_mod = memory.dma_read(io_registers::TIMER_MOD_ADDR).unwrap();
                    memory.dma_write(io_registers::TIMER_COUNTER_ADDR, timer_mod);

                    cpu.request_interrupt(memory, cpu::Interrupt::TimerOverflow);
                    log::trace!("Timer interrupt requested");
                }

                memory.dma_write(io_registers::TIMER_COUNTER_ADDR, timer_register);
            }
        }
    }
}
