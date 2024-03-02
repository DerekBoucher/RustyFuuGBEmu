use crate::interface;
use crate::interface::mock;
use crate::memory::*;
use std::vec;

#[test]
fn read() {
    struct TestCase {
        description: String,
        init_fn: fn() -> Memory,
        assert_fn: fn(Memory),
    }

    let test_cases: Vec<TestCase> = vec![
        TestCase{
            description: String::from("reading from memory region 0x0000 - 0x0100, boot-rom enabled -> should read from bootrom"),
            init_fn: || -> Memory {
                Memory::new(mock::Cartridge::new(vec![0x0; 0x10000]))
            },
            assert_fn: |memory| {
                let read_result = memory.read(0x0);
                assert_eq!(read_result.unwrap(), 0x31);
            }
        },
        TestCase{
            description: String::from("reading from memory region 0x0000 - 0x8000, should access cartridge rom"),
            init_fn: || -> Memory {
                let mut cartridge: Box<dyn interface::Cartridge> = mock::Cartridge::new(vec![0x0; 0x10000]);
                cartridge.write(0x0000, 0x17);
                let mut mem = Memory::new(cartridge);
                mem.io_registers[io_registers::BOOT_ROM_DISABLE_ADDR - 0xFF00] = 0x1;
                return mem;
            },
            assert_fn: |memory| {
                let read_result = memory.read(0x0);
                assert_eq!(read_result.unwrap(), 0x17);
            }
        },
        TestCase{
            description: String::from("reading from memory region 0x8000 - 0xA000, should access video RAM"),
            init_fn: || -> Memory {
                let mut mem = Memory::new(mock::Cartridge::new(vec![0x0; 0x10000]));
                mem.video_ram[0x100] = 0x37;
                return mem;
            },
            assert_fn: |memory| {
                let read_result = memory.read(0x8100);
                assert_eq!(read_result.unwrap(), 0x37);
            }
        },
        TestCase{
            description: String::from("reading from memory region 0xA000 - 0xC000, should access cartridge RAM"),
            init_fn: || -> Memory {
                Memory::new(mock::Cartridge::new(vec![0x0; 0x10000]))
            },
            assert_fn: |memory| {
                let read_result = memory.read(0xA000);
                assert_eq!(read_result.unwrap(), 0x00);
            }
        },
        TestCase{
            description: String::from("reading from memory region 0xC000 - 0xD000, should access work ram 0"),
            init_fn: || -> Memory {
                let mut mem = Memory::new(mock::Cartridge::new(vec![0x0; 0x10000]));
                mem.work_ram0[0x0100] = 0x17;
                return mem;
            },
            assert_fn: |memory| {
                let read_result = memory.read(0xC100);
                assert_eq!(read_result.unwrap(), 0x17);
            }
        },
        TestCase{
            description: String::from("reading from memory region 0xD000 - 0xE000, should access work ram 1"),
            init_fn: || -> Memory {
                let mut mem = Memory::new(mock::Cartridge::new(vec![0x0; 0x10000]));
                mem.work_ram1[0x0100] = 0x17;
                return mem;
            },
            assert_fn: |memory| {
                let read_result = memory.read(0xD100);
                assert_eq!(read_result.unwrap(), 0x17);
            }
        },
        TestCase{
            description: String::from("reading from memory region 0xE000 - 0xFE00, should access work ram 0 (through echo ram)"),
            init_fn: || -> Memory {
                let mut mem = Memory::new(mock::Cartridge::new(vec![0x0; 0x10000]));
                mem.work_ram0[0x0100] = 0x17;
                return mem;
            },
            assert_fn: |memory| {
                let read_result = memory.read(0xE100);
                assert_eq!(read_result.unwrap(), 0x17);
            }
        },
        TestCase{
            description: String::from("reading from memory region 0xFE00 - 0xFF00, should access sprites attributes"),
            init_fn: || -> Memory {
                let mut mem = Memory::new(mock::Cartridge::new(vec![0x0; 0x10000]));
                mem.sprite_attributes[0x0000] = 0x17;
                return mem;
            },
            assert_fn: |memory| {
                let read_result = memory.read(0xFE00);
                assert_eq!(read_result.unwrap(), 0x17);
            }
        },
        TestCase{
            description: String::from("reading from memory region 0xFF00 - 0xFF80, should access io registers"),
            init_fn: || -> Memory {
                let mut mem = Memory::new(mock::Cartridge::new(vec![0x0; 0x10000]));
                mem.io_registers[0x0000] = 0x17;
                return mem;
            },
            assert_fn: |memory| {
                let read_result = memory.read(0xFF00);
                assert_eq!(read_result.unwrap(), 0x17);
            }
        },
        TestCase{
            description: String::from("reading from memory region 0xFF80 - 0xFFFF, should access high ram"),
            init_fn: || -> Memory {
                let mut mem = Memory::new(mock::Cartridge::new(vec![0x0; 0x10000]));
                mem.hi_ram[0x7E] = 0x17;
                return mem;
            },
            assert_fn: |memory| {
                let read_result = memory.read(0xFFFE);
                assert_eq!(read_result.unwrap(), 0x17);
            }
        },
        TestCase{
            description: String::from("reading from memory region 0xFFFF, should access master interrupt enable register"),
            init_fn: || -> Memory {
                let mut mem = Memory::new(mock::Cartridge::new(vec![0x0; 0x10000]));
                mem.interrupt_enable_register = 0x17;
                return mem;
            },
            assert_fn: |memory| {
                let read_result = memory.read(0xFFFF);
                assert_eq!(read_result.unwrap(), 0x17);
            }
        },
        TestCase{
            description: String::from("reading from memory region > 0xFFFF, should return none"),
            init_fn: || -> Memory {
                Memory::new(mock::Cartridge::new(vec![0x0; 0x10000]))
            },
            assert_fn: |memory| {
                let read_result = memory.read(0x10000);
                assert!(read_result.is_none());
            }
        },
        TestCase{
            description: String::from("reading from a memory while the oam dma transfer is in progress, should return 0xFF dummy data"),
            init_fn: || -> Memory {
                let mut mem = Memory::new(mock::Cartridge::new(vec![0x0; 0x10000]));
                mem.oam_dma_transfer_in_progress = true;
                return mem;
            },
            assert_fn: |memory| {
                let read_result = memory.read(0x0000);
                assert_eq!(read_result.unwrap(), 0xFF);
            }
        },
        TestCase{
            description: String::from("reading from a HI RAM while the oam dma transfer is in progress, should still return the data in the HI RAM"),
            init_fn: || -> Memory {
                let mut mem = Memory::new(mock::Cartridge::new(vec![0x0; 0x10000]));
                mem.hi_ram[0x00] = 0x17;
                mem.oam_dma_transfer_in_progress = true;
                return mem;
            },
            assert_fn: |memory| {
                let read_result = memory.read(0xFF80);
                assert_eq!(read_result.unwrap(), 0x17);
            }
        },
    ];

    for tc in test_cases {
        println!("{}", tc.description);
        let memory = (tc.init_fn)();
        (tc.assert_fn)(memory);
    }
}

#[test]
fn write() {
    struct TestCase {
        description: String,
        init_fn: fn() -> Memory,
        assert_fn: fn(Memory),
    }

    let test_cases: Vec<TestCase> = vec![
        TestCase {
            description: String::from(
                "writing to region 0x0000 - 0x8000 -> should write to cartridge",
            ),
            init_fn: || -> Memory { Memory::new(mock::Cartridge::new(vec![0x0; 0x10000])) },
            assert_fn: |mut memory| {
                memory.write(0x0000, 0x17);
                assert_eq!(memory.read(0x0000).unwrap(), 0x31); //boot rom
            },
        },
        TestCase {
            description: String::from(
                "writing to region 0x8000 - 0xA000 -> should write to video ram",
            ),
            init_fn: || -> Memory { Memory::new(mock::Cartridge::new(vec![0x0; 0x10000])) },
            assert_fn: |mut memory| {
                memory.write(0x8000, 0x17);
                assert_eq!(memory.video_ram[0x0000], 0x17);
            },
        },
        TestCase {
            description: String::from(
                "writing to region 0xA000 - 0xC000 -> should write to cartridge ram",
            ),
            init_fn: || -> Memory { Memory::new(mock::Cartridge::new(vec![0x0; 0x10000])) },
            assert_fn: |mut memory| {
                memory.write(0xA000, 0x17);
                assert_eq!(memory.read(0xA000).unwrap(), 0x17);
            },
        },
        TestCase {
            description: String::from(
                "writing to region 0xC000 - 0xD000 -> should write to work ram 0",
            ),
            init_fn: || -> Memory { Memory::new(mock::Cartridge::new(vec![0x0; 0x10000])) },
            assert_fn: |mut memory| {
                memory.write(0xC000, 0x17);
                assert_eq!(memory.work_ram0[0x0000], 0x17);
            },
        },
        TestCase {
            description: String::from(
                "writing to region 0xD000 - 0xE000 -> should write to work ram 1",
            ),
            init_fn: || -> Memory { Memory::new(mock::Cartridge::new(vec![0x0; 0x10000])) },
            assert_fn: |mut memory| {
                memory.write(0xD000, 0x17);
                assert_eq!(memory.work_ram1[0x0000], 0x17);
            },
        },
        TestCase {
            description: String::from(
                "writing to region 0xE000 - 0xFE00 -> should write to work ram 0 (echo ram)",
            ),
            init_fn: || -> Memory { Memory::new(mock::Cartridge::new(vec![0x0; 0x10000])) },
            assert_fn: |mut memory| {
                memory.write(0xE000, 0x17);
                assert_eq!(memory.work_ram0[0x0000], 0x17);
            },
        },
        TestCase {
            description: String::from(
                "writing to region 0xFE00 - 0xFF00 -> should not result in any writes",
            ),
            init_fn: || -> Memory { Memory::new(mock::Cartridge::new(vec![0x0; 0x10000])) },
            assert_fn: |mut memory| {
                memory.write(0xFE00, 0x17);
                assert_eq!(memory.sprite_attributes[0x0000], 0x00);
            },
        },
        TestCase {
            description: String::from(
                "writing to the oam dma transfer register -> should begin dma transfer and put expected data in the sprites attributes",
            ),
            init_fn: || -> Memory {
                let mut cart_data: Vec<u8> = vec![0x0; 0x10000];
                for i in 0x1800..0x18A0 {
                    cart_data[i] = 0xFF;
                }
                return Memory::new(mock::Cartridge::new(cart_data));
            },
            assert_fn: |mut memory| {
                memory.write(io_registers::OAM_DMA_TRANSFER_ADDR, 0x18);
                assert!(memory.oam_dma_transfer_in_progress);
                for i in 0..0xA0 {
                    assert_eq!(memory.sprite_attributes[i], 0xFF);
                }
            },
        },
        TestCase {
            description: String::from(
                "writing to region 0xFF80 - 0xFFFF -> should write to high ram",
            ),
            init_fn: || -> Memory { Memory::new(mock::Cartridge::new(vec![0x0; 0x10000])) },
            assert_fn: |mut memory| {
                memory.write(0xFF80, 0x17);
                assert_eq!(memory.hi_ram[0x0000], 0x17);
            },
        },
        TestCase {
            description: String::from(
                "writing to region 0xFFFF -> should write to master interrupt enable register",
            ),
            init_fn: || -> Memory { Memory::new(mock::Cartridge::new(vec![0x0; 0x10000])) },
            assert_fn: |mut memory| {
                memory.write(0xFFFF, 0x17);
                assert_eq!(memory.interrupt_enable_register, 0x17);
            },
        },
    ];

    for tc in test_cases {
        println!("{}", tc.description);
        let memory = (tc.init_fn)();
        (tc.assert_fn)(memory);
    }
}

#[test]
fn update_dma_transfer_cycles() {
    struct TestCase {
        description: String,
        init_fn: fn() -> Memory,
        cycles: u32,
        assert_fn: fn(Memory),
    }

    let test_cases: Vec<TestCase> = vec![
        TestCase {
            description: String::from(
                "dma transfer in progress, 4 cycles passed -> should not update the dma transfer in progress flag",
            ),
            init_fn: || -> Memory {
                let mut mem = Memory::new(mock::Cartridge::new(vec![0x0; 0x10000]));
                mem.oam_dma_transfer_in_progress = true;
                return mem;
            },
            cycles: 4,
            assert_fn: |memory| {
                assert!(memory.oam_dma_transfer_in_progress);
            },
        },
        TestCase {
            description: String::from(
                "dma transfer in progress, 160 cycles passed -> should set the dma transfer in progress flag to false",
            ),
            init_fn: || -> Memory {
                let mut mem = Memory::new(mock::Cartridge::new(vec![0x0; 0x10000]));
                mem.oam_dma_transfer_in_progress = true;
                return mem;
            },
            cycles: 160,
            assert_fn: |memory| {
                assert!(!memory.oam_dma_transfer_in_progress);
            },
        },
        TestCase {
            description: String::from(
                "dma transfer not in progress, 160 cycles passed -> should reset the dma transfer cycles to 0",
            ),
            init_fn: || -> Memory {
                let mut mem = Memory::new(mock::Cartridge::new(vec![0x0; 0x10000]));
                mem.oam_dma_transfer_in_progress = false;
                mem.oam_dma_transfer_cycles_completed = 160;
                return mem;
            },
            cycles: 160,
            assert_fn: |memory| {
                assert!(!memory.oam_dma_transfer_in_progress);
                assert_eq!(memory.oam_dma_transfer_cycles_completed, 0);
            },
        },
    ];

    for (i, tc) in test_cases.iter().enumerate() {
        println!("Test case #{}: {}", i, tc.description);
        let mut memory = (tc.init_fn)();
        memory.update_dma_transfer_cycles(tc.cycles);
        (tc.assert_fn)(memory);
    }
}

#[test]
fn boot_rom_enabled() {
    let mut mem = Memory::new(mock::Cartridge::new(vec![0x0; 0x10000]));
    mem.io_registers[io_registers::BOOT_ROM_DISABLE_ADDR - 0xFF00] = 0x1;
    assert!(!mem.boot_rom_enabled());
    mem.io_registers[io_registers::BOOT_ROM_DISABLE_ADDR - 0xFF00] = 0x0;
    assert!(mem.boot_rom_enabled());
}
