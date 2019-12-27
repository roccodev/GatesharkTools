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
#[derive(Eq, PartialEq, TryFromPrimitive, Copy, Clone, Debug)]
pub enum Opcode {
    WriteWord = 0x0,
    WriteShort = 0x1,
    WriteByte = 0x2,
    LtWord = 0x3,
    GtWord = 0x4,
    EqWord = 0x5,
    NeWord = 0x6,
    LtShort = 0x7,
    GtShort = 0x8,
    EqShort = 0x9,
    NeShort = 0xa,
    SetOffsetPtr = 0xb,
    Repeat = 0xc,
    EndCond = 0xd0,
    EndRepeat = 0xd1,
    Reset = 0xd2,
    SetOffsetImmediate = 0xd3,
    AddToDxData = 0xd4,
    SetDxData = 0xd5,
    CopyDxWord = 0xd6,
    CopyDxShort = 0xd7,
    CopyDxByte = 0xd8,
    LoadDxWord = 0xd9,
    LoadDxShort = 0xda,
    LoadDxByte = 0xdb,
    AddOffset = 0xdc,
    BtnCode = 0xdd
}