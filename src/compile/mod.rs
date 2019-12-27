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

use crate::cheat::Cheat;
use crate::cheat::Opcode::{self, *};

mod write;
mod bool;

pub trait IntoCompiled {
    fn compile(&self, opcode: Opcode, block_a: &String, block_b: &String, env: &Implementation) -> String;
}


pub enum Implementation {
    C {ntr: bool}
}

pub struct EndBracketCompiler;
pub struct SetOffsetCompiler;


impl IntoCompiled for EndBracketCompiler {
    fn compile(&self, opcode: Opcode, block_a: &String, block_b: &String, env: &Implementation) -> String {
        match *env {
            Implementation::C {ntr} => "}".to_owned()
        }
    }
}

impl IntoCompiled for SetOffsetCompiler {
    fn compile(&self, opcode: Opcode, block_a: &String, block_b: &String, env: &Implementation) -> String {
        match *env {
            Implementation::C {ntr} => {
                match opcode {
                    SetOffsetImmediate => format!("offset = 0x{};", block_b),
                    SetOffsetPtr => format!("offset = {};", {
                        if ntr {
                            format!("READU32(0x0{} + offset)", block_a)
                        }
                        else {
                            format!("*(u32 *)(0x0{} + offset)", block_a)
                        }
                    }),
                    AddOffset => format!("offset += 0x{};", block_b),
                    _ => panic!("Invalid opcode for offset.")
                }
            }
        }.to_owned()
    }
}

pub fn get_compiler(opcode: Opcode) -> Box<dyn IntoCompiled> {
    match opcode {
        WriteShort | WriteByte | WriteWord => Box::new(write::WriteCompiler),
        GtWord | LtWord | EqWord | NeWord => Box::new(r#bool::WordBoolCompiler),
        SetOffsetPtr | SetOffsetImmediate | AddOffset => Box::new(SetOffsetCompiler),
        EndCond => Box::new(EndBracketCompiler),
        _ => Box::new(write::WriteCompiler)
    }
}

pub fn compile_cheat(cheat: &Cheat, env: &Implementation) {
    for instr in &cheat.instructions {
        let compiler = get_compiler(instr.opcode);
        let compiled = compiler.compile(instr.opcode, &instr.block_a, &instr.block_b, env);
    }
}