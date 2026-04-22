use unbug::prelude::*;

fn main() {
    // Activate tracing before using Unbug
    tracing_subscriber::fmt::init();

    fail!("Log messages will be formatted with tracing");

    let some_data = Some("data");
    ensure!(false, "Interpolated variables are supported: {some_data:?}");
}

