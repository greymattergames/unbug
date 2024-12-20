use unbug::prelude::*;

fn try_some_option(in_opt: Option<()>) {
    let Some(_out_var) = in_opt else {
        fail_always!("fail_always! can also be formatted {:?}", in_opt);
        return;
    };
}

fn main() {
    tracing_subscriber::fmt::init();

    breakpoint!();

    for i in 0..5 {
        ensure!(false);
        ensure!(false, "ensure messages can be logged during development");
        ensure!(false, "ensure will only break once per false result");
        ensure!(i % 2 == 0, "ensure messages can be formatted {}", i,);

        ensure_always!(false);
        ensure_always!(false, "ensure_always will happen multiple times");
        ensure_always!(false, "ensure_always messages can be formatted {}", i);

        fail!("fail! will happen only once");
        fail!("fail! can also format output {}", i);

        fail_always!("fail_always! will happen multiple times");

        try_some_option(None);
    }
}
