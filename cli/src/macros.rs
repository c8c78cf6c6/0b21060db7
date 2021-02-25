#[macro_export]
macro_rules! unwrap_or_err /*that is the question*/ {
    ($val:expr, $error: expr) => {
        match $val {
            Err(_) => {
                return Err($error);
            },
            Ok(val) => val,
        }
    };
}
