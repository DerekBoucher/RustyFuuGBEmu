use crate::cartridge;
use crate::cartridge::mbc1::MBC1;
use crate::interface::Cartridge;

#[test]
fn new() {
    struct TestCase {
        init_fn: fn() -> Vec<u8>,
    }

    let test_cases: Vec<TestCase> = vec![
        TestCase {
            init_fn: || -> Vec<u8> {
                let mut cart_data: Vec<u8> = vec![0x00; 0x8000];
                cart_data[cartridge::header::TYPE_ADDR] = cartridge::mbc_id::MBC1;
                return cart_data;
            },
        },
        TestCase {
            init_fn: || -> Vec<u8> {
                let mut cart_data: Vec<u8> = vec![0x00; 0x8000];
                cart_data[cartridge::header::TYPE_ADDR] = cartridge::mbc_id::MBC1_RAM;
                return cart_data;
            },
        },
        TestCase {
            init_fn: || -> Vec<u8> {
                let mut cart_data: Vec<u8> = vec![0x00; 0x8000];
                cart_data[cartridge::header::TYPE_ADDR] = cartridge::mbc_id::MBC1_RAM_BATTERY;
                return cart_data;
            },
        },
    ];

    for tc in test_cases {
        let implementation = cartridge::new((tc.init_fn)());
        match implementation.as_any().downcast_ref::<MBC1>() {
            Some(_) => {}
            None => panic!("returned cartridge implementation was not a MBC1!"),
        };
    }
}

#[test]
fn read() {
    struct TestCase {
        description: String,
        run_fn: fn(),
    }

    let test_cases: Vec<TestCase> = vec![
            TestCase {
                description: String::from("read from bank 0 w/ no bank switching"),
                run_fn: || {
                    let mut cart_data: Vec<u8> = vec![0x0; 0x4000];
                    cart_data[0x0] = 0x7F;
                    let mbc1 = MBC1::new(cart_data);
                    let read_result = mbc1.read(0x0);
                    assert_eq!(
                        read_result.unwrap(),
                        0x7F,
                    )
                },
            },
            TestCase {
                description: String::from(
                    "read from bank 0 w/ bank switching but ram bank register is 0",
                ),
                run_fn: || {
                    let mut cart_data: Vec<u8> = vec![0x0; 0x4000];
                    cart_data[0x0] = 0x7F;
                    cart_data[cartridge::header::ROM_SIZE_ADDR] =
                        cartridge::rom_size_id::ONE_MEGABYTE;
                    let mut mbc1 = MBC1::new(cart_data);
                    mbc1.banking_mode = MBC1::RAM_BANKING_MODE;
                    let read_result = mbc1.read(0x0);
                    assert_eq!(
                        read_result.unwrap(),
                        0x7F,
                    )
                },
            },
            TestCase {
                description: String::from(
                    "read from bank 0x10 w/ bank switching when ram bank register is 1",
                ),
                run_fn: || {
                    let mut cart_data: Vec<u8> = vec![0x0; 0x11 * 0x4000];
                    cart_data[0x10 * 0x4000] = 0x7F;
                    cart_data[cartridge::header::ROM_SIZE_ADDR] =
                        cartridge::rom_size_id::ONE_MEGABYTE;
                    let mut mbc1 = MBC1::new(cart_data);
                    mbc1.banking_mode = MBC1::RAM_BANKING_MODE;
                    mbc1.ram_bank_select_register = 0x1;
                    let read_result = mbc1.read(0x0);
                    assert_eq!(
                        read_result.unwrap(),
                        0x7F,
                    )
                },
            },
            TestCase {
                description: String::from(
                    "read from bank 0x20 w/ bank switching when ram bank register is 2",
                ),
                run_fn: || {
                    let mut cart_data: Vec<u8> = vec![0x0; 0x21 * 0x4000];
                    cart_data[0x20 * 0x4000] = 0x7F;
                    cart_data[cartridge::header::ROM_SIZE_ADDR] =
                        cartridge::rom_size_id::ONE_MEGABYTE;
                    let mut mbc1 = MBC1::new(cart_data);
                    mbc1.banking_mode = MBC1::RAM_BANKING_MODE;
                    mbc1.ram_bank_select_register = 0x2;
                    let read_result = mbc1.read(0x0);
                    assert_eq!(
                        read_result.unwrap(),
                        0x7F,
                    )
                },
            },
            TestCase {
                description: String::from(
                    "read from bank 0x30 w/ bank switching when ram bank register is 3",
                ),
                run_fn: || {
                    let mut cart_data: Vec<u8> = vec![0x0; 0x31 * 0x4000];
                    cart_data[0x30 * 0x4000] = 0x7F;
                    cart_data[cartridge::header::ROM_SIZE_ADDR] =
                        cartridge::rom_size_id::ONE_MEGABYTE;
                    let mut mbc1 = MBC1::new(cart_data);
                    mbc1.banking_mode = MBC1::RAM_BANKING_MODE;
                    mbc1.ram_bank_select_register = 0x3;
                    let read_result = mbc1.read(0x0);
                    assert_eq!(
                        read_result.unwrap(),
                        0x7F,
                    )
                },
            },
            TestCase {
                description: String::from(
                    "read from bank 0x00 w/ bank switching when ram bank register is greater than 4 (bit masking)",
                ),
                run_fn: || {
                    let mut cart_data: Vec<u8> = vec![0x0; 0x1 * 0x4000];
                    cart_data[0x0000] = 0x7F;
                    cart_data[cartridge::header::ROM_SIZE_ADDR] =
                        cartridge::rom_size_id::ONE_MEGABYTE;
                    let mut mbc1 = MBC1::new(cart_data);
                    mbc1.banking_mode = MBC1::RAM_BANKING_MODE;
                    mbc1.ram_bank_select_register = 0x4;
                    let read_result = mbc1.read(0x0);
                    assert_eq!(
                        read_result.unwrap(),
                        0x7F,
                    )
                },
            },
            TestCase {
                description: String::from(
                    "read rom region 0x4000 - 0x8000 w/ rom select register 0 -> should still access bank 1, not bank 0",
                ),
                run_fn: || {
                    let mut cart_data: Vec<u8> = vec![0x0; 0x2 * 0x4000];
                    cart_data[0x4000] = 0x7F;
                    let mut mbc1 = MBC1::new(cart_data);
                    mbc1.rom_bank_select_register = 0x0;
                    let read_result = mbc1.read(0x4000);
                    assert_eq!(
                        read_result.unwrap(),
                        0x7F,
                    )
                },
            },
            TestCase {
                description: String::from(
                    "read rom region 0x4000 - 0x8000 w/ rom select register 1 -> should select data in bank 1",
                ),
                run_fn: || {
                    let mut cart_data: Vec<u8> = vec![0x0; 0x2 * 0x4000];
                    cart_data[0x4000] = 0x7F;
                    let mut mbc1 = MBC1::new(cart_data);
                    mbc1.rom_bank_select_register = 0x1;
                    let read_result = mbc1.read(0x4000);
                    assert_eq!(
                        read_result.unwrap(),
                        0x7F,
                    )
                },
            },
            TestCase {
                description: String::from(
                    "read rom region 0x4000 - 0x8000 w/ rom select register 3 -> should select data in bank 3",
                ),
                run_fn: || {
                    let mut cart_data: Vec<u8> = vec![0x0; 0x4 * 0x4000];
                    cart_data[0x4000 * 0x3] = 0x7F;
                    let mut mbc1 = MBC1::new(cart_data);
                    mbc1.rom_bank_select_register = 0x3;
                    let read_result = mbc1.read(0x4000);
                    assert_eq!(
                        read_result.unwrap(),
                        0x7F,
                    )
                },
            },
            TestCase {
                description: String::from(
                    "read rom region 0x4000 - 0x8000 w/ rom select register 3 + ram banking mode -> should still only select bank 3",
                ),
                run_fn: || {
                    let mut cart_data: Vec<u8> = vec![0x0; 0x4 * 0x4000];
                    cart_data[0x4000 * 0x3] = 0x7F;
                    let mut mbc1 = MBC1::new(cart_data);
                    mbc1.rom_bank_select_register = 0x3;
                    mbc1.ram_bank_select_register = 0x1;
                    mbc1.banking_mode = MBC1::RAM_BANKING_MODE;
                    let read_result = mbc1.read(0x4000);
                    assert_eq!(
                        read_result.unwrap(),
                        0x7F,
                    )
                },
            },
            TestCase {
                description: String::from(
                    "read rom region 0x4000 - 0x8000 w/ rom select register 3 + ram banking mode + 1MB cart -> should select bank 0x23",
                ),
                run_fn: || {
                    let mut cart_data: Vec<u8> = vec![0x0; 0x24 * 0x4000];
                    cart_data[cartridge::header::ROM_SIZE_ADDR] =
                        cartridge::rom_size_id::ONE_MEGABYTE;
                    cart_data[0x4000 * 0x23] = 0x7F;
                    let mut mbc1 = MBC1::new(cart_data);
                    mbc1.rom_bank_select_register = 0x3;
                    mbc1.ram_bank_select_register = 0x1;
                    mbc1.banking_mode = MBC1::RAM_BANKING_MODE;
                    let read_result = mbc1.read(0x4000);
                    assert_eq!(
                        read_result.unwrap(),
                        0x7F,
                    )
                },
            },
            TestCase {
                description: String::from(
                    "read rom region 0x4000 - 0x8000 w/ rom select register 0 + ram banking mode + ram select register 3 + 1MB cart -> should select bank 0x61",
                ),
                run_fn: || {
                    let mut cart_data: Vec<u8> = vec![0x0; 0x62 * 0x4000];
                    cart_data[cartridge::header::ROM_SIZE_ADDR] =
                        cartridge::rom_size_id::ONE_MEGABYTE;
                    cart_data[0x4000 * 0x61] = 0x7F;
                    let mut mbc1 = MBC1::new(cart_data);
                    mbc1.rom_bank_select_register = 0x0;
                    mbc1.ram_bank_select_register = 0x3;
                    mbc1.banking_mode = MBC1::RAM_BANKING_MODE;
                    let read_result = mbc1.read(0x4000);
                    assert_eq!(
                        read_result.unwrap(),
                        0x7F,
                    )
                },
            },
            TestCase {
                description: String::from(
                    "read ram region 0xA000-0xBFFF -> cart has no ram -> should return 0xFF",
                ),
                run_fn: || {
                    let mut cart_data: Vec<u8> = vec![0x0; 0x1 * 0x4000];
                    cart_data[cartridge::header::RAM_SIZE_ADDR] =
                        cartridge::ram_size_id::NO_RAM;
                    let mut mbc1 = MBC1::new(cart_data);
                    mbc1.ram_bank_select_register = 0x0;
                    mbc1.ram_banks[0][0] = 0x7F;
                    let read_result = mbc1.read(0xA000);
                    assert_eq!(
                        read_result.unwrap(),
                        0xFF,
                    )
                },
            },
            TestCase {
                description: String::from(
                    "read ram region 0xA000-0xBFFF + cart type has ram + but type is not MBC1 w/ RAM -> should return 0xFF",
                ),
                run_fn: || {
                    let mut cart_data: Vec<u8> = vec![0x0; 0x1 * 0x4000];
                    cart_data[cartridge::header::RAM_SIZE_ADDR] =
                        cartridge::ram_size_id::_ONE_BANK;
                    cart_data[cartridge::header::TYPE_ADDR] = cartridge::mbc_id::MBC1;
                    let mut mbc1 = MBC1::new(cart_data);
                    mbc1.ram_bank_select_register = 0x0;
                    mbc1.ram_banks[0][0] = 0x7F;
                    let read_result = mbc1.read(0xA000);
                    assert_eq!(
                        read_result.unwrap(),
                        0xFF,
                    )
                },
            },
            TestCase {
                description: String::from(
                    "read ram region 0xA000-0xBFFF + ram select register is 0 + cart type has ram + cart is MBC1_RAM + ram is disabled -> should return 0xFF",
                ),
                run_fn: || {
                    let mut cart_data: Vec<u8> = vec![0x0; 0x1 * 0x4000];
                    cart_data[cartridge::header::RAM_SIZE_ADDR] =
                        cartridge::ram_size_id::_ONE_BANK;
                    cart_data[cartridge::header::TYPE_ADDR] = cartridge::mbc_id::MBC1_RAM;
                    let mut mbc1 = MBC1::new(cart_data);
                    mbc1.ram_bank_select_register = 0x0;
                    mbc1.ram_banks[0][0] = 0x7F;
                    let read_result = mbc1.read(0xA000);
                    assert_eq!(
                        read_result.unwrap(),
                        0xFF,
                    )
                },
            },
            TestCase {
                description: String::from(
                    "read ram region 0xA000-0xBFFF + ram select register is 0 + cart type has ram + cart is MBC1_RAM + ram is enabled -> should return value in ram bank 0",
                ),
                run_fn: || {
                    let mut cart_data: Vec<u8> = vec![0x0; 0x1 * 0x4000];
                    cart_data[cartridge::header::RAM_SIZE_ADDR] =
                        cartridge::ram_size_id::_ONE_BANK;
                    cart_data[cartridge::header::TYPE_ADDR] = cartridge::mbc_id::MBC1_RAM;
                    let mut mbc1 = MBC1::new(cart_data);
                    mbc1.ram_bank_select_register = 0x0;
                    mbc1.ram_banks[0][0] = 0x7F;
                    mbc1.ram_enabled = true;
                    let read_result = mbc1.read(0xA000);
                    assert_eq!(
                        read_result.unwrap(),
                        0x7F,
                    )
                },
            },
            TestCase {
                description: String::from(
                    "read ram region 0xA000-0xBFFF + ram select register is 1 + cart type has ram + cart is MBC1_RAM + ram is enabled + cart has more than 4 banks -> should return value in ram bank 1",
                ),
                run_fn: || {
                    let mut cart_data: Vec<u8> = vec![0x0; 0x1 * 0x4000];
                    cart_data[cartridge::header::RAM_SIZE_ADDR] =
                        cartridge::ram_size_id::FOUR_BANKS;
                    cart_data[cartridge::header::TYPE_ADDR] = cartridge::mbc_id::MBC1_RAM;
                    let mut mbc1 = MBC1::new(cart_data);
                    mbc1.ram_bank_select_register = 0x1;
                    mbc1.ram_banks[1][0] = 0x7F;
                    mbc1.ram_enabled = true;
                    let read_result = mbc1.read(0xA000);
                    assert_eq!(
                        read_result.unwrap(),
                        0x7F,
                    )
                },
            },
            TestCase {
                description: String::from(
                    "read ram region 0xA000-0xBFFF + ram select register is 4 + cart type has ram + cart is MBC1_RAM + ram is enabled + cart has more than 4 banks -> should return value in ram bank 0 (because of mask)",
                ),
                run_fn: || {
                    let mut cart_data: Vec<u8> = vec![0x0; 0x1 * 0x4000];
                    cart_data[cartridge::header::RAM_SIZE_ADDR] =
                        cartridge::ram_size_id::FOUR_BANKS;
                    cart_data[cartridge::header::TYPE_ADDR] = cartridge::mbc_id::MBC1_RAM;
                    let mut mbc1 = MBC1::new(cart_data);
                    mbc1.ram_bank_select_register = 0x4;
                    mbc1.ram_banks[0][0] = 0x7F;
                    mbc1.ram_enabled = true;
                    let read_result = mbc1.read(0xA000);
                    assert_eq!(
                        read_result.unwrap(),
                        0x7F,
                    )
                },
            },
            TestCase {
                description: String::from(
                    "read from undefined region",
                ),
                run_fn: || {
                    let cart_data: Vec<u8> = vec![0x0; 0x1 * 0x4000];
                    let mbc1 = MBC1::new(cart_data);
                    let read_result = mbc1.read(0x8000);
                    assert!(read_result.is_none())
                },
            },
        ];

    for tc in test_cases {
        println!("{}", tc.description);
        (tc.run_fn)();
    }
}

#[test]
fn write() {
    struct TestCase {
        description: String,
        run_fn: fn(),
    }

    let test_cases: Vec<TestCase> = vec![
            TestCase {
                description: String::from(
                    "writing to 0x0000 - 0x2000 -> non 0x-A value in the lower nibble -> disables ram",
                ),
                run_fn: || {
                    let cart_data: Vec<u8> = vec![0x0; 0x1 * 0x4000];
                    let mut mbc1 = MBC1::new(cart_data);
                    mbc1.ram_enabled = true;
                    mbc1.write(0x0000, 0x13);
                    assert!(!mbc1.ram_enabled);
                },
            },
            TestCase {
                description: String::from(
                    "writing to 0x0000 - 0x2000 -> 0x5A value (lower nibble == 0xA) -> enables ram",
                ),
                run_fn: || {
                    let cart_data: Vec<u8> = vec![0x0; 0x1 * 0x4000];
                    let mut mbc1 = MBC1::new(cart_data);
                    mbc1.write(0x0000, 0x5A);
                    assert!(mbc1.ram_enabled);
                },
            },
            TestCase {
                description: String::from(
                    "writing to 0x2000 - 0x4000 -> value is 0xFF -> cart only has 1 bank -> should set register to 1",
                ),
                run_fn: || {
                    let mut cart_data: Vec<u8> = vec![0x0; 0x1 * 0x4000];
                    cart_data[cartridge::header::ROM_SIZE_ADDR] = 0x00;
                    let mut mbc1 = MBC1::new(cart_data);
                    mbc1.write(0x2000, 0xFF);
                    assert_eq!(mbc1.rom_bank_select_register, 0x01);
                },
            },
            TestCase {
                description: String::from(
                    "writing to 0x2000 - 0x4000 -> value is 0xFF -> cart only has 4 banks -> should set register to 3",
                ),
                run_fn: || {
                    let mut cart_data: Vec<u8> = vec![0x0; 0x1 * 0x4000];
                    cart_data[cartridge::header::ROM_SIZE_ADDR] = 0x01;
                    let mut mbc1 = MBC1::new(cart_data);
                    mbc1.write(0x2000, 0xFF);
                    assert_eq!(mbc1.rom_bank_select_register, 0x03);
                },
            },
            TestCase {
                description: String::from(
                    "writing to 0x2000 - 0x4000 -> value is 0xFF -> cart only has 8 banks -> should set register to 7",
                ),
                run_fn: || {
                    let mut cart_data: Vec<u8> = vec![0x0; 0x1 * 0x4000];
                    cart_data[cartridge::header::ROM_SIZE_ADDR] = 0x02;
                    let mut mbc1 = MBC1::new(cart_data);
                    mbc1.write(0x2000, 0xFF);
                    assert_eq!(mbc1.rom_bank_select_register, 0x07);
                },
            },
            TestCase {
                description: String::from(
                    "writing to 0x2000 - 0x4000 -> value is 0xFF -> cart only has 16 banks -> should set register to 15",
                ),
                run_fn: || {
                    let mut cart_data: Vec<u8> = vec![0x0; 0x1 * 0x4000];
                    cart_data[cartridge::header::ROM_SIZE_ADDR] = 0x03;
                    let mut mbc1 = MBC1::new(cart_data);
                    mbc1.write(0x2000, 0xFF);
                    assert_eq!(mbc1.rom_bank_select_register, 0x0F);
                },
            },
            TestCase {
                description: String::from(
                    "writing to 0x2000 - 0x4000 -> value is 0xFF -> cart only has 32 banks -> should set register to 31",
                ),
                run_fn: || {
                    let mut cart_data: Vec<u8> = vec![0x0; 0x1 * 0x4000];
                    cart_data[cartridge::header::ROM_SIZE_ADDR] = 0x04;
                    let mut mbc1 = MBC1::new(cart_data);
                    mbc1.write(0x2000, 0xFF);
                    assert_eq!(mbc1.rom_bank_select_register, 0x1F);
                },
            },
            TestCase {
                description: String::from(
                    "writing to 0x4000 - 0x6000 -> value is 0x1 -> should set ram select register to 1",
                ),
                run_fn: || {
                    let cart_data: Vec<u8> = vec![0x0; 0x1 * 0x4000];
                    let mut mbc1 = MBC1::new(cart_data);
                    mbc1.write(0x4000, 0x1);
                    assert_eq!(mbc1.ram_bank_select_register, 0x1);
                },
            },
            TestCase {
                description: String::from(
                    "writing to 0x4000 - 0x6000 -> value is 0xFF -> should set ram select register to 3",
                ),
                run_fn: || {
                    let cart_data: Vec<u8> = vec![0x0; 0x1 * 0x4000];
                    let mut mbc1 = MBC1::new(cart_data);
                    mbc1.write(0x4000, 0xFF);
                    assert_eq!(mbc1.ram_bank_select_register, 0x3);
                },
            },
            TestCase {
                description: String::from(
                    "writing to 0x6000 - 0x8000 -> value is 0x1 -> should set banking mode to ram banking mode",
                ),
                run_fn: || {
                    let cart_data: Vec<u8> = vec![0x0; 0x1 * 0x4000];
                    let mut mbc1 = MBC1::new(cart_data);
                    mbc1.write(0x6000, 0x1);
                    assert_eq!(mbc1.banking_mode, MBC1::RAM_BANKING_MODE);
                },
            },
            TestCase {
                description: String::from(
                    "writing to 0x6000 - 0x8000 -> value is 0x10 -> should set banking mode to simple banking mode",
                ),
                run_fn: || {
                    let cart_data: Vec<u8> = vec![0x0; 0x1 * 0x4000];
                    let mut mbc1 = MBC1::new(cart_data);
                    mbc1.banking_mode = MBC1::RAM_BANKING_MODE;
                    mbc1.write(0x6000, 0x10);
                    assert_eq!(mbc1.banking_mode, MBC1::SIMPLE_BANKING_MODE);
                },
            },
            TestCase {
                description: String::from(
                    "writing to 0xA000 - 0xC000 (ram) + ram enabled + cart does not support ram -> should do nothing",
                ),
                run_fn: || {
                    let mut cart_data: Vec<u8> = vec![0x00; 0x4000];
                    cart_data[cartridge::header::TYPE_ADDR] = cartridge::mbc_id::MBC1;
                    let mut mbc1 = MBC1::new(cart_data);
                    mbc1.write(0xB000, 0x10);
                    assert_eq!(mbc1.ram_banks[0][0x1000], 0x00);
                },
            },
            TestCase {
                description: String::from(
                    "writing to 0xA000 - 0xC000 (ram) + ram disabled + cart has ram -> should do nothing",
                ),
                run_fn: || {
                    let mut cart_data: Vec<u8> = vec![0x00; 0x4000];
                    cart_data[cartridge::header::TYPE_ADDR] = cartridge::mbc_id::MBC1_RAM;
                    let mut mbc1 = MBC1::new(cart_data);
                    mbc1.write(0xB000, 0x10);
                    assert_eq!(mbc1.ram_banks[0][0x1000], 0x00);
                },
            },
            TestCase {
                description: String::from(
                    "writing to 0xA000 - 0xC000 (ram) + ram enabled + cart has ram -> should write to ram bank 0",
                ),
                run_fn: || {
                    let mut cart_data: Vec<u8> = vec![0x00; 0x4000];
                    cart_data[cartridge::header::RAM_SIZE_ADDR] = cartridge::ram_size_id::_ONE_BANK;
                    cart_data[cartridge::header::TYPE_ADDR] = cartridge::mbc_id::MBC1_RAM;
                    let mut mbc1 = MBC1::new(cart_data);
                    mbc1.ram_enabled = true;
                    mbc1.write(0xB000, 0x10);
                    assert_eq!(mbc1.ram_banks[0][0x1000], 0x10);
                },
            },
            TestCase {
                description: String::from(
                    "writing to 0xA000 - 0xC000 (ram) + ram enabled + cart has ram -> should write to ram bank 1",
                ),
                run_fn: || {
                    let mut cart_data: Vec<u8> = vec![0x00; 0x4000];
                    cart_data[cartridge::header::RAM_SIZE_ADDR] = cartridge::ram_size_id::FOUR_BANKS;
                    cart_data[cartridge::header::TYPE_ADDR] = cartridge::mbc_id::MBC1_RAM;
                    let mut mbc1 = MBC1::new(cart_data);
                    mbc1.ram_enabled = true;
                    mbc1.ram_bank_select_register = 0x01;
                    mbc1.write(0xB000, 0x10);
                    assert_eq!(mbc1.ram_banks[1][0x1000], 0x10);
                },
            },
            TestCase {
                description: String::from(
                    "writing to 0xA000 - 0xC000 (ram) + ram enabled + cart has ram -> should write to ram bank 2",
                ),
                run_fn: || {
                    let mut cart_data: Vec<u8> = vec![0x00; 0x4000];
                    cart_data[cartridge::header::RAM_SIZE_ADDR] = cartridge::ram_size_id::FOUR_BANKS;
                    cart_data[cartridge::header::TYPE_ADDR] = cartridge::mbc_id::MBC1_RAM;
                    let mut mbc1 = MBC1::new(cart_data);
                    mbc1.ram_enabled = true;
                    mbc1.ram_bank_select_register = 0x02;
                    mbc1.write(0xB000, 0x10);
                    assert_eq!(mbc1.ram_banks[2][0x1000], 0x10);
                },
            },
            TestCase {
                description: String::from(
                    "writing to 0xA000 - 0xC000 (ram) + ram enabled + cart has ram -> should write to ram bank 3",
                ),
                run_fn: || {
                    let mut cart_data: Vec<u8> = vec![0x00; 0x4000];
                    cart_data[cartridge::header::RAM_SIZE_ADDR] = cartridge::ram_size_id::FOUR_BANKS;
                    cart_data[cartridge::header::TYPE_ADDR] = cartridge::mbc_id::MBC1_RAM;
                    let mut mbc1 = MBC1::new(cart_data);
                    mbc1.ram_enabled = true;
                    mbc1.ram_bank_select_register = 0x03;
                    mbc1.write(0xB000, 0x10);
                    assert_eq!(mbc1.ram_banks[3][0x1000], 0x10);
                },
            },
            TestCase {
                description: String::from(
                    "writing to 0xA000 - 0xC000 (ram) + ram enabled + cart has ram -> should write to ram bank 0",
                ),
                run_fn: || {
                    let mut cart_data: Vec<u8> = vec![0x00; 0x4000];
                    cart_data[cartridge::header::RAM_SIZE_ADDR] = cartridge::ram_size_id::FOUR_BANKS;
                    cart_data[cartridge::header::TYPE_ADDR] = cartridge::mbc_id::MBC1_RAM;
                    let mut mbc1 = MBC1::new(cart_data);
                    mbc1.ram_enabled = true;
                    mbc1.ram_bank_select_register = 0x04;
                    mbc1.write(0xB000, 0x10);
                    assert_eq!(mbc1.ram_banks[0][0x1000], 0x10);
                },
            },
        ];

    for tc in test_cases {
        println!("{}", tc.description);
        (tc.run_fn)();
    }
}
