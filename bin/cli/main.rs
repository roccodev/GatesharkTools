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

use clap::{App, AppSettings};

mod check;

fn main() {
    let yml = load_yaml!("res/cli.yml");
    let app = App::from_yaml(yml).setting(AppSettings::ArgRequiredElseHelp);
    let matches = app.get_matches();
    match matches.subcommand_name() {
        Some("check") => check::check(matches.subcommand().1
            .unwrap().value_of("file").unwrap().to_owned()), // TODO Prettify
        _ => {}
    }
}