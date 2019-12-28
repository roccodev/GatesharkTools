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

use std::marker::PhantomData;

use crate::cheat::{Cheat, Descriptor, Opcode};
use crate::compile::{get_compiler, Implementation, IntoCompiled};

pub struct WriteCompiler;
pub struct DxDataCompiler;

impl IntoCompiled for WriteCompiler {
    fn compile(&self, opcode: Opcode, block_a: &String, block_b: &String, env: &mut Implementation) -> String {
        let size = format!("u{}", match opcode {
            Opcode::WriteWord => 32,
            Opcode::WriteShort => 16,
            Opcode::WriteByte => 8,
            _ => panic!("Invalid opcode for write")
        });
        match *env {
            Implementation::C {ntr, conds} => {
                if !ntr {format!("*({} *)(0x0{} + offset) = 0x{};", size, block_a, block_b)} else
                {format!("WRITE{}(0x0{} + offset, 0x{});", size.to_uppercase(), block_a, block_b)}
            },
        }
    }
}

impl IntoCompiled for DxDataCompiler {
    fn compile(&self, opcode: Opcode, block_a: &String, block_b: &String, env: &mut Implementation) -> String {
        let size = format!("{}", match opcode {
            Opcode::CopyDxWord | Opcode::LoadDxWord => 32,
            Opcode::CopyDxShort | Opcode::LoadDxShort => 16,
            Opcode::CopyDxByte | Opcode::LoadDxByte => 8,
            Opcode::SetDxData | Opcode::AddToDxData => 0,
            _ => panic!("Invalid opcode for DxData")
        });
        match *env {
            Implementation::C {ntr, conds} => {
                match opcode {
                    Opcode::CopyDxByte | Opcode::CopyDxShort | Opcode::CopyDxWord => {
                        format!("{} offset += {}; ",
                        if !ntr {format!("*(u{} *)(0x{} + offset) = data;", size, block_b)} else
                        {format!("WRITEU{}(0x{} + offset, data);", size, block_b)}, size)
                    }
                    Opcode::LoadDxByte | Opcode::LoadDxShort | Opcode::LoadDxWord => {
                        if !ntr {format!("data = *(u{} *)(0x{} + offset);", size, block_b)} else
                        {format!("data = READU{}(0x{} + offset);", size, block_b)}
                    }
                    Opcode::SetDxData => {
                        format!("data = 0x{}", block_b)
                    }
                    Opcode::AddToDxData => {
                        format!("data += 0x{}", block_b)
                    }
                    _ => panic!("Invalid opcode for dxdata")
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cheat::Opcode;
    use crate::compile::{Implementation, IntoCompiled};
    use crate::compile::write::WriteCompiler;

    #[test]
    pub fn compile_write() {
        let mut env = Implementation::C {ntr: false, conds: 0};
        let compiler = WriteCompiler;
        assert_eq!("*(u32 *)(0x001 + offset) = 0x02;", compiler.compile(Opcode::WriteWord,
                                                                       &"01".to_owned(), &"02".to_owned(), &mut env));
        let mut env = Implementation::C {ntr: true, conds: 0};
        assert_eq!("WRITEU32(0x001 + offset, 0x02);", compiler.compile(Opcode::WriteWord,
                                                                       &"01".to_owned(), &"02".to_owned(), &mut env));
    }
}
