#[cfg(test)]
mod rom_only {
    use crate::memory::cartridge;

    #[test]
    fn new() {
        let mut cart_data: Vec<u8> = vec![0x00; 0x8000];
        cart_data[cartridge::header::TYPE_ADDR] = cartridge::mbc_id::ROM_ONLY;

        let implementation = cartridge::new(cart_data);
        match implementation.as_any().downcast_ref::<cartridge::RomOnly>() {
            Some(cartridge::RomOnly { data: _cart_data }) => {}
            None => panic!("returned cartridge implementation was not a RomOnly!"),
        };
    }

    #[test]
    fn read() {
        let mut cart_data: Vec<u8> = vec![0x00; 0x8000];
        cart_data[cartridge::header::TYPE_ADDR] = cartridge::mbc_id::ROM_ONLY;
        cart_data[0x1000] = 0x7F;
        cart_data[0x7FFF] = 0x6F;

        let cartridge = cartridge::new(cart_data);
        let read1 = cartridge.read(0x1000);
        let read2 = cartridge.read(0x7FFF);
        let read3 = cartridge.read(0x10000);
        assert_eq!(
            *read1.unwrap(),
            0x7F,
            "should return the right data in rom bank 0"
        );
        assert_eq!(
            *read2.unwrap(),
            0x6F,
            "should return the right data in rom bank 1"
        );
        assert!(
            read3.is_none(),
            "should return None when reading out of bounds"
        )
    }

    #[test]
    fn write() {
        let mut cart_data: Vec<u8> = vec![0x00; 0x8000];
        cart_data[cartridge::header::TYPE_ADDR] = cartridge::mbc_id::ROM_ONLY;

        let mut cartridge = cartridge::new(cart_data);

        cartridge.write(0x1000, 0x7F);
        let read = cartridge.read(0x1000);
        assert_eq!(*read.unwrap(), 0x00, "should not have resulted in a write")
    }
}
