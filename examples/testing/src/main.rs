use unbug::prelude::*;

fn main() {
    fail!("I had a failure");
}

// In order to have unbug panic in tests, you will need to activate the `testing` feature,
// See this example's Cargo.toml
#[cfg(test)]
// inline test runner will fail in VS Code unless you change rust analyzer's test setting
// see .vscode/settings.json
mod test {
    use super::*;

    #[test]
    #[should_panic]
    fn main_should_panic() {
        main();
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
