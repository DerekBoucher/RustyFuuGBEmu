#[cfg(test)]
mod register {
    use crate::cpu::cpu::Register;

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

#[cfg(test)]
mod lr35902 {
    use crate::cpu::cpu::CpuError;
    use crate::cpu::cpu::RegisterID;
    use crate::cpu::cpu::LR35902;

    #[test]
    fn ld_8_reg_to_reg() {
        struct TestCase {
            init_fn: fn() -> LR35902,
            dest: RegisterID,
            src: RegisterID,
            assert_fn: fn(LR35902),
            expected_err: Option<CpuError>,
        }

        let test_cases = vec![
            TestCase {
                init_fn: || -> LR35902 {
                    let mut cpu = LR35902::new();
                    cpu.bc.hi = 0x1;
                    return cpu;
                },
                dest: RegisterID::B,
                src: RegisterID::B,
                expected_err: None,
                assert_fn: |cpu| {
                    assert_eq!(cpu.bc.hi, cpu.bc.hi, "loading B into B");
                    assert_eq!(cpu.bc.word(), 0x0100, "loading B into B");
                },
            },
            TestCase {
                init_fn: || -> LR35902 {
                    let mut cpu = LR35902::new();
                    cpu.bc.hi = 0x1;
                    cpu.bc.lo = 0x7;
                    return cpu;
                },
                dest: RegisterID::B,
                src: RegisterID::C,
                expected_err: None,
                assert_fn: |cpu| {
                    assert_eq!(cpu.bc.hi, cpu.bc.lo, "loading C into B");
                    assert_eq!(cpu.bc.word(), 0x0707, "loading C into B");
                },
            },
            TestCase {
                init_fn: || -> LR35902 {
                    let mut cpu = LR35902::new();
                    cpu.bc.hi = 0x1;
                    cpu.de.hi = 0x7;
                    return cpu;
                },
                dest: RegisterID::B,
                src: RegisterID::D,
                expected_err: None,
                assert_fn: |cpu| {
                    assert_eq!(cpu.bc.hi, cpu.de.hi, "loading D into B");
                    assert_eq!(cpu.bc.word(), 0x0700, "loading D into B");
                },
            },
            TestCase {
                init_fn: || -> LR35902 {
                    let mut cpu = LR35902::new();
                    cpu.bc.hi = 0x1;
                    cpu.de.lo = 0x7;
                    return cpu;
                },
                dest: RegisterID::B,
                src: RegisterID::E,
                expected_err: None,
                assert_fn: |cpu| {
                    assert_eq!(cpu.bc.hi, cpu.de.lo, "loading E into B");
                    assert_eq!(cpu.bc.word(), 0x0700, "loading E into B");
                },
            },
            TestCase {
                init_fn: || -> LR35902 {
                    let mut cpu = LR35902::new();
                    cpu.bc.hi = 0x1;
                    cpu.hl.hi = 0x7;
                    return cpu;
                },
                dest: RegisterID::B,
                src: RegisterID::H,
                expected_err: None,
                assert_fn: |cpu| {
                    assert_eq!(cpu.bc.hi, cpu.hl.hi, "loading H into B");
                    assert_eq!(cpu.bc.word(), 0x0700, "loading H into B");
                },
            },
            TestCase {
                init_fn: || -> LR35902 {
                    let mut cpu = LR35902::new();
                    cpu.bc.hi = 0x1;
                    cpu.hl.lo = 0x7;
                    return cpu;
                },
                dest: RegisterID::B,
                src: RegisterID::L,
                expected_err: None,
                assert_fn: |cpu| {
                    assert_eq!(cpu.bc.hi, cpu.hl.lo, "loading L into B");
                    assert_eq!(cpu.bc.word(), 0x0700, "loading L into B");
                },
            },
            TestCase {
                init_fn: || -> LR35902 {
                    let mut cpu = LR35902::new();
                    cpu.bc.hi = 0x1;
                    cpu.af.hi = 0x7;
                    return cpu;
                },
                dest: RegisterID::B,
                src: RegisterID::A,
                expected_err: None,
                assert_fn: |cpu| {
                    assert_eq!(cpu.bc.hi, cpu.af.hi, "loading A into B");
                    assert_eq!(cpu.bc.word(), 0x0700, "loading A into B");
                },
            },
            TestCase {
                init_fn: || -> LR35902 {
                    let mut cpu = LR35902::new();
                    cpu.bc.lo = 0x1;
                    cpu.bc.hi = 0x7;
                    return cpu;
                },
                dest: RegisterID::C,
                src: RegisterID::B,
                expected_err: None,
                assert_fn: |cpu| {
                    assert_eq!(cpu.bc.lo, cpu.bc.hi, "loading B into C");
                    assert_eq!(cpu.bc.word(), 0x0707, "loading B into C");
                },
            },
            TestCase {
                init_fn: || -> LR35902 {
                    let mut cpu = LR35902::new();
                    cpu.bc.lo = 0x1;
                    return cpu;
                },
                dest: RegisterID::C,
                src: RegisterID::C,
                expected_err: None,
                assert_fn: |cpu| {
                    assert_eq!(cpu.bc.lo, cpu.bc.lo, "loading C into C");
                    assert_eq!(cpu.bc.word(), 0x0001, "loading C into C");
                },
            },
            TestCase {
                init_fn: || -> LR35902 {
                    let mut cpu = LR35902::new();
                    cpu.bc.lo = 0x1;
                    cpu.de.hi = 0x7;
                    return cpu;
                },
                dest: RegisterID::C,
                src: RegisterID::D,
                expected_err: None,
                assert_fn: |cpu| {
                    assert_eq!(cpu.bc.lo, cpu.de.hi, "loading D into C");
                    assert_eq!(cpu.bc.word(), 0x0007, "loading D into C");
                },
            },
            TestCase {
                init_fn: || -> LR35902 {
                    let mut cpu = LR35902::new();
                    cpu.bc.lo = 0x1;
                    cpu.de.lo = 0x7;
                    return cpu;
                },
                dest: RegisterID::C,
                src: RegisterID::E,
                expected_err: None,
                assert_fn: |cpu| {
                    assert_eq!(cpu.bc.lo, cpu.de.lo, "loading E into C");
                    assert_eq!(cpu.bc.word(), 0x0007, "loading E into C");
                },
            },
            TestCase {
                init_fn: || -> LR35902 {
                    let mut cpu = LR35902::new();
                    cpu.bc.lo = 0x1;
                    cpu.hl.hi = 0x7;
                    return cpu;
                },
                dest: RegisterID::C,
                src: RegisterID::H,
                expected_err: None,
                assert_fn: |cpu| {
                    assert_eq!(cpu.bc.lo, cpu.hl.hi, "loading H into C");
                    assert_eq!(cpu.bc.word(), 0x0007, "loading H into C");
                },
            },
            TestCase {
                init_fn: || -> LR35902 {
                    let mut cpu = LR35902::new();
                    cpu.bc.lo = 0x1;
                    cpu.hl.lo = 0x7;
                    return cpu;
                },
                dest: RegisterID::C,
                src: RegisterID::L,
                expected_err: None,
                assert_fn: |cpu| {
                    assert_eq!(cpu.bc.lo, cpu.hl.lo, "loading L into C");
                    assert_eq!(cpu.bc.word(), 0x0007, "loading L into C");
                },
            },
            TestCase {
                init_fn: || -> LR35902 {
                    let mut cpu = LR35902::new();
                    cpu.bc.lo = 0x1;
                    cpu.af.hi = 0x7;
                    return cpu;
                },
                dest: RegisterID::C,
                src: RegisterID::A,
                expected_err: None,
                assert_fn: |cpu| {
                    assert_eq!(cpu.bc.lo, cpu.af.hi, "loading A into C");
                    assert_eq!(cpu.bc.word(), 0x0007, "loading A into C");
                },
            },
            TestCase {
                init_fn: || -> LR35902 {
                    let mut cpu = LR35902::new();
                    cpu.de.hi = 0x1;
                    cpu.bc.hi = 0x7;
                    return cpu;
                },
                dest: RegisterID::D,
                src: RegisterID::B,
                expected_err: None,
                assert_fn: |cpu| {
                    assert_eq!(cpu.de.hi, cpu.bc.hi, "loading B into D");
                    assert_eq!(cpu.de.word(), 0x0700, "loading B into D");
                },
            },
            TestCase {
                init_fn: || -> LR35902 {
                    let mut cpu = LR35902::new();
                    cpu.de.hi = 0x1;
                    cpu.bc.lo = 0x7;
                    return cpu;
                },
                dest: RegisterID::D,
                src: RegisterID::C,
                expected_err: None,
                assert_fn: |cpu| {
                    assert_eq!(cpu.de.hi, cpu.bc.lo, "loading C into D");
                    assert_eq!(cpu.de.word(), 0x0700, "loading C into D");
                },
            },
            TestCase {
                init_fn: || -> LR35902 {
                    let mut cpu = LR35902::new();
                    cpu.de.hi = 0x1;
                    return cpu;
                },
                dest: RegisterID::D,
                src: RegisterID::D,
                expected_err: None,
                assert_fn: |cpu| {
                    assert_eq!(cpu.de.hi, cpu.de.hi, "loading D into D");
                    assert_eq!(cpu.de.word(), 0x0100, "loading D into D");
                },
            },
            TestCase {
                init_fn: || -> LR35902 {
                    let mut cpu = LR35902::new();
                    cpu.de.hi = 0x1;
                    cpu.de.lo = 0x7;
                    return cpu;
                },
                dest: RegisterID::D,
                src: RegisterID::E,
                expected_err: None,
                assert_fn: |cpu| {
                    assert_eq!(cpu.de.hi, cpu.de.lo, "loading E into D");
                    assert_eq!(cpu.de.word(), 0x0707, "loading E into D");
                },
            },
            TestCase {
                init_fn: || -> LR35902 {
                    let mut cpu = LR35902::new();
                    cpu.de.hi = 0x1;
                    cpu.hl.hi = 0x7;
                    return cpu;
                },
                dest: RegisterID::D,
                src: RegisterID::H,
                expected_err: None,
                assert_fn: |cpu| {
                    assert_eq!(cpu.de.hi, cpu.hl.hi, "loading H into D");
                    assert_eq!(cpu.de.word(), 0x0700, "loading H into D");
                },
            },
            TestCase {
                init_fn: || -> LR35902 {
                    let mut cpu = LR35902::new();
                    cpu.de.hi = 0x1;
                    cpu.hl.lo = 0x7;
                    return cpu;
                },
                dest: RegisterID::D,
                src: RegisterID::L,
                expected_err: None,
                assert_fn: |cpu| {
                    assert_eq!(cpu.de.hi, cpu.hl.lo, "loading L into D");
                    assert_eq!(cpu.de.word(), 0x0700, "loading L into D");
                },
            },
            TestCase {
                init_fn: || -> LR35902 {
                    let mut cpu = LR35902::new();
                    cpu.de.hi = 0x1;
                    cpu.af.hi = 0x7;
                    return cpu;
                },
                dest: RegisterID::D,
                src: RegisterID::A,
                expected_err: None,
                assert_fn: |cpu| {
                    assert_eq!(cpu.de.hi, cpu.af.hi, "loading A into D");
                    assert_eq!(cpu.de.word(), 0x0700, "loading A into D");
                },
            },
            TestCase {
                init_fn: || -> LR35902 {
                    let mut cpu = LR35902::new();
                    cpu.de.lo = 0x1;
                    cpu.bc.hi = 0x7;
                    return cpu;
                },
                dest: RegisterID::E,
                src: RegisterID::B,
                expected_err: None,
                assert_fn: |cpu| {
                    assert_eq!(cpu.de.lo, cpu.bc.hi, "loading B into E");
                    assert_eq!(cpu.de.word(), 0x0007, "loading B into E");
                },
            },
            TestCase {
                init_fn: || -> LR35902 {
                    let mut cpu = LR35902::new();
                    cpu.de.lo = 0x1;
                    cpu.bc.lo = 0x7;
                    return cpu;
                },
                dest: RegisterID::E,
                src: RegisterID::C,
                expected_err: None,
                assert_fn: |cpu| {
                    assert_eq!(cpu.de.lo, cpu.bc.lo, "loading C into E");
                    assert_eq!(cpu.de.word(), 0x0007, "loading C into E");
                },
            },
            TestCase {
                init_fn: || -> LR35902 {
                    let mut cpu = LR35902::new();
                    cpu.de.lo = 0x1;
                    cpu.de.hi = 0x7;
                    return cpu;
                },
                dest: RegisterID::E,
                src: RegisterID::D,
                expected_err: None,
                assert_fn: |cpu| {
                    assert_eq!(cpu.de.lo, cpu.de.hi, "loading D into E");
                    assert_eq!(cpu.de.word(), 0x0707, "loading D into E");
                },
            },
            TestCase {
                init_fn: || -> LR35902 {
                    let mut cpu = LR35902::new();
                    cpu.de.lo = 0x1;
                    return cpu;
                },
                dest: RegisterID::E,
                src: RegisterID::E,
                expected_err: None,
                assert_fn: |cpu| {
                    assert_eq!(cpu.de.lo, cpu.de.lo, "loading E into E");
                    assert_eq!(cpu.de.word(), 0x0001, "loading E into E");
                },
            },
            TestCase {
                init_fn: || -> LR35902 {
                    let mut cpu = LR35902::new();
                    cpu.de.lo = 0x1;
                    cpu.hl.hi = 0x7;
                    return cpu;
                },
                dest: RegisterID::E,
                src: RegisterID::H,
                expected_err: None,
                assert_fn: |cpu| {
                    assert_eq!(cpu.de.lo, cpu.hl.hi, "loading H into E");
                    assert_eq!(cpu.de.word(), 0x0007, "loading H into E");
                },
            },
            TestCase {
                init_fn: || -> LR35902 {
                    let mut cpu = LR35902::new();
                    cpu.de.lo = 0x1;
                    cpu.hl.lo = 0x7;
                    return cpu;
                },
                dest: RegisterID::E,
                src: RegisterID::L,
                expected_err: None,
                assert_fn: |cpu| {
                    assert_eq!(cpu.de.lo, cpu.hl.lo, "loading L into E");
                    assert_eq!(cpu.de.word(), 0x0007, "loading L into E");
                },
            },
            TestCase {
                init_fn: || -> LR35902 {
                    let mut cpu = LR35902::new();
                    cpu.de.lo = 0x1;
                    cpu.af.hi = 0x7;
                    return cpu;
                },
                dest: RegisterID::E,
                src: RegisterID::A,
                expected_err: None,
                assert_fn: |cpu| {
                    assert_eq!(cpu.de.lo, cpu.af.hi, "loading A into E");
                    assert_eq!(cpu.de.word(), 0x0007, "loading A into E");
                },
            },
            TestCase {
                init_fn: || -> LR35902 {
                    let mut cpu = LR35902::new();
                    cpu.hl.hi = 0x1;
                    cpu.bc.hi = 0x7;
                    return cpu;
                },
                dest: RegisterID::H,
                src: RegisterID::B,
                expected_err: None,
                assert_fn: |cpu| {
                    assert_eq!(cpu.hl.hi, cpu.bc.hi, "loading B into H");
                    assert_eq!(cpu.hl.word(), 0x0700, "loading B into H");
                },
            },
            TestCase {
                init_fn: || -> LR35902 {
                    let mut cpu = LR35902::new();
                    cpu.hl.hi = 0x1;
                    cpu.bc.lo = 0x7;
                    return cpu;
                },
                dest: RegisterID::H,
                src: RegisterID::C,
                expected_err: None,
                assert_fn: |cpu| {
                    assert_eq!(cpu.hl.hi, cpu.bc.lo, "loading C into H");
                    assert_eq!(cpu.hl.word(), 0x0700, "loading C into H");
                },
            },
            TestCase {
                init_fn: || -> LR35902 {
                    let mut cpu = LR35902::new();
                    cpu.hl.hi = 0x1;
                    cpu.de.hi = 0x7;
                    return cpu;
                },
                dest: RegisterID::H,
                src: RegisterID::D,
                expected_err: None,
                assert_fn: |cpu| {
                    assert_eq!(cpu.hl.hi, cpu.de.hi, "loading D into H");
                    assert_eq!(cpu.hl.word(), 0x0700, "loading D into H");
                },
            },
            TestCase {
                init_fn: || -> LR35902 {
                    let mut cpu = LR35902::new();
                    cpu.hl.hi = 0x1;
                    cpu.de.lo = 0x7;
                    return cpu;
                },
                dest: RegisterID::H,
                src: RegisterID::E,
                expected_err: None,
                assert_fn: |cpu| {
                    assert_eq!(cpu.hl.hi, cpu.de.lo, "loading E into H");
                    assert_eq!(cpu.hl.word(), 0x0700, "loading E into H");
                },
            },
            TestCase {
                init_fn: || -> LR35902 {
                    let mut cpu = LR35902::new();
                    cpu.hl.hi = 0x1;
                    return cpu;
                },
                dest: RegisterID::H,
                src: RegisterID::H,
                expected_err: None,
                assert_fn: |cpu| {
                    assert_eq!(cpu.hl.hi, cpu.hl.hi, "loading H into H");
                    assert_eq!(cpu.hl.word(), 0x0100, "loading H into H");
                },
            },
            TestCase {
                init_fn: || -> LR35902 {
                    let mut cpu = LR35902::new();
                    cpu.hl.hi = 0x1;
                    cpu.hl.lo = 0x7;
                    return cpu;
                },
                dest: RegisterID::H,
                src: RegisterID::L,
                expected_err: None,
                assert_fn: |cpu| {
                    assert_eq!(cpu.hl.hi, cpu.hl.lo, "loading L into H");
                    assert_eq!(cpu.hl.word(), 0x0707, "loading L into H");
                },
            },
            TestCase {
                init_fn: || -> LR35902 {
                    let mut cpu = LR35902::new();
                    cpu.hl.hi = 0x1;
                    cpu.af.hi = 0x7;
                    return cpu;
                },
                dest: RegisterID::H,
                src: RegisterID::A,
                expected_err: None,
                assert_fn: |cpu| {
                    assert_eq!(cpu.hl.hi, cpu.af.hi, "loading A into H");
                    assert_eq!(cpu.hl.word(), 0x0700, "loading A into H");
                },
            },
            TestCase {
                init_fn: || -> LR35902 {
                    let mut cpu = LR35902::new();
                    cpu.hl.lo = 0x1;
                    cpu.bc.hi = 0x7;
                    return cpu;
                },
                dest: RegisterID::L,
                src: RegisterID::B,
                expected_err: None,
                assert_fn: |cpu| {
                    assert_eq!(cpu.hl.lo, cpu.bc.hi, "loading B into L");
                    assert_eq!(cpu.hl.word(), 0x0007, "loading B into L");
                },
            },
            TestCase {
                init_fn: || -> LR35902 {
                    let mut cpu = LR35902::new();
                    cpu.hl.lo = 0x1;
                    cpu.bc.lo = 0x7;
                    return cpu;
                },
                dest: RegisterID::L,
                src: RegisterID::C,
                expected_err: None,
                assert_fn: |cpu| {
                    assert_eq!(cpu.hl.lo, cpu.bc.lo, "loading C into L");
                    assert_eq!(cpu.hl.word(), 0x0007, "loading C into L");
                },
            },
            TestCase {
                init_fn: || -> LR35902 {
                    let mut cpu = LR35902::new();
                    cpu.hl.lo = 0x1;
                    cpu.de.hi = 0x7;
                    return cpu;
                },
                dest: RegisterID::L,
                src: RegisterID::D,
                expected_err: None,
                assert_fn: |cpu| {
                    assert_eq!(cpu.hl.lo, cpu.de.hi, "loading D into L");
                    assert_eq!(cpu.hl.word(), 0x0007, "loading D into L");
                },
            },
            TestCase {
                init_fn: || -> LR35902 {
                    let mut cpu = LR35902::new();
                    cpu.hl.lo = 0x1;
                    cpu.de.lo = 0x7;
                    return cpu;
                },
                dest: RegisterID::L,
                src: RegisterID::E,
                expected_err: None,
                assert_fn: |cpu| {
                    assert_eq!(cpu.hl.lo, cpu.de.lo, "loading E into L");
                    assert_eq!(cpu.hl.word(), 0x0007, "loading E into L");
                },
            },
            TestCase {
                init_fn: || -> LR35902 {
                    let mut cpu = LR35902::new();
                    cpu.hl.lo = 0x1;
                    cpu.hl.hi = 0x7;
                    return cpu;
                },
                dest: RegisterID::L,
                src: RegisterID::H,
                expected_err: None,
                assert_fn: |cpu| {
                    assert_eq!(cpu.hl.lo, cpu.hl.hi, "loading H into L");
                    assert_eq!(cpu.hl.word(), 0x0707, "loading H into L");
                },
            },
            TestCase {
                init_fn: || -> LR35902 {
                    let mut cpu = LR35902::new();
                    cpu.hl.lo = 0x1;
                    return cpu;
                },
                dest: RegisterID::L,
                src: RegisterID::L,
                expected_err: None,
                assert_fn: |cpu| {
                    assert_eq!(cpu.hl.lo, cpu.hl.lo, "loading L into L");
                    assert_eq!(cpu.hl.word(), 0x0001, "loading L into L");
                },
            },
            TestCase {
                init_fn: || -> LR35902 {
                    let mut cpu = LR35902::new();
                    cpu.hl.lo = 0x1;
                    cpu.af.hi = 0x7;
                    return cpu;
                },
                dest: RegisterID::L,
                src: RegisterID::A,
                expected_err: None,
                assert_fn: |cpu| {
                    assert_eq!(cpu.hl.lo, cpu.af.hi, "loading A into L");
                    assert_eq!(cpu.hl.word(), 0x0007, "loading A into L");
                },
            },
            TestCase {
                init_fn: || -> LR35902 {
                    let mut cpu = LR35902::new();
                    cpu.af.hi = 0x1;
                    cpu.bc.hi = 0x7;
                    return cpu;
                },
                dest: RegisterID::A,
                src: RegisterID::B,
                expected_err: None,
                assert_fn: |cpu| {
                    assert_eq!(cpu.af.hi, cpu.bc.hi, "loading B into A");
                    assert_eq!(cpu.af.word(), 0x0700, "loading B into A");
                },
            },
            TestCase {
                init_fn: || -> LR35902 {
                    let mut cpu = LR35902::new();
                    cpu.af.hi = 0x1;
                    cpu.bc.lo = 0x7;
                    return cpu;
                },
                dest: RegisterID::A,
                src: RegisterID::C,
                expected_err: None,
                assert_fn: |cpu| {
                    assert_eq!(cpu.af.hi, cpu.bc.lo, "loading C into A");
                    assert_eq!(cpu.af.word(), 0x0700, "loading C into A");
                },
            },
            TestCase {
                init_fn: || -> LR35902 {
                    let mut cpu = LR35902::new();
                    cpu.af.hi = 0x1;
                    cpu.de.hi = 0x7;
                    return cpu;
                },
                dest: RegisterID::A,
                src: RegisterID::D,
                expected_err: None,
                assert_fn: |cpu| {
                    assert_eq!(cpu.af.hi, cpu.de.hi, "loading D into A");
                    assert_eq!(cpu.af.word(), 0x0700, "loading D into A");
                },
            },
            TestCase {
                init_fn: || -> LR35902 {
                    let mut cpu = LR35902::new();
                    cpu.af.hi = 0x1;
                    cpu.de.lo = 0x7;
                    return cpu;
                },
                dest: RegisterID::A,
                src: RegisterID::E,
                expected_err: None,
                assert_fn: |cpu| {
                    assert_eq!(cpu.af.hi, cpu.de.lo, "loading E into A");
                    assert_eq!(cpu.af.word(), 0x0700, "loading E into A");
                },
            },
            TestCase {
                init_fn: || -> LR35902 {
                    let mut cpu = LR35902::new();
                    cpu.af.hi = 0x1;
                    cpu.hl.hi = 0x7;
                    return cpu;
                },
                dest: RegisterID::A,
                src: RegisterID::H,
                expected_err: None,
                assert_fn: |cpu| {
                    assert_eq!(cpu.af.hi, cpu.hl.hi, "loading H into A");
                    assert_eq!(cpu.af.word(), 0x0700, "loading H into A");
                },
            },
            TestCase {
                init_fn: || -> LR35902 {
                    let mut cpu = LR35902::new();
                    cpu.af.hi = 0x1;
                    cpu.hl.lo = 0x7;
                    return cpu;
                },
                dest: RegisterID::A,
                src: RegisterID::L,
                expected_err: None,
                assert_fn: |cpu| {
                    assert_eq!(cpu.af.hi, cpu.hl.lo, "loading L into A");
                    assert_eq!(cpu.af.word(), 0x0700, "loading L into A");
                },
            },
            TestCase {
                init_fn: || -> LR35902 {
                    let mut cpu = LR35902::new();
                    cpu.af.hi = 0x1;
                    return cpu;
                },
                dest: RegisterID::A,
                src: RegisterID::A,
                expected_err: None,
                assert_fn: |cpu| {
                    assert_eq!(cpu.af.hi, cpu.af.hi, "loading A into A");
                    assert_eq!(cpu.af.word(), 0x0100, "loading A into A");
                },
            },
            TestCase {
                init_fn: || -> LR35902 { LR35902::new() },
                dest: RegisterID::F,
                src: RegisterID::A,
                expected_err: Some(CpuError::InvalidLoadOperands),
                assert_fn: |cpu| {},
            },
            TestCase {
                init_fn: || -> LR35902 { LR35902::new() },
                dest: RegisterID::SP,
                src: RegisterID::A,
                expected_err: Some(CpuError::InvalidLoadOperands),
                assert_fn: |cpu| {},
            },
            TestCase {
                init_fn: || -> LR35902 { LR35902::new() },
                dest: RegisterID::PC,
                src: RegisterID::A,
                expected_err: Some(CpuError::InvalidLoadOperands),
                assert_fn: |cpu| {},
            },
            TestCase {
                init_fn: || -> LR35902 { LR35902::new() },
                dest: RegisterID::A,
                src: RegisterID::F,
                expected_err: Some(CpuError::InvalidLoadOperands),
                assert_fn: |cpu| {},
            },
            TestCase {
                init_fn: || -> LR35902 { LR35902::new() },
                dest: RegisterID::A,
                src: RegisterID::SP,
                expected_err: Some(CpuError::InvalidLoadOperands),
                assert_fn: |cpu| {},
            },
            TestCase {
                init_fn: || -> LR35902 { LR35902::new() },
                dest: RegisterID::A,
                src: RegisterID::PC,
                expected_err: Some(CpuError::InvalidLoadOperands),
                assert_fn: |cpu| {},
            },
        ];

        for tc in test_cases {
            let mut cpu = (tc.init_fn)();
            match cpu.ld_8_reg_to_reg(tc.dest, tc.src) {
                Ok(()) => (tc.assert_fn)(cpu),
                Err(why) => {
                    assert_eq!(
                        why,
                        tc.expected_err.unwrap(),
                        "returned error should match expected"
                    )
                }
            }
        }
    }

    #[test]
    fn ld_8_imm_to_reg() {
        struct TestCase {
            cpu: LR35902,
            dest: RegisterID,
            val: u8,
            assert_fn: fn(LR35902),
            expected_err: Option<CpuError>,
        }

        let test_cases: Vec<TestCase> = vec![
            TestCase {
                cpu: LR35902::new(),
                dest: RegisterID::A,
                val: 0x10,
                assert_fn: |cpu| assert_eq!(cpu.af.hi, 0x10, "loading immediate 8bit value into A"),
                expected_err: None,
            },
            TestCase {
                cpu: LR35902::new(),
                dest: RegisterID::B,
                val: 0x10,
                assert_fn: |cpu| assert_eq!(cpu.bc.hi, 0x10, "loading immediate 8bit value into B"),
                expected_err: None,
            },
            TestCase {
                cpu: LR35902::new(),
                dest: RegisterID::C,
                val: 0x10,
                assert_fn: |cpu| assert_eq!(cpu.bc.lo, 0x10, "loading immediate 8bit value into C"),
                expected_err: None,
            },
            TestCase {
                cpu: LR35902::new(),
                dest: RegisterID::D,
                val: 0x10,
                assert_fn: |cpu| assert_eq!(cpu.de.hi, 0x10, "loading immediate 8bit value into D"),
                expected_err: None,
            },
            TestCase {
                cpu: LR35902::new(),
                dest: RegisterID::E,
                val: 0x10,
                assert_fn: |cpu| assert_eq!(cpu.de.lo, 0x10, "loading immediate 8bit value into E"),
                expected_err: None,
            },
            TestCase {
                cpu: LR35902::new(),
                dest: RegisterID::H,
                val: 0x10,
                assert_fn: |cpu| assert_eq!(cpu.hl.hi, 0x10, "loading immediate 8bit value into H"),
                expected_err: None,
            },
            TestCase {
                cpu: LR35902::new(),
                dest: RegisterID::L,
                val: 0x10,
                assert_fn: |cpu| assert_eq!(cpu.hl.lo, 0x10, "loading immediate 8bit value into L"),
                expected_err: None,
            },
            TestCase {
                cpu: LR35902::new(),
                dest: RegisterID::F,
                val: 0x10,
                assert_fn: |cpu| {},
                expected_err: Some(CpuError::InvalidLoadOperands),
            },
            TestCase {
                cpu: LR35902::new(),
                dest: RegisterID::SP,
                val: 0x10,
                assert_fn: |cpu| {},
                expected_err: Some(CpuError::InvalidLoadOperands),
            },
            TestCase {
                cpu: LR35902::new(),
                dest: RegisterID::PC,
                val: 0x10,
                assert_fn: |cpu| {},
                expected_err: Some(CpuError::InvalidLoadOperands),
            },
        ];

        for tc in test_cases {
            let mut cpu = tc.cpu;
            match cpu.ld_8_imm_to_reg(tc.dest, tc.val) {
                Ok(()) => (tc.assert_fn)(cpu),
                Err(why) => {
                    assert_eq!(
                        why,
                        tc.expected_err.unwrap(),
                        "returned error should match expected"
                    )
                }
            }
        }
    }
}
