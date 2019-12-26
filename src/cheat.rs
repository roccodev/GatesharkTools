struct Cheat {
    descriptor: Descriptor,
    instructions: Vec<Instruction>
}

struct Descriptor {
    name: String
}

struct Instruction {
    opcode: Opcode,
    block_a: String,
    block_b: String,
    checker: Box<dyn crate::check::Checker>
}

#[derive(Eq, PartialEq)]
pub enum Opcode {
    WriteWord,
    WriteShort,
    WriteByte,
    LtWord,
    GtWord,
    EqWord,
    NeWord,
    LtShort,
    GtShort,
    EqShort,
    NeShort,
    SetOffsetPtr,
    Repeat,
    EndCond,
    Reset,
    SetOffsetImmediate,
    AddToDxData,
    SetDxData,
    CopyDxWord,
    CopyDxShort,
    CopyDxByte,
    LoadDxWord,
    LoadDxShort,
    LoadDxByte,
    AddOffset,
    BtnCode
}