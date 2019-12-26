type Error = (usize, &'static str);

macro_rules! err {
    ($name:tt, $num:expr, $msg:expr) => {
        pub const $name: Error = ($num, $msg);
    };
}

err!(WRONG_PARAM, 0, "Wrong parameter P used in Block A (PXXXXXXX). Check Gateshark reference for information.");