#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    Down,
    Up,
    North,
    South,
    West,
    East,
}

impl Default for Facing {
    fn default() -> Self { Facing::Down }
}

impl Facing {
    pub fn to_i8(&self) -> i8 {
        *self as i8
    }

    pub fn from_i8(byte: i8) -> Option<Facing> {
        match byte {
            0 => Some(Facing::Down),
            1 => Some(Facing::Up),
            2 => Some(Facing::North),
            3 => Some(Facing::South),
            4 => Some(Facing::West),
            5 => Some(Facing::East),
            _ => None,
        }
    }
}