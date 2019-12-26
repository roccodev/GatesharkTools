use num_enum::TryFromPrimitive;

pub struct Cheat {
    pub descriptor: Descriptor,
    pub instructions: Vec<Instruction>
}

pub struct Descriptor {
    pub name: String
}

pub struct Instruction {
    pub opcode: Opcode,
    pub block_a: String,
    pub block_b: String,
    pub checker: Box<dyn crate::check::Checker>
}

#[repr(usize)]
#[derive(Eq, PartialEq, TryFromPrimitive, Copy, Clone)]
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