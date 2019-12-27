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

use crate::check;
use crate::check::{Checker, CheckResult, errors};
use crate::cheat::Opcode;

pub struct WriteChecker;
impl Checker for WriteChecker {
    fn check(&self, instr: Opcode, block_a: &str, block_b: &str) -> CheckResult {
        let short = match instr {
            Opcode::WriteShort | Opcode::GtShort | Opcode::LtShort | Opcode::EqShort | Opcode::NeShort => true,
            _ => false
        };
        err_if!(short && !block_b.starts_with("0000"), errors::WRONG_SIZE);
        err_if!(instr == Opcode::WriteByte && !block_b.starts_with("000000"), errors::WRONG_SIZE);
        CheckResult::Pass
    }
}

pub struct ResetChecker;
impl Checker for ResetChecker {
    fn check(&self, instr: Opcode, block_a: &str, block_b: &str) -> CheckResult {
        err_if!(block_b.ne("00000000"), errors::ZERO_B);
        err_if!(instr == Opcode::EndCond && block_a.ne("d0000000"), errors::ZERO_A);
        err_if!(instr == Opcode::Reset && block_a.ne("d2000000"), errors::ZERO_A);
        CheckResult::Pass
    }
}

pub struct ZeroAfterOpcodeChecker;
impl Checker for ZeroAfterOpcodeChecker {
    fn check(&self, instr: Opcode, block_a: &str, block_b: &str) -> CheckResult {
        let opcode = format!("{:x}", instr as usize);
        let blk_a = if !opcode.starts_with("d") {(&opcode[0..1]).to_owned() + "0000000"}
        else {(&opcode[0..2]).to_owned() + "000000"};
        err_if!(block_a != blk_a, errors::ZERO_A);
        CheckResult::Pass
    }
}

#[cfg(test)]
mod tests {
    use crate::cheat::{Instruction, Opcode};
    use crate::check::checks::{ResetChecker, ZeroAfterOpcodeChecker};
    use crate::check::CheckResult;

    #[test]
    pub fn pass_reset() {
        let instr = Instruction {
            opcode: Opcode::Reset,
            block_a: "d2000000".to_string(),
            block_b: "00000000".to_string(),
            checker: Box::new(ResetChecker)
        };
        assert_eq!(CheckResult::Pass, instr.checker.check(instr.opcode, &instr.block_a, &instr.block_b));
    }

    #[test]
    pub fn fail_reset() {
        let instr = Instruction {
            opcode: Opcode::Reset,
            block_a: "d2000010".to_string(),
            block_b: "00100000".to_string(),
            checker: Box::new(ResetChecker)
        };
        assert_ne!(CheckResult::Pass, instr.checker.check(instr.opcode, &instr.block_a, &instr.block_b));
    }

    #[test]
    pub fn pass_zero() {
        let instr = Instruction {
            opcode: Opcode::CopyDxByte,
            block_a: "d8000000".to_string(),
            block_b: "a5c11ee2".to_string(),
            checker: Box::new(ZeroAfterOpcodeChecker)
        };
        assert_eq!(CheckResult::Pass, instr.checker.check(instr.opcode, &instr.block_a, &instr.block_b));
    }

    #[test]
    pub fn fail_zero() {
        let instr = Instruction {
            opcode: Opcode::CopyDxByte,
            block_a: "d8001000".to_string(),
            block_b: "a5c11ee2".to_string(),
            checker: Box::new(ZeroAfterOpcodeChecker)
        };
        assert_ne!(CheckResult::Pass, instr.checker.check(instr.opcode, &instr.block_a, &instr.block_b));
    }
}