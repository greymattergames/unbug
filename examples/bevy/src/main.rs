use unbug::prelude::*;

use bevy_app::{App, AppExit, Startup};
use bevy_ecs::{error::{Result, ignore}, message::MessageWriter, schedule::IntoScheduleConfigs};
use bevy_log::{LogPlugin, Level};

fn main() -> AppExit {
    let mut app = App::new();

    // Since you're using fancy debug tooling
    // you probably don't want the default error handler
    app.set_error_handler(ignore);

    // Set up logging which unbug will integrate with through tracing
    app.add_plugins(LogPlugin {
        level: Level::INFO,
        ..Default::default()
    });

    // (Project scaffolding)
    app.add_systems(Startup, (
        fail_system,
        msg_system,
        ignore_system,
        standard_system,
        end,
    ).chain());

    app.run()
}

fn get_result(succeed: bool) -> Result<String> {
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

// Annotate your failable systems with `debug_fail` to get unbug integration
#[debug_fail]
fn fail_system() -> Result {
    // Any invocation of a try-operator (?) will trigger a breakpoint on failure
    let _important_data = get_result(false)?;

    Ok(())
}

#[debug_fail]
fn msg_system() -> Result {
    // Use the fail_msg annotaion to log messages with breakpoints
    // These messages will also be present in release builds
    #[fail_msg = "A message to log when this fails"]
    // unbug's Bevy integration handles Option types
    // by converting them to Result
    let _important_data = get_option(false)?;

    Ok(())
}

#[debug_fail]
fn ignore_system() -> Result {
    // You can skip creating a breakpoint or logging a message with the fail_ignore annotation
    #[fail_ignore]
    let _important_data = get_result(false)?;

    Ok(())
}

fn standard_system() {
    // standard infailable systems don't need debug_fail

    for _ in 0..1 {
        let Some(_important_data) = get_option(true) else {
            // in the case of loops, it's most likely better to use a let-else expression
            // and continue instead of invoking the try-operator so errors do not short
            // circuit other iterations of the loop
            continue;
        };
    }

    for _ in 0..1 {
        // If you want to use the try-operator (?) in a loop
        // an inner helper function can make things easier
        if inner_worker().is_err() {
            fail!("Standard unbug fail assertions are available too");
        }
    }
}

#[debug_fail]
fn inner_worker() -> Result {
    #[fail_msg = "Any function that returns a Result can be annotated with debug_fail"]
    let _important_data = get_result(false)?;

    Ok(())
}

fn end(mut writer: MessageWriter<AppExit>) {
    writer.write(AppExit::Success);
}
