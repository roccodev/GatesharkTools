use crate::cheat::Opcode::{self, *};

macro_rules! err_if {
    ($assertion:expr, $err:expr) => {
        if $assertion {
            return CheckResult::Error($err.0, $err.1.to_owned());
        }
    };
}

pub mod write;
mod errors;

enum CheckResult {
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