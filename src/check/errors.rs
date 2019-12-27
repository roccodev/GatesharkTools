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