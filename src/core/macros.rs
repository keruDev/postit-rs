//! Useful macros (WIP).

/// Equivalent to the [`eprintln!`] macro, but also finishes the program's
/// execution with 1 as the exit code.
#[macro_export]
macro_rules! exit {
    () => {{
        ::std::process::exit(1);
    }};
    ($($arg:tt)*) => {{
        eprintln!($($arg)*);
        ::std::process::exit(1);
    }};
}
