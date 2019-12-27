use crate::cheat::Opcode::{self, *};
use crate::cheat::{Cheat, Instruction};
use crate::check::CheckResult::Error;

macro_rules! err_if {
    ($assertion:expr, $err:expr) => {
        if $assertion {
            return CheckResult::Error($err.0, $err.1.to_owned());
        }
    };
}

pub mod write;
pub mod comparison;
mod errors;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum CheckResult {
    Pass,
    Warning(String),
    Error(usize, String)
}

pub trait Checker {
    fn check(&self, instr: Opcode, block_a: &str, block_b: &str) -> CheckResult;
}

pub fn get_checker(opcode: Opcode) -> Box<dyn Checker> {
    Box::new(match opcode {
        WriteWord | WriteShort | WriteByte => write::WriteChecker,
        _ => write::WriteChecker
    })
}

fn check_instruction_pre(instruction: &Instruction, results: &mut Vec<CheckResult>) {
    if instruction.block_a.len() != 8 {
        results.push(CheckResult::Error(errors::WRONG_LENGTH_A.0, errors::WRONG_LENGTH_A.1.to_owned()));
    }
    if instruction.block_b.len() != 8 {
        results.push(CheckResult::Error(errors::WRONG_LENGTH_B.0, errors::WRONG_LENGTH_B.1.to_owned()));
    }
    if i64::from_str_radix(&instruction.block_a, 16).is_err() {
        results.push(CheckResult::Error(errors::INVALID_HEX_A.0, errors::INVALID_HEX_A.1.to_owned()));
    }
    if i64::from_str_radix(&instruction.block_b, 16).is_err() {
        results.push(CheckResult::Error(errors::INVALID_HEX_B.0, errors::INVALID_HEX_B.1.to_owned()));
    }
}

pub fn check_cheat(cheat: &Cheat) -> (CheckResult, Vec<CheckResult>) {
    let mut results = vec![];
    for instr in &cheat.instructions {
        check_instruction_pre(instr, &mut results);
        let result = instr.checker.check(instr.opcode, &instr.block_a, &instr.block_b);
        results.push(result);
    }
    let mut final_res = CheckResult::Pass;
    if results.iter().filter(|r| if let CheckResult::Error(_, _) = **r {true} else {false})
        .count() > 0 {
        final_res = CheckResult::Error(0, "Instruction compiled with errors.".to_owned());
    }
    else if results.iter().filter(|r| if let CheckResult::Warning(_) = **r {true} else {false})
        .count() > 0 {
        final_res = CheckResult::Warning("Instruction compiled with warnings.".to_owned());
    }
    (final_res, results)
}

#[cfg(test)]
mod tests {
    use crate::cheat::{Instruction, Opcode, Cheat, Descriptor};
    use crate::check::{get_checker, check_cheat, CheckResult};

    #[test]
    pub fn pass_check() {
        assert_eq!(CheckResult::Pass, check("0AF2CD18", "CFF2AD4C"));
    }

    #[test]
    pub fn fail_check() {
        assert_ne!(CheckResult::Pass, check("0GZA7F9C", "A4B8LF7J8L82JK"));
    }

    fn check<'a>(block_a: &str, block_b: &str) -> CheckResult {
        let instruction = Instruction {
            opcode: Opcode::WriteWord,
            block_a: block_a.to_string(),
            block_b: block_b.to_string(),
            checker: get_checker(Opcode::WriteWord)
        };
        let cheat = Cheat {
            descriptor: Descriptor {
                name: "[Test Cheat]".to_string()
            },
            instructions: vec![instruction]
        };
        let (result, _) = check_cheat(&cheat);
        result
    }
}