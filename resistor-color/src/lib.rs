use int_enum::IntEnum;
use std::fmt;
use enum_iterator;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum, enum_iterator::Sequence)]
pub enum ResistorColor {
    Black = 0,
    Brown = 1,
    Red = 2,
    Orange = 3,
    Yellow = 4,
    Green = 5,
    Blue = 6,
    Violet = 7,
    Grey = 8,
    White = 9,
}

impl fmt::Display for ResistorColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn color_to_value(_color: ResistorColor) -> u32 {
    return _color.int_value();
}

pub fn value_to_color_string(value: u32) -> String {
    return match ResistorColor::from_int(value) {
        Ok(s) => s.to_string(),
        Err(_) => String::from("value out of range"),
    };
}

pub fn colors() -> Vec<ResistorColor> {
    return enum_iterator::all::<ResistorColor>().collect::<Vec<_>>();
}
