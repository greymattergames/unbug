#![cfg_attr(all(debug_assertions, feature = "dev_debug"), feature(core_intrinsics))]

fn fail_once(in_opt: Option<()>) {
    let Some(_out_var) = in_opt else {
        unbug::fail!("fail error message");
        return;
    };
}

fn fail_always(in_opt: Option<()>) {
    let Some(_out_var) = in_opt else {
        unbug::fail_always!("fail always error message");
        return;
    };
}

fn main() {
    tracing_subscriber::fmt::init();

    unbug::breakpoint!();

    let _some_bool = false;

    for _i in 0..5 {
        unbug::ensure!(_some_bool);
        unbug::ensure_always!(_some_bool);

        fail_once(None);
        fail_always(None);
    }
}