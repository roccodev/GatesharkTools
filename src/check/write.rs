#[macro_use]
use crate::check;
use crate::check::{Checker, CheckResult, errors};
use crate::cheat::Opcode;

pub struct WriteChecker;
impl Checker for WriteChecker {
    fn check(&self, instr: Opcode, block_a: &str, block_b: &str) -> CheckResult {
        err_if!(instr == Opcode::WriteShort && !block_b.starts_with("0000"), errors::WRONG_SIZE);
        err_if!(instr == Opcode::WriteByte && !block_b.starts_with("000000"), errors::WRONG_SIZE);
        CheckResult::Pass
    }
}