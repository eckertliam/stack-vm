pub enum Type {
    UInt,
    Int,
    Float,
    Bool,
    Void,
}

pub struct Function {
    name: String,
    args: Vec<Type>,
    ret: Type,
    instructions: Vec<Instruction>,
}

pub enum Instruction {
    Invoke(String),
    ULoad(usize),
    ILoad(usize),
    FLoad(usize),
    UConst(u64),
    IConst(i64),
    FConst(f64),
    UAdd,
    USub,
    UMul,
    UDiv,
    IAdd,
    ISub,
    IMul,
    IDiv,
    FAdd,
    FSub,
    FMul,
    FDiv,
    IPrint,
    FPrint,
    UPrint,
    BPrint,
    IReturn,
    FReturn,
    UReturn,
    BReturn,
}
