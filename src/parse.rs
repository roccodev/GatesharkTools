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

use std::convert::TryFrom;

use crate::cheat::{Cheat, Descriptor, Instruction, Opcode};
use crate::check::get_checker;

pub fn parse_cheat(input: &[String]) -> Cheat {
    let desc_line = &input[0];
    let mut instructions = vec![];
    input.iter().skip(1).for_each(|line| {
        let mut blocks = line.split(" ");
        let blk_a = blocks.next().unwrap();
        let blk_b = blocks.next().unwrap();
        let opcode = {
            let first = blk_a.chars().take(2).map(|c| c.to_string())
                .collect::<Vec<String>>().join("").to_lowercase();
            if first.starts_with("d") {
                usize::from_str_radix(&first, 16).unwrap()
            }
            else {
                usize::from_str_radix(&first[0..1], 16).unwrap()
            }
        };
        let opcode = Opcode::try_from(opcode).unwrap();
        instructions.push(Instruction {
            opcode,
            block_a: blk_a.to_owned().to_lowercase(),
            block_b: blk_b.to_owned().to_lowercase(),
            checker: get_checker(opcode)
        });
    });
    Cheat {
        descriptor: Descriptor {
            name: desc_line.to_owned()
        },
        instructions
    }
}

#[cfg(test)]
mod tests {
    use crate::cheat::Opcode;
    use crate::parse::parse_cheat;

    #[test]
    pub fn test_parsing() {
        let cheat = "[Test Cheat]\n0AF2CD18 CFF2AD4C";
        let parsed = parse_cheat(cheat.lines().map(|s| s.to_owned())
            .collect::<Vec<String>>().as_slice());
        assert_eq!("[Test Cheat]", parsed.descriptor.name);
        assert_eq!(Opcode::WriteWord, parsed.instructions[0].opcode);
        assert_eq!("0af2cd18", parsed.instructions[0].block_a);
        assert_eq!("cff2ad4c", parsed.instructions[0].block_b);
    }
}