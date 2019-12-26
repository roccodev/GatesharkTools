#[macro_use]
use crate::check;
use crate::check::{Checker, CheckResult, errors};
use crate::cheat::Opcode;

struct WriteChecker;
impl Checker for WriteChecker {
    fn check(&self, instr: &Opcode, block_a: &str, block_b: &str) -> CheckResult {
        err_if!(*instr == Opcode::WriteWord && !block_a.starts_with("0"), errors::WRONG_PARAM);
        err_if!(*instr == Opcode::WriteShort && !block_a.starts_with("1"), errors::WRONG_PARAM);
        err_if!(*instr == Opcode::WriteByte && !block_a.starts_with("2"), errors::WRONG_PARAM);
        CheckResult::Pass
    }
}