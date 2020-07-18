#[allow(dead_code)] // Allows variants of the enum to be unused. Cool feature to have the compiler check this!
#[derive(Debug, Clone, Copy, PartialEq, Eq)] // Enables copy semantics for the enum, makes it printable, and makes it equatable
#[repr(u8)] // Specifies the representation of an enum and allows its variants to have associated values
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}