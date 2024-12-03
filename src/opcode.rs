#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum Opcode {
    Goto,
    UIntImm,
    IntImm,
    FloatImm,
    True,
    False,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Print,
    Return,
}

impl Into<u8> for Opcode {
    fn into(self) -> u8 {
        self as u8
    }
}

impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}
