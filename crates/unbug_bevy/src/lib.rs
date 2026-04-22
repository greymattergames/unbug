// /// A convenience constant for returning a successful result in a fallible system.
// pub const OK: Result = Ok(());

use bevy_ecs::error::{Result, BevyError};

const OPTION_ERR_MSG: &str = "Try on Option with None value";

pub trait DebugFailResult<T, E> {
    /// Takes a single argument, `f` which is a closure that takes no arguments nor has any return value
    /// the closure will be evaluated when the Result this is called on is Err
    /// The function signature matches the `on_fail` implemented for Option types
    /// so we can apply it to all invocations of the try-operator (`?`)
    /// for use with the `debug_fail` proc macro
    fn on_fail<F: FnOnce()>(self, f: F) -> Result<T, E>;

    fn try_to_result(self) -> Result<T, E>;
}

impl<T, E> DebugFailResult<T, E> for Result<T, E> {
    // Add `.on_fail` to Result types which returns the original result
    fn on_fail<F: FnOnce()>(self, f: F) -> Result<T, E> {
        if self.is_err() {
            f();
        }

        self
    }

    fn try_to_result(self) -> Result<T, E> {
        self
    }
}

pub trait DebugFailOption<T> {
    /// Takes a single argument, `f` which is a closure that takes no arguments nor has any return value
    /// the closure will be evaluated when the Option this is called on is None
    /// The function signature matches the `on_fail` implemented for Result types
    /// so we can apply it to all invocations of the try-operator (`?`)
    /// for use with the `debug_fail` proc macro
    fn on_fail<F: FnOnce()>(self, f: F) -> Result<T, BevyError>;

    fn try_to_result(self) -> Result<T, BevyError>;
}

impl<T> DebugFailOption<T> for Option<T> {
    // Add `.on_fail` to Option types which will convert the Option to a Result with the default error message
    fn on_fail<F: FnOnce()>(self, f: F) -> Result<T, BevyError> {
        match self {
            Some(t) => Ok(t),
            None => {
                f();
                Err(OPTION_ERR_MSG.into())
            },
        }
    }

    fn try_to_result(self) -> Result<T, BevyError> {
        self.ok_or(OPTION_ERR_MSG.into())
    }
}

pub mod prelude {
    pub use super::*;
}

