type Error = (usize, &'static str);

macro_rules! err {
    ($name:tt, $num:expr, $msg:expr) => {
        pub const $name: Error = ($num, $msg);
    };
}

err!(WRONG_SIZE, 0, "Wrong size used in Block B. Check Gateshark reference for more information.");