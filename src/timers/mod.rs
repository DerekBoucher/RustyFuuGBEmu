use crate::interface;
use crate::memory::io_registers;
const TIMER_CONTROL_ENABLED_MASK: u8 = 1 << 2;

#[derive(Debug)]
pub struct Timers {
    /// Counter used to keep track of when the divider register timer
    /// should be incremented. The divider register is incremented whenever this
    /// counter reaches 256.
    divider_tick_counter: u32,

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

    fn update(
        &mut self,
        cycles: u32,
        memory: &mut impl interface::Memory,
        cpu: &mut impl interface::CPU,
    ) {
        let timer_control_register = match memory.read(io_registers::TIMER_CTRL_ADDR) {
            Some(val) => val,
            None => panic!("Timer control register not found"),
        };

        self.divider_tick_counter += cycles;
        if self.divider_tick_counter >= 256 {
            self.divider_tick_counter -= 256;
            let incremented_div_timer = memory
                .read(io_registers::TIMER_DIV_ADDR)
                .unwrap()
                .wrapping_add(1);
            memory.write(io_registers::TIMER_DIV_ADDR, incremented_div_timer);
            log::trace!(
                "Divider register incremented to {:X}",
                incremented_div_timer
            );
        }

        if timer_control_register & TIMER_CONTROL_ENABLED_MASK > 0 {
            self.timer_tick_counter -= cycles as i32;

            while self.timer_tick_counter <= 0 {
                self.timer_tick_counter += match timer_control_register & 0x03 {
                    0 => 1024,
                    1 => 16,
                    2 => 64,
                    3 => 256,
                    _ => panic!("Invalid timer control register value"),
                } as i32;

                let timer_register = memory.read(io_registers::TIMER_COUNTER_ADDR).unwrap();
                if timer_register == 0xFF {
                    let timer_mod = memory.read(io_registers::TIMER_MOD_ADDR).unwrap();
                    memory.write(io_registers::TIMER_COUNTER_ADDR, timer_mod);

                    cpu.request_interrupt(memory, interface::Interrupt::TimerOverflow);
                    log::trace!("Timer interrupt requested");
                    break;
                }

                memory.write(
                    io_registers::TIMER_COUNTER_ADDR,
                    timer_register.wrapping_add(1),
                );
            }
        }
    }
}

impl interface::Timers for Timers {
    fn update(
        &mut self,
        cycles: u32,
        memory: &mut impl interface::Memory,
        cpu: &mut impl interface::CPU,
    ) {
        self.update(cycles, memory, cpu);
    }

    fn reset(&mut self) {
        *self = Timers::new();
    }
}
