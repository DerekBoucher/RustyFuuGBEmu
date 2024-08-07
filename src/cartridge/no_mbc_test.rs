use crate::cartridge;
use crate::cartridge::NoMBC;

#[test]
fn read() {
    let mut cart_data: Vec<u8> = vec![0x00; 0x8000];
    cart_data[cartridge::header::TYPE_ADDR] = cartridge::mbc_id::ROM_ONLY;
    cart_data[0x1000] = 0x7F;
    cart_data[0x7FFF] = 0x6F;

    let mut rom_only = NoMBC::new(cart_data);
    rom_only.ram_bank[0x0] = 0x17;
    let read1 = rom_only.read(0x1000);
    let read2 = rom_only.read(0x7FFF);
    let read3 = rom_only.read(0x10000);
    let read4 = rom_only.read(0xA000);
    assert_eq!(read1.unwrap(), 0x7F,);
    assert_eq!(read2.unwrap(), 0x6F,);
    assert!(read3.is_none());
    assert_eq!(read4.unwrap(), 0x17);
}

#[test]
fn write() {
    let mut cart_data: Vec<u8> = vec![0x00; 0x8000];
    cart_data[cartridge::header::TYPE_ADDR] = cartridge::mbc_id::ROM_ONLY;

    let mut cartridge = NoMBC::new(cart_data);

    cartridge.write(0x1000, 0x7F);
    let read = cartridge.read(0x1000);
    assert_eq!(read.unwrap(), 0x00, "should not have resulted in a write")
}
