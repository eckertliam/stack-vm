#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum Opcode {
    Invoke,
    ULoad,
    ILoad,
    FLoad,
    UConst,
    IConst,
    FConst,
    UAdd,
    USub,
    UMul,
    IAdd,
    ISub,
    IMul,
    IDiv,
    FAdd,
    FSub,
    FMul,
    FDiv,
    IReturn,
    FReturn,
    UReturn,
    BReturn,
    IPrint,
    FPrint,
    UPrint,
    BPrint,
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
