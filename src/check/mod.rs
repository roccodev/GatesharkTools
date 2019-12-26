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
    if i64::from_str_radix(&instruction.block_a, 16).is_err() {
        results.push(CheckResult::Error(errors::INVALID_HEX_A.0, errors::INVALID_HEX_A.1.to_owned()));
    }
    if i64::from_str_radix(&instruction.block_b, 16).is_err() {
        results.push(CheckResult::Error(errors::INVALID_HEX_B.0, errors::INVALID_HEX_B.1.to_owned()));
    }
}

pub fn check_cheat(cheat: &Cheat) -> Vec<CheckResult> {
    let mut results = vec![];
    let mut errored = false;
    for instr in &cheat.instructions {
        check_instruction_pre(instr, &mut results);
        let result = instr.checker.check(instr.opcode, &instr.block_a, &instr.block_b);
        if let CheckResult::Error(_, _) = result {
            errored = true;
        }
        results.push(result);
    }
    if !errored {
        results.push(CheckResult::Pass);
    }
    results
}