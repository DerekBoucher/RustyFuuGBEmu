use std::vec;

use crate::cpu::bit;

#[test]
fn is_carry() {
    struct TestCase {
        byte: u8,
        added_byte: u8,
        carry: bool,
        expected_carry: bool,
    }

    let test_cases: Vec<TestCase> = vec![
        TestCase {
            byte: 0x0F,
            added_byte: 0xF0,
            carry: false,
            expected_carry: false,
        },
        TestCase {
            byte: 0x0F,
            added_byte: 0xF0,
            carry: true,
            expected_carry: true,
        },
        TestCase {
            byte: 0xFF,
            added_byte: 0x00,
            carry: true,
            expected_carry: true,
        },
        TestCase {
            byte: 0xFF,
            added_byte: 0x01,
            carry: false,
            expected_carry: true,
        },
    ];

    for tc in test_cases {
        let result = bit::is_carry(tc.byte, tc.added_byte, tc.carry);
        assert_eq!(tc.expected_carry, result);
    }
}

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

#[test]
fn is_half_carry_word() {
    struct TestCase {
        word: u16,
        added_word: u16,
        carry: bool,
        expected_half_carry: bool,
    }

    let test_cases: Vec<TestCase> = vec![
        TestCase {
            word: 0x0FFF,
            added_word: 0x0001,
            carry: false,
            expected_half_carry: true,
        },
        TestCase {
            word: 0x0FFF,
            added_word: 0x0000,
            carry: true,
            expected_half_carry: true,
        },
        TestCase {
            word: 0x0FFF,
            added_word: 0x0000,
            carry: false,
            expected_half_carry: false,
        },
        TestCase {
            word: 0xFFFF,
            added_word: 0xFFFF,
            carry: false,
            expected_half_carry: true,
        },
        TestCase {
            word: 0xFFFF,
            added_word: 0xF000,
            carry: false,
            expected_half_carry: false,
        },
    ];

    for tc in test_cases {
        let result = bit::is_half_carry_word(tc.word, tc.added_word, 0x0FFF, tc.carry);
        assert_eq!(tc.expected_half_carry, result)
    }
}

#[test]
fn two_compliment_byte() {
    struct TestCase {
        input_byte: u8,
        expected_byte: u8,
    }

    let test_cases: Vec<TestCase> = vec![
        TestCase {
            input_byte: 0xFF,
            expected_byte: 0x01,
        },
        TestCase {
            input_byte: 0x01,
            expected_byte: 0xFF,
        },
    ];

    for tc in test_cases {
        let result = bit::two_compliment_byte(tc.input_byte);
        assert_eq!(result, tc.expected_byte);
    }
}

#[test]
fn test_most_significant_bit() {
    struct TestCase {
        input_byte: u8,
        expected_result: bool,
    }

    let test_cases: Vec<TestCase> = vec![
        TestCase {
            input_byte: 0x80,
            expected_result: true,
        },
        TestCase {
            input_byte: 0x40,
            expected_result: false,
        },
    ];

    for tc in test_cases {
        assert_eq!(
            bit::test_most_significant_bit(tc.input_byte),
            tc.expected_result,
        );
    }
}

#[test]
fn is_borrow() {
    struct TestCase {
        byte: u8,
        subtracted_byte: u8,
        carry: bool,
        expected_borrow: bool,
    }

    let test_cases: Vec<TestCase> = vec![
        TestCase {
            byte: 0x00,
            subtracted_byte: 0x01,
            carry: false,
            expected_borrow: true,
        },
        TestCase {
            byte: 0x00,
            subtracted_byte: 0x00,
            carry: true,
            expected_borrow: true,
        },
        TestCase {
            byte: 0x00,
            subtracted_byte: 0x00,
            carry: false,
            expected_borrow: false,
        },
    ];

    for tc in test_cases {
        let result = bit::is_borrow(tc.byte, tc.subtracted_byte, tc.carry);
        assert_eq!(tc.expected_borrow, result);
    }
}
