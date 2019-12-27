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

#[cfg(test)]
mod tests {
    use crate::parse::parse_cheat;
    use crate::cheat::Opcode;

    #[test]
    pub fn test_parsing() {
        let cheat = "[Test Cheat]\n0AF2CD18 CFF2AD4C";
        let parsed = parse_cheat(cheat.lines().map(|s| s.to_owned())
            .collect::<Vec<String>>().as_slice());
        assert_eq!("[Test Cheat]", parsed.descriptor.name);
        assert_eq!(Opcode::WriteWord, parsed.instructions[0].opcode);
        assert_eq!("0AF2CD18", parsed.instructions[0].block_a);
        assert_eq!("CFF2AD4C", parsed.instructions[0].block_b);
    }
}