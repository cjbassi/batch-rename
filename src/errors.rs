// https://www.reddit.com/r/rust/comments/5u0vkk/nice_error_handling_in_rust/

use std::error::Error;
use std::process::exit;

#[macro_export]
macro_rules! print_error {
    ($fmt:expr, $($arg:tt)*) => ({
        eprintln!(concat!("error: failed to ", $fmt, ": {}"), $($arg)*);
    })
}

pub trait UnwrapOrExit<T> {
    fn unwrap_or_exit(self, message: &str) -> T;
}

impl<T> UnwrapOrExit<T> for Option<T> {
    fn unwrap_or_exit(self, message: &str) -> T {
        match self {
            Some(r) => r,
            None => {
                eprintln!("error: failed to {}", message);
                exit(1);
            }
        }
    }
}

impl<T, E: Error> UnwrapOrExit<T> for Result<T, E> {
    fn unwrap_or_exit(self, message: &str) -> T {
        match self {
            Ok(r) => r,
            Err(e) => {
                print_error!("{}", message, e);
                exit(1);
            }
        }
    }
}
