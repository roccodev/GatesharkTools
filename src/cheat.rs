/*
 *    Copyright 2019 RoccoDev
 *
 *    Licensed under the Apache License, Version 2.0 (the "License");
 *    you may not use this file except in compliance with the License.
 *    You may obtain a copy of the License at
 *
 *        http://www.apache.org/licenses/LICENSE-2.0
 *
 *    Unless required by applicable law or agreed to in writing, software
 *    distributed under the License is distributed on an "AS IS" BASIS,
 *    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *    See the License for the specific language governing permissions and
 *    limitations under the License.
*/

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
    BtnCode = 0xdd,
    PatchCode = 0xe,
    MemoryCopy = 0xf
}