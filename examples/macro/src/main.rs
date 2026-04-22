use unbug::prelude::*;

// all invocations of the try-operator (?) will have breakpoints attached
// in a function annotated with the debug_fail macro
#[debug_fail]
fn main() -> Result<(), String> {
    // This will have a breakpoint
    get_option(false)?;

    // using fail_msg will also log messages for all try-operator invocations for this expression
    #[fail_msg = "A message to log on failure"]
    get_result(false)?;

    #[fail_msg = "Messages apply to all ? invocations in the following expression"]
    let _certain = get_uncertain(true)?.observe(false)?;

    // using fail_ignore will skip adding breakpoints to all try-operator invocations for this expression
    #[fail_ignore]
    get_result(false)?;

    Ok(())
}

// Unbug macro will expect `.on_fail` and `.try_to_result`
// exists on both Result and Option types
// You will have to construct these trait extensions with your error type

const OPTION_ERR_MSG: &str = "Try on Option with None value";

trait DebugFailResult<T, E> {
    fn on_fail<F: FnOnce()>(self, f: F) -> Result<T, E>;

    #[allow(unused)]
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

trait DebugFailOption<T> {
    fn on_fail<F: FnOnce()>(self, f: F) -> Result<T, String>;

    #[allow(unused)]
    fn try_to_result(self) -> Result<T, String>;
}

impl<T> DebugFailOption<T> for Option<T> {
    // Add `.on_fail` to Option types which will convert the Option to a Result with the default error message
    fn on_fail<F: FnOnce()>(self, f: F) -> Result<T, String> {
        match self {
            Some(t) => Ok(t),
            None => {
                f();
                Err(OPTION_ERR_MSG.to_string())
            },
        }
    }

    fn try_to_result(self) -> Result<T, String> {
        self.ok_or(OPTION_ERR_MSG.to_string())
    }
}

fn get_result(succeed: bool) -> Result<String, String> {
    if succeed {
        return Ok("Some important value".to_string());
    }

    Err("I had a failure".into())
}

fn get_option(succeed: bool) -> Option<String> {
    if succeed {
        return Some("Some important value".to_string());
    }

    None
}

struct Uncertain;

impl Uncertain {
    pub fn observe(&self, succeed: bool) -> Option<String> {
        if succeed {
            return Some("Success!".into());
        }

        None
    }
}

fn get_uncertain(succeed: bool) -> Result<Uncertain, String> {
    if succeed {
        return Ok(Uncertain);
    }

    Err("Failed to succeed".into())
}

