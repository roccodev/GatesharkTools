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

type Error = (usize, &'static str);

macro_rules! err {
    ($name:tt, $num:expr, $msg:expr) => {
        pub const $name: Error = ($num, $msg);
    };
}

err!(WRONG_SIZE, 1, "Wrong size used in Block B. Check Gateshark reference for more information.");
err!(INVALID_HEX_A, 2, "Invalid hexadecimal value in Block A. Allowed values are A-F, 0-9.");
err!(INVALID_HEX_B, 2, "Invalid hexadecimal value in Block B. Allowed values are A-F, 0-9.");
err!(WRONG_LENGTH_A, 3, "Wrong length in Block A. Hexadecimal blocks should be 8 characters in length.");
err!(WRONG_LENGTH_B, 3, "Wrong length in Block B. Hexadecimal blocks should be 8 characters in length.");
err!(ZERO_B, 4, "Block B should be 00000000.");
err!(ZERO_A, 4, "Block A should be all 0 after the opcode.");