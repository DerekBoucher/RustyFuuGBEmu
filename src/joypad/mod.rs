#[derive(Clone, Copy, Debug)]
pub enum DirectionButton {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Clone, Copy, Debug)]
pub enum ActionButton {
    A,
    B,
    Select,
    Start,
}

impl DirectionButton {
    pub fn to_u8(&self) -> u8 {
        match self {
            DirectionButton::Right => !(1 << 0),
            DirectionButton::Left => !(1 << 1),
            DirectionButton::Up => !(1 << 2),
            DirectionButton::Down => !(1 << 3),
        }
    }
}

impl ActionButton {
    pub fn to_u8(&self) -> u8 {
        match self {
            ActionButton::A => !(1 << 0),
            ActionButton::B => !(1 << 1),
            ActionButton::Select => !(1 << 2),
            ActionButton::Start => !(1 << 3),
        }
    }
}
