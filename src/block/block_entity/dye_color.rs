#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DyeColor {
    Black,
    Red,
    Green,
    Brown,
    Blue,
    Purple,
    Cyan,
    Silver,
    Gray,
    Pink,
    Lime,
    Yellow,
    LightBlue,
    Magenta,
    Orange,
    White,
}

impl Default for DyeColor {
    fn default() -> Self {
        DyeColor::White
    }
}

impl DyeColor {
    pub fn to_i32(&self) -> i32 {
        *self as i32
    }

    pub fn from_i32(x: i32) -> Option<Self> {
        use self::DyeColor::*;
        match x {
            0 => Some(Black),
            1 => Some(Red),
            2 => Some(Green),
            3 => Some(Brown),
            4 => Some(Blue),
            5 => Some(Purple),
            6 => Some(Cyan),
            7 => Some(Silver),
            8 => Some(Gray),
            9 => Some(Pink),
            10 => Some(Lime),
            11 => Some(Yellow),
            12 => Some(LightBlue),
            13 => Some(Magenta),
            14 => Some(Orange),
            15 => Some(White),
            _ => None,
        }
    }
}