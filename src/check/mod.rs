use crate::cheat::Opcode;

macro_rules! err_if {
    ($assertion:expr, $err:expr) => {
        if $assertion {
            return CheckResult::Error($err.0, $err.1.to_owned());
        }
    };
}

mod write;
mod errors;

enum CheckResult {
    Pass,
    Warning(String),
    Error(usize, String)
}

pub trait Checker {
    fn check(&self, instr: &Opcode, block_a: &str, block_b: &str) -> CheckResult;
}