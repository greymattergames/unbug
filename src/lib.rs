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
/// Platforms other than x86, x86_64, and ARM64 require Nightly Rust and the `breakpoint` feature
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
#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    debug_assertions,
    feature = "enable"
))]
macro_rules! breakpoint {
    () => {
        unsafe {
            ::core::arch::asm!("int3;nop");
        }
    };
}
#[macro_export]
#[cfg(all(
    target_arch = "aarch64",
    debug_assertions,
    feature = "enable"
))]
macro_rules! breakpoint {
    () => {
        unsafe {
            ::core::arch::asm!("brk#0xF000\nnop");
        }
    };
}
#[macro_export]
#[cfg(all(
    not(any(
        target_arch = "x86",
        target_arch = "x86_64",
        target_arch = "aarch64",
    )),
    debug_assertions,
    feature = "enable"
))]
macro_rules! breakpoint {
    () => {
        unsafe {
            ::core::arch::breakpoint();
        }
    };
}

/// When enabled, will pause execution with a break point when expression is false
///
/// Will break on every execution
///
/// additional arguments are processed by Tracing's error! macro (Tracing required)
///
/// Platforms other than x86, x86_64, and ARM64 Require Nightly Rust and the core_intrinsics feature
///
/// # Examples
///
/// ```rust
/// unbug::ensure_always!(false);
/// unbug::ensure_always!(false, "some message to log before breaking");
/// unbug::ensure_always!(false, "a formatted message to log {:?}", some_var);
/// ```
#[macro_export]
#[cfg(not(all(debug_assertions, feature = "enable")))]
macro_rules! ensure_always {
    ($expression: expr) => {};
    ($expression: expr, $($argument: tt),+ $(,)?) => {};
}
#[macro_export]
#[cfg(all(debug_assertions, feature = "enable"))]
macro_rules! ensure_always {
    ($expression: expr) => {
        if !$expression {
            $crate::breakpoint!();
        }
    };
    ($expression: expr, $($argument: tt),+ $(,)?) => {
        if !$expression {
            $crate::_internal::_error!($($argument),+);
            $crate::breakpoint!();
        }
    };
}

/// When enabled, will pause execution with a break point when expression is false
///
/// Will only break once per program run
///
/// additional arguments are processed by Tracing's error! macro (Tracing required)
///
/// Platforms other than x86, x86_64, and ARM64 Require Nightly Rust and the core_intrinsics feature
///
/// # Examples
///
/// ```rust
/// unbug::ensure!(false);
/// unbug::ensure!(false, "some message to log before breaking");
/// unbug::ensure!(false, "a formatted message to log {:?}", some_var);
/// ```
#[macro_export]
#[cfg(not(all(debug_assertions, feature = "enable")))]
macro_rules! ensure {
    ($expression: expr) => {};
    ($expression: expr, $($argument: tt),+ $(,)?) => {};
}
#[macro_export]
#[cfg(all(debug_assertions, feature = "enable"))]
macro_rules! ensure {
    ($expression: expr) => {
        if !$expression {
            $crate::_internal::_once!($crate::breakpoint!());
        }
    };
    ($expression: expr, $($argument: tt),+ $(,)?) => {
        if !$expression {
            $crate::_internal::_once!({
                $crate::_internal::_error!($($argument),+);
                $crate::breakpoint!();
            });
        }
    };
}

/// When enabled, will log an error and pause execution
///
/// Will break on every execution
///
/// arguments are processed by Tracing's error! macro (Tracing required)
///
/// When disabled, will log an error
///
/// Will log on every execution
///
/// Platforms other than x86, x86_64, and ARM64 Require Nightly Rust and the core_intrinsics feature
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
        $crate::_internal::_error!($($argument),+);
    };
}
#[macro_export]
#[cfg(all(debug_assertions, feature = "enable"))]
macro_rules! fail_always {
    ($($argument: tt),+ $(,)?) => {
        $crate::_internal::_error!($($argument),+);
        $crate::breakpoint!();
    };
}

/// When enabled, will log an error and pause execution
///
/// Will only break once per program run
///
/// arguments are processed by Tracing's error! macro (Tracing required)
///
/// When disabled, will log an error
///
/// Will log once per program run
///
/// Platforms other than x86, x86_64, and ARM64 Require Nightly Rust and the core_intrinsics feature
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
        $crate::_internal::_once!($crate::fail_always!($($argument),+));
    };
}

pub mod prelude {
    pub use super::{breakpoint, ensure, ensure_always, fail, fail_always};
}
