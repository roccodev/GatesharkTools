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

use std::fs::OpenOptions;
use std::io::BufWriter;
use std::io::Write;

use colored::*;

use gateshark_tools::compile::{compile_cheat, get_file_header, Implementation};

use crate::CliError;

pub fn compile(input: &str, output: Option<&str>, implementation: Option<&str>) -> Result<(), CliError> {
    let path = input.to_owned();
    let check = crate::check::check(path);
    if check.is_err() {
        println!("{}: {}", "Aborted".red().bold(), "Check failed.".white().bold());
        return Err(CliError::Check);
    }
    let cheats = check.unwrap();
    let mut implementation = Implementation::C {ntr: false, conds: 0}; // TODO modularize
    let output = OpenOptions::new().create(true).write(true).open(output.unwrap());
    let mut output = match output {
        Ok(f) => f,
        Err(err) => {
            println!("{}: {}: {}", "Error".red().bold(), "Error opening file".white().bold(), err);
            return Err(CliError::File);
        }
    };

    let mut write_vec = |v: Vec<String>| {
      v.iter().for_each(|l| writeln!(output, "{}", l).unwrap());
    };

    write_vec(get_file_header(&implementation));
    for cheat in &cheats {
        write_vec(compile_cheat(cheat, &mut implementation));
    }
    println!("{}: {}", "Success".bright_green().bold(), format!("compiled {} cheat(s).", cheats.len()));
    Ok(())
}