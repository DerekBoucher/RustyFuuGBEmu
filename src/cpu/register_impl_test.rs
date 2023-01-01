#[cfg(test)]
mod register {
    use crate::cpu::Register;

    #[test]
    fn word() {
        let register = Register { hi: 0x79, lo: 0x89 };
        assert_eq!(
            register.word(),
            0x7989,
            "should match the concatenated 8bit registers"
        )
    }
}
