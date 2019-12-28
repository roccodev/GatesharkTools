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
use std::cell::{Cell, RefCell};
use std::fs::File;
use std::io::{BufRead, BufReader};

use colored::*;

use gateshark_tools::cheat::Cheat;
use gateshark_tools::check::{check_cheat, CheckResult};
use gateshark_tools::parse::parse_cheat;

use crate::CliError;

pub fn check(path: String) -> Result<Vec<Cheat>, CliError> {
    let mut errors = 0;
    let file = match File::open(&path) {
        Ok(f) => f,
        Err(err) => {
            println!("{}: {}: {}", "Error".red().bold(), "Error opening file".white().bold(), err);
            return Err(CliError::File);
        }
    };
    println!("{} {} {}\n", "Running".bright_green().bold(), "check on", path);
    let reader = BufReader::new(&file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let mut cheats = lines.split(|l| l.trim().is_empty());
    let mut parsed = vec![];
    for cheat in cheats {
        let cheat = parse_cheat(cheat);
        let (result, info) = check_cheat(&cheat);
        if result == CheckResult::Pass {
            parsed.push(cheat);
            continue;
        }
        for res in info {
            let line = res.cheat_line;
            match res.res_type {
                CheckResult::Warning(w) => {
                    println!("--> {}: {}", "Warning".yellow().bold(), w);
                    println!("   --> {} @ Line {}", cheat.descriptor.name, line + 2);
                    println!("   --> {} {}", cheat.instructions[line].block_a, cheat.instructions[line].block_b);
                }
                CheckResult::Error(id, msg) => {
                    println!("--> {}: ({}) {}", "Error".red().bold(), id, msg);
                    println!("   --> {} @ Line {}", cheat.descriptor.name, line + 2);
                    println!("   --> {} {}", cheat.instructions[line].block_a, cheat.instructions[line].block_b);
                    errors += 1;
                }
                _ => {}
            }
        }
        parsed.push(cheat);
    }
    println!();
    if errors > 0 {
        println!("{}: {}", "Failed".red().bold(), format!("check returned {} error(s).", errors));
        Err(CliError::Check)
    }
    else {
        println!("{}: {}", "Success".bright_green().bold(), "check returned no errors.");
        Ok(parsed)
    }
}