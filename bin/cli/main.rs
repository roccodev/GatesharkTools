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
#[macro_use]
extern crate clap;

use std::convert::TryInto;

use clap::{App, AppSettings};

mod check;
mod compile;

#[derive(Debug)]
pub enum CliError {
    Subcommand,
    File,
    Check
}

fn main() {
    let yml = load_yaml!("res/cli.yml");
    let app = App::from_yaml(yml).setting(AppSettings::ArgRequiredElseHelp);
    let matches = app.get_matches();
    let (name, sub) = matches.subcommand();
    let sub = sub.unwrap();
    let _res = match name {
        "check" => {check::check(sub.value_of("file").unwrap().to_owned()); Ok(())}, // TODO Prettify
        "compile" => compile::compile(sub.value_of("file").unwrap(),
                                      sub.value_of("output"), sub.value_of("implementation")),
        _ => Err(CliError::Subcommand)
    };
}