#![doc = include_str!("../README.md")]

#[doc(hidden)]
pub mod _internal {

    // once implementation from bevy_utils (bevyengine.org), also dual-licensed under the MIT and Apache licenses
    #[doc(hidden)]
    #[macro_export]
    macro_rules! _internal_once {
        ($expression:expr) => {{
            use ::core::sync::atomic::{AtomicBool, Ordering};

            static SHOULD_FIRE: AtomicBool = AtomicBool::new(true);
            if SHOULD_FIRE.swap(false, Ordering::Relaxed) {
                $expression;
            }
        }};
    }

    #[doc(hidden)]
    pub use tracing::error as _error;

    // macro_export will move the macro to the root module, thus it is necessary to use `super::` here
    #[doc(hidden)]
    pub use super::_internal_once as _once;
}

/// When enabled, will pause execution with a break point
///
/// Requires Nightly Rust
///
/// # Examples
///
/// ```rust
/// unbug::breakpoint!();
/// ```
#[macro_export]
#[cfg(not(all(debug_assertions, feature = "enable")))]
macro_rules! breakpoint {
    () => {};
}
#[macro_export]
#[cfg(all(debug_assertions, feature = "enable"))]
macro_rules! breakpoint {
    () => {
        unsafe {
            std::intrinsics::breakpoint();
        }
    };
}

/// When enabled, will pause execution with a break point when expression is false
///
/// Will break on every execution
///
/// Requires Nightly Rust
///
/// # Examples
///
/// ```rust
/// unbug::ensure_always!(false);
/// ```
#[macro_export]
#[cfg(not(all(debug_assertions, feature = "enable")))]
macro_rules! ensure_always {
    ($expression: expr) => {};
}
#[macro_export]
#[cfg(all(debug_assertions, feature = "enable"))]
macro_rules! ensure_always {
    ($expression: expr) => {
        if !$expression {
            unbug::breakpoint!();
        }
    };
}

/// When enabled, will pause execution with a break point when expression is false
///
/// Will only break once per program run
///
/// Requires Nightly Rust
///
/// # Examples
///
/// ```rust
/// unbug::ensure!(false);
/// ```
#[macro_export]
#[cfg(not(all(debug_assertions, feature = "enable")))]
macro_rules! ensure {
    ($expression: expr) => {};
}
#[macro_export]
#[cfg(all(debug_assertions, feature = "enable"))]
macro_rules! ensure {
    ($expression: expr) => {
        unbug::_internal::_once!(unbug::ensure_always!($expression))
    };
}

/// When enabled, will log an error and pause execution
///
/// Will break on every execution
///
/// When disabled, will log an error
///
/// Will log on every execution
///
/// arguments are processed by tracing's error! macro
///
/// Requires Nightly Rust
///
/// Requires Tracing logging for error messages
///
/// # Examples
///
/// ```rust
/// unbug::fail_always!("failed to do something");
/// unbug::fail_always!("failed to do something: {:?}", some_var);
/// ```
#[macro_export]
#[cfg(not(all(debug_assertions, feature = "enable")))]
macro_rules! fail_always {
    ($($argument: tt),+ $(,)?) => {
        unbug::_internal::_error!($($argument),+);
    };
}
#[macro_export]
#[cfg(all(debug_assertions, feature = "enable"))]
macro_rules! fail_always {
    ($($argument: tt),+ $(,)?) => {
        unbug::_internal::_error!($($argument),+);
        unbug::breakpoint!();
    };
}

/// When enabled, will log an error and pause execution
///
/// Will only break once per program run
///
/// When disabled, will log an error
///
/// Will log once per program run
///
/// arguments are processed by Tracing's error! macro
///
/// Requires Nightly Rust
///
/// Requires Tracing logging for error messages
///
/// # Examples
///
/// ```rust
/// unbug::fail!("failed to do something");
/// unbug::fail!("failed to do something: {:?}", some_var);
/// ```
#[macro_export]
macro_rules! fail {
    ($($argument: tt),+ $(,)?) => {
        unbug::_internal::_once!(unbug::fail_always!($($argument),+));
    };
}

pub mod prelude {
    pub use super::{breakpoint, ensure, ensure_always, fail, fail_always};
}
