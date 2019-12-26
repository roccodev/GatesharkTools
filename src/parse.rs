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
                first.parse::<usize>().unwrap()
            }
            else {
                first.chars().nth(0).unwrap().to_string().parse::<usize>().unwrap()
            }
        };
        let opcode = Opcode::try_from(opcode).unwrap();
        instructions.push(Instruction {
            opcode,
            block_a: blk_a.to_owned(),
            block_b: blk_b.to_owned(),
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
