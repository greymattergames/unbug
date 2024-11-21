use unbug::prelude::*;

fn try_some_option(in_opt: Option<()>) {
    let Some(_out_var) = in_opt else {
        fail_always!(
            "fail_always! can also be formatted - in_opt was: {:?}",
            in_opt
        );
        return;
    };
}

fn main() {
    tracing_subscriber::fmt::init();

    breakpoint!();

    for i in 0..5 {
        ensure!(false);
        ensure!(false, "ensure messages can be logged during development");
        ensure_always!(i % 2 == 0);

        fail!("fail! will happen only once");
        fail!("fail! can also format output - i was: {}", i);
        fail_always!("fail_always! will happen multiple times");

        try_some_option(None);
    }
}
