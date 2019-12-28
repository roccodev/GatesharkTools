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
use crate::compile::write::DxDataCompiler;

mod write;
mod bool;

pub trait IntoCompiled {
    fn compile(&self, opcode: Opcode, block_a: &String, block_b: &String, env: &mut Implementation) -> String;
}

pub enum Implementation {
    C {ntr: bool, conds: usize}
}

pub struct EndBracketCompiler;
pub struct SetOffsetCompiler;
pub struct LoopCompiler;
pub struct SkipCompiler;

impl IntoCompiled for EndBracketCompiler {
    fn compile(&self, opcode: Opcode, block_a: &String, block_b: &String, env: &mut Implementation) -> String {
        match *env {
            Implementation::C {ntr, ref mut conds } => {
                if opcode == Reset {
                    let mut brackets = "".to_owned();
                    while *conds != 0 {
                        brackets.push('}');
                        *conds -= 1;
                    }
                    brackets
                }
                else {
                    *conds -= 1;
                    "}".to_owned()
                }
            }
        }
    }
}

impl IntoCompiled for SetOffsetCompiler {
    fn compile(&self, opcode: Opcode, block_a: &String, block_b: &String, env: &mut Implementation) -> String {
        match *env {
            Implementation::C {ntr, conds} => {
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

impl IntoCompiled for LoopCompiler {
    fn compile(&self, opcode: Opcode, block_a: &String, block_b: &String, env: &mut Implementation) -> String {
        match *env {
            Implementation::C {ntr, ref mut conds} => {
                if opcode == Repeat {
                    *conds += 1;
                    println!("Conds: {}", conds);
                    return format!("for (int i = 0; i < 0x{}; i++) {{", block_b);
                }
                else if opcode == EndRepeat {
                    println!("Conds (end) {}", conds);
                    *conds -= 1;
                    return format!("}} offset += 0x{};", block_b);
                }
                panic!("Invalid opcode for loop");
            }
        }
    }
}

impl IntoCompiled for SkipCompiler {
    fn compile(&self, opcode: Opcode, block_a: &String, block_b: &String, env: &mut Implementation) -> String {
        match *env {
            Implementation::C {ntr, conds } => {
                format!("// Instruction 0x{:x} was skipped. (Not supported yet)", opcode as usize)
            }
        }
    }
}

pub fn get_compiler(opcode: Opcode) -> Box<dyn IntoCompiled> {
    match opcode {
        WriteShort | WriteByte | WriteWord => Box::new(write::WriteCompiler),
        GtWord | LtWord | EqWord | NeWord => Box::new(r#bool::WordBoolCompiler),
        GtShort | LtShort | EqShort | NeShort => Box::new(r#bool::ShortBoolCompiler),
        SetOffsetPtr | SetOffsetImmediate | AddOffset => Box::new(SetOffsetCompiler),
        Repeat | EndRepeat => Box::new(LoopCompiler),
        EndCond | Reset => Box::new(EndBracketCompiler),
        LoadDxWord | LoadDxShort | LoadDxByte | SetDxData | AddToDxData | CopyDxWord | CopyDxShort
        | CopyDxByte => Box::new(DxDataCompiler),
        BtnCode | PatchCode | MemoryCopy => Box::new(SkipCompiler)
    }
}

pub fn compile_cheat(cheat: &Cheat, env: &mut Implementation) -> Vec<String> {
    let mut lines = vec![];
    lines.extend(compile_cheat_start(env, cheat));
    for instr in &cheat.instructions {
        let compiler = get_compiler(instr.opcode);
        let blk_a = if instr.opcode as usize > 0xD0 {&instr.block_a[2..]} else {&instr.block_a[1..]};
        let compiled = compiler.compile(instr.opcode, &blk_a.to_owned(), &instr.block_b, env);
        lines.push(compiled);
    }
    lines.extend(compile_cheat_end(env, cheat));
    lines
}

pub fn get_file_header(env: &Implementation) -> Vec<String> {
    let mut lines = vec![];
    match *env {
        Implementation::C {ntr, conds} => {
            lines.push("#define u32 unsigned int".to_owned());
            lines.push("#define u16 unsigned short".to_owned());
            lines.push("#define u8 unsigned char".to_owned());
            lines.push(String::new());
        }
    }
    lines
}

fn compile_cheat_start(env: &Implementation, cheat: &Cheat) -> Vec<String> {
    let mut lines = vec![];
    match *env {
        Implementation::C {ntr, conds} => {
            let re = regex::Regex::new(r"[-/\(\)\[\]\s]").unwrap();
            lines.push(format!("void {}() {{",
                               re.replace_all(&cheat.descriptor.name.to_lowercase(), "_")));
            lines.push("u32 data = 0;".to_owned());
            lines.push("u32 offset = 0;".to_owned());
            lines.push(String::new());
        }
    }
    lines
}

fn compile_cheat_end(env: &Implementation, cheat: &Cheat) -> Vec<String> {
    let mut lines = vec![];
    match *env {
        Implementation::C {ntr, conds} => {
            lines.push("}".to_owned());
        }
    }
    lines
}