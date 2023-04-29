use crate::memory::{memory::io_registers, *};
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
                    Memory::new(vec![0x0; 0x10000])
                },
                assert_fn: |memory| {
                    let read_result = memory.read(0x0);
                    assert_eq!(*read_result.unwrap(), 0x31);
                }
            },
            TestCase{
                description: String::from("reading from memory region 0x0000 - 0x8000, should access cartridge rom"),
                init_fn: || -> Memory {
                    let mut cart_data: Vec<u8> = vec![0x0; 0x10000];
                    cart_data[0x0] = 0x17;
                    let mut mem = Memory::new(cart_data);
                    mem.io_registers[memory::io_registers::BOOT_ROM_DISABLE_ADDR - 0xFF00] = 0x1;
                    return mem;
                },
                assert_fn: |memory| {
                    let read_result = memory.read(0x0);
                    assert_eq!(*read_result.unwrap(), 0x17);
                }
            },
            TestCase{
                description: String::from("reading from memory region 0x8000 - 0xA000, should access video RAM"),
                init_fn: || -> Memory {
                    let mut mem = Memory::new(vec![0x0; 0x10000]);
                    mem.video_ram[0x100] = 0x37;
                    return mem;
                },
                assert_fn: |memory| {
                    let read_result = memory.read(0x8100);
                    assert_eq!(*read_result.unwrap(), 0x37);
                }
            },
            TestCase{
                description: String::from("reading from memory region 0xA000 - 0xC000, should access cartridge RAM"),
                init_fn: || -> Memory {
                    Memory::new(vec![0x0; 0x10000])
                },
                assert_fn: |memory| {
                    let read_result = memory.read(0xA000);
                    assert_eq!(*read_result.unwrap(), 0x00);
                }
            },
            TestCase{
                description: String::from("reading from memory region 0xC000 - 0xD000, should access work ram 0"),
                init_fn: || -> Memory {
                    let mut mem = Memory::new(vec![0x0; 0x10000]);
                    mem.work_ram0[0x0100] = 0x17;
                    return mem;
                },
                assert_fn: |memory| {
                    let read_result = memory.read(0xC100);
                    assert_eq!(*read_result.unwrap(), 0x17);
                }
            },
            TestCase{
                description: String::from("reading from memory region 0xD000 - 0xE000, should access work ram 1"),
                init_fn: || -> Memory {
                    let mut mem = Memory::new(vec![0x0; 0x10000]);
                    mem.work_ram1[0x0100] = 0x17;
                    return mem;
                },
                assert_fn: |memory| {
                    let read_result = memory.read(0xD100);
                    assert_eq!(*read_result.unwrap(), 0x17);
                }
            },
            TestCase{
                description: String::from("reading from memory region 0xE000 - 0xFE00, should access work ram 0 (through echo ram)"),
                init_fn: || -> Memory {
                    let mut mem = Memory::new(vec![0x0; 0x10000]);
                    mem.work_ram0[0x0100] = 0x17;
                    return mem;
                },
                assert_fn: |memory| {
                    let read_result = memory.read(0xE100);
                    assert_eq!(*read_result.unwrap(), 0x17);
                }
            },
            TestCase{
                description: String::from("reading from memory region 0xFE00 - 0xFF00, should access sprites attributes"),
                init_fn: || -> Memory {
                    let mut mem = Memory::new(vec![0x0; 0x10000]);
                    mem.sprite_attributes[0x0000] = 0x17;
                    return mem;
                },
                assert_fn: |memory| {
                    let read_result = memory.read(0xFE00);
                    assert_eq!(*read_result.unwrap(), 0x17);
                }
            },
            TestCase{
                description: String::from("reading from memory region 0xFF00 - 0xFF80, should access io registers"),
                init_fn: || -> Memory {
                    let mut mem = Memory::new(vec![0x0; 0x10000]);
                    mem.io_registers[0x0000] = 0x17;
                    return mem;
                },
                assert_fn: |memory| {
                    let read_result = memory.read(0xFF00);
                    assert_eq!(*read_result.unwrap(), 0x17);
                }
            },
            TestCase{
                description: String::from("reading from memory region 0xFF80 - 0xFFFF, should access high ram"),
                init_fn: || -> Memory {
                    let mut mem = Memory::new(vec![0x0; 0x10000]);
                    mem.hi_ram[0x7E] = 0x17;
                    return mem;
                },
                assert_fn: |memory| {
                    let read_result = memory.read(0xFFFE);
                    assert_eq!(*read_result.unwrap(), 0x17);
                }
            },
            TestCase{
                description: String::from("reading from memory region 0xFFFF, should access master interrupt enable register"),
                init_fn: || -> Memory {
                    let mut mem = Memory::new(vec![0x0; 0x10000]);
                    mem.interrupt_enable_register = 0x17;
                    return mem;
                },
                assert_fn: |memory| {
                    let read_result = memory.read(0xFFFF);
                    assert_eq!(*read_result.unwrap(), 0x17);
                }
            },
            TestCase{
                description: String::from("reading from memory region > 0xFFFF, should return none"),
                init_fn: || -> Memory {
                    Memory::new(vec![0x0; 0x10000])
                },
                assert_fn: |memory| {
                    let read_result = memory.read(0x10000);
                    assert!(read_result.is_none());
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
            init_fn: || -> Memory { Memory::new(vec![0x0; 0x10000]) },
            assert_fn: |mut memory| {
                memory.write(0x0000, 0x17);
                assert_eq!(*memory.read(0x0000).unwrap(), 0x31); //boot rom
            },
        },
        TestCase {
            description: String::from(
                "writing to region 0x8000 - 0xA000 -> should write to video ram",
            ),
            init_fn: || -> Memory { Memory::new(vec![0x0; 0x10000]) },
            assert_fn: |mut memory| {
                memory.write(0x8000, 0x17);
                assert_eq!(memory.video_ram[0x0000], 0x17);
            },
        },
        TestCase {
            description: String::from(
                "writing to region 0xA000 - 0xC000 -> should write to cartridge ram",
            ),
            init_fn: || -> Memory { Memory::new(vec![0x0; 0x10000]) },
            assert_fn: |mut memory| {
                memory.write(0xA000, 0x17);
                assert_eq!(*memory.read(0xA000).unwrap(), 0x17);
            },
        },
        TestCase {
            description: String::from(
                "writing to region 0xC000 - 0xD000 -> should write to work ram 0",
            ),
            init_fn: || -> Memory { Memory::new(vec![0x0; 0x10000]) },
            assert_fn: |mut memory| {
                memory.write(0xC000, 0x17);
                assert_eq!(memory.work_ram0[0x0000], 0x17);
            },
        },
        TestCase {
            description: String::from(
                "writing to region 0xD000 - 0xE000 -> should write to work ram 1",
            ),
            init_fn: || -> Memory { Memory::new(vec![0x0; 0x10000]) },
            assert_fn: |mut memory| {
                memory.write(0xD000, 0x17);
                assert_eq!(memory.work_ram1[0x0000], 0x17);
            },
        },
        TestCase {
            description: String::from(
                "writing to region 0xE000 - 0xFE00 -> should write to work ram 0 (echo ram)",
            ),
            init_fn: || -> Memory { Memory::new(vec![0x0; 0x10000]) },
            assert_fn: |mut memory| {
                memory.write(0xE000, 0x17);
                assert_eq!(memory.work_ram0[0x0000], 0x17);
            },
        },
        TestCase {
            description: String::from(
                "writing to region 0xFE00 - 0xFF00 -> should write to sprites attributes",
            ),
            init_fn: || -> Memory { Memory::new(vec![0x0; 0x10000]) },
            assert_fn: |mut memory| {
                memory.write(0xFE00, 0x17);
                assert_eq!(memory.sprite_attributes[0x0000], 0x17);
            },
        },
        TestCase {
            description: String::from(
                "writing to region 0xFF00 - 0xFF80 -> should write to io registers",
            ),
            init_fn: || -> Memory { Memory::new(vec![0x0; 0x10000]) },
            assert_fn: |mut memory| {
                memory.write(0xFF00, 0x17);
                assert_eq!(memory.io_registers[0x0000], 0x17);
            },
        },
        TestCase {
            description: String::from(
                "writing to region 0xFF80 - 0xFFFF -> should write to high ram",
            ),
            init_fn: || -> Memory { Memory::new(vec![0x0; 0x10000]) },
            assert_fn: |mut memory| {
                memory.write(0xFF80, 0x17);
                assert_eq!(memory.hi_ram[0x0000], 0x17);
            },
        },
        TestCase {
            description: String::from(
                "writing to region 0xFFFF -> should write to master interrupt enable register",
            ),
            init_fn: || -> Memory { Memory::new(vec![0x0; 0x10000]) },
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
fn boot_rom_enabled() {
    let mut mem = Memory::new(vec![0x0; 0x10000]);
    mem.io_registers[io_registers::BOOT_ROM_DISABLE_ADDR - 0xFF00] = 0x1;
    assert!(!mem.boot_rom_enabled());
    mem.io_registers[io_registers::BOOT_ROM_DISABLE_ADDR - 0xFF00] = 0x0;
    assert!(mem.boot_rom_enabled());
}
