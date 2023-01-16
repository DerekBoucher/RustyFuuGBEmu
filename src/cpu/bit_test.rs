use crate::cpu::bit;

#[test]
fn is_half_carry() {
    struct TestCase {
        byte: u8,
        added_byte: u8,
        carry: bool,
        expected_half_carry: bool,
    }

    let test_cases: Vec<TestCase> = vec![
        TestCase {
            byte: 0x00,
            added_byte: 0x0F,
            carry: true,
            expected_half_carry: true,
        },
        TestCase {
            byte: 0x00,
            added_byte: 0x0F,
            carry: false,
            expected_half_carry: false,
        },
        TestCase {
            byte: 0x01,
            added_byte: 0x0F,
            carry: false,
            expected_half_carry: true,
        },
        TestCase {
            byte: 0x01,
            added_byte: 0x1F,
            carry: false,
            expected_half_carry: true,
        },
        TestCase {
            byte: 0xFF,
            added_byte: 0x01,
            carry: false,
            expected_half_carry: true,
        },
        TestCase {
            byte: 0xFE,
            added_byte: 0x01,
            carry: true,
            expected_half_carry: true,
        },
    ];

    for tc in test_cases {
        let result = bit::is_half_carry(tc.byte, tc.added_byte, tc.carry);
        assert_eq!(tc.expected_half_carry, result);
    }
}

#[test]
fn is_half_borrow() {
    struct TestCase {
        byte: u8,
        subtracted_byte: u8,
        carry: bool,
        expected_half_carry: bool,
    }

    let test_cases: Vec<TestCase> = vec![
        TestCase {
            byte: 0x10,
            subtracted_byte: 0x01,
            carry: false,
            expected_half_carry: true,
        },
        TestCase {
            byte: 0x10,
            subtracted_byte: 0x00,
            carry: true,
            expected_half_carry: true,
        },
        TestCase {
            byte: 0x10,
            subtracted_byte: 0x00,
            carry: false,
            expected_half_carry: false,
        },
        TestCase {
            byte: 0x00,
            subtracted_byte: 0x1,
            carry: false,
            expected_half_carry: true,
        },
    ];

    for tc in test_cases {
        let result = bit::is_half_borrow(tc.byte, tc.subtracted_byte, tc.carry);
        assert_eq!(tc.expected_half_carry, result);
    }
}
