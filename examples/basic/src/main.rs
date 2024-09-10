#![allow(internal_features)]
#![cfg_attr(all(debug_assertions, feature = "dev_debug"), feature(core_intrinsics))]

use unbug::prelude::*;

fn fail_once(in_opt: Option<()>) {
    if in_opt == None {
        fail!("failed to get in_opt");
        return;
    }
}

fn fail_always(in_opt: Option<()>) {
    let Some(_out_var) = in_opt else {
        fail_always!("failed to get in_opt: {:?}", in_opt);
        return;
    };
}

fn main() {
    tracing_subscriber::fmt::init();

    breakpoint!();

    let _some_bool = false;

    for _i in 0..5 {
        ensure!(_some_bool);
        ensure_always!(_some_bool);

        fail!("standalone fail");

        fail_once(None);
        fail_always(None);
    }
}