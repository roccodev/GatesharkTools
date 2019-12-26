use crate::check;
use crate::check::{Checker, CheckResult, errors};
use crate::cheat::Opcode;

pub struct ComparisonChecker;
impl Checker for ComparisonChecker {
    fn check(&self, instr: Opcode, block_a: &str, block_b: &str) -> CheckResult {
        CheckResult::Pass
    }
}