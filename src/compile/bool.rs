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

use crate::cheat::Opcode::{self, *};
use crate::compile::{Implementation, IntoCompiled};

pub struct WordBoolCompiler;

impl IntoCompiled for WordBoolCompiler {
    fn compile(&self, opcode: Opcode, block_a: &String, block_b: &String, env: &Implementation) -> String {
        match *env {
            Implementation::C {ntr} => {
                format!("if ({}) {{", {
                    format!("{} {} 0x{}", {
                        if ntr {
                            format!("READU32(0x0{} + offset)", block_a)
                        }
                        else {
                            format!("*(u32 *)(0x0{} + offset)", block_a)
                        }
                    }, get_c_operator(opcode), block_b)
                })
            }
        }
    }
}

fn get_c_operator(opcode: Opcode) -> String {
    match opcode {
        GtShort | GtWord => ">",
        LtShort | LtWord => "<",
        EqShort | EqWord => "==",
        NeShort | NeWord => "!=",
        _ => panic!("Invalid opcode for boolean.")
    }.to_owned()
}