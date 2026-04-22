use unbug::prelude::*;

fn try_some_option(in_opt: Option<()>) {
    let Some(_out_var) = in_opt else {
        fail_always!("fail_always! can also be formatted {:?}", in_opt);
        return;
    };
}

fn main() {
    // tracing_subscriber::fmt::init();

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

// In order to have unbug panic in tests, you will need to activate the `testing` feature,
// See this example's Cargo.toml
#[cfg(test)]
mod test {
    #![allow(clippy::assertions_on_constants)]

    use super::*;

    #[test]
    #[should_panic]
    fn main_should_panic() {
        main();
    }

    #[test]
    #[should_panic]
    fn try_some_option_should_panic() {
        try_some_option(None);
    }

    #[test]
    #[should_panic]
    fn breakpoint_should_panic_in_test() {
        breakpoint!();
    }

    #[test]
    #[should_panic]
    fn ensure_should_panic_in_test() {
        ensure!(false, "ensure should panic");
    }

    #[test]
    #[should_panic]
    fn ensure_always_should_panic_in_test() {
        ensure_always!(false, "ensure_always should panic");
    }

    #[test]
    #[should_panic]
    fn fail_should_panic_in_test() {
        fail!("fail should panic");
    }

    #[test]
    #[should_panic]
    fn fail_always_should_panic_in_test() {
        fail_always!("fail_always should panic");
    }
}
