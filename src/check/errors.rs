type Error = (usize, &'static str);

macro_rules! err {
    ($name:tt, $num:expr, $msg:expr) => {
        pub const $name: Error = ($num, $msg);
    };
}

err!(WRONG_SIZE, 0, "Wrong size used in Block B. Check Gateshark reference for more information.");
err!(INVALID_HEX_A, 1, "Invalid hexadecimal value in Block A. Allowed values are A-F, 0-9.");
err!(INVALID_HEX_B, 1, "Invalid hexadecimal value in Block B. Allowed values are A-F, 0-9.");