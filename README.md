
# Unbug

A crate to programmatically invoke debugging breakpoints with helping macros.

These macros are designed to help developers catch errors during debugging sessions that would otherwise be a panic (which may not be desirable in certain contexts) or simply a log message (which may go unnoticed).

This crate's internals are disabled by default, there are shims provided so breakpoints will not be compiled outside of a debugging context. This means that the macros in this crate can be used freely throughout your code without having to conditionally compile them out yourself.

> ### NOTICE
>
> You must use the `enable` feature of this crate (deactivated by default) to activate the breakpoints. This crate cannot detect the presence of a debugger.
>
> __BREAKPOINTS REQUIRE NIGHTLY RUST__
>
> __BREAKPOINTS REQUIRE ENABLING THE EXPERIMENTAL [`core_intrinsics`](https://doc.rust-lang.org/core/intrinsics/fn.breakpoint.html) FEATURE__
>
> Additonally, debugging may not land on the macro statements themselves. This can have the consequence that the debgger may pause on an internal module. To avoid this, `return` or `continue` immediately following a macro invocation. Alternatively, use your debugger's "step-out" feature until you reenter the scope of your code.

Error messages are logged when used in conjuction with [Tracing](https://github.com/tokio-rs/tracing)

## Examples

# [![VSCode debugging example](https://raw.githubusercontent.com/greymattergames/unbug/master/assets/debug.png)](https://github.com/greymattergames/unbug/blob/master/examples/basic/src/main.rs)

```rust
// trigger the debugger
unbug::breakpoint!();

let some_bool = false;

for i in 0..5 {
    // ensure! will only trigger the debugger once when the expression argument is false
    unbug::ensure!(some_bool);

    // ensure_always! will trigger the debugger every time
    unbug::ensure_always!(some_bool);
}

let my_var: Option<()> = None;

// Use the tracing_subscriber crate to log error messages from the fail! and fail_always! macros.
tracing_subscriber::fmt::init();

if my_var == None {
    // fail! pauses and logs an error message, will also only trigger once
    // fail! will continue to log in non-debug builds
    unbug::fail!("failed to get my_var");
    return;
}

let Some(other_out_var) = my_var else {
    // fail_always! will do the same, but will trigger every time
    // fail! and fail_always! can be formatted just like error! from the Tracing crate
    unbug::fail_always!("failed to get my_var: {:?}", my_var);
    return;
};
```

## Setup

Prepare your environment for debugging Rust.
> If you are using VSCode you will need the [Rust Analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) and [Code LLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb) extensions. [See Microsoft's Documentation on Rust Debugging in VSCode](https://code.visualstudio.com/docs/languages/rust#_debugging).

__1.__ Enable Nightly Rust:
```bash
rustup install nightly
rustup default nightly
```

__2.__ Create a debug feature in your project that will only be active in the context of a debugger, i.e. not enabled by default.

`Cargo.toml`:
```toml
[features]
default = []
my_debug_feature = [
    "unbug/enable"
]
```

__3.__ enable the core_intrinsics feature in the root of your crate (`main.rs` or `lib.rs`)

`main.rs`:
```rust
#![cfg_attr(all(debug_assertions, feature = "my_debug_feature"), feature(core_intrinsics))]
```
> *this configuration will only activate core_intrinsics for debug builds when your debug feature is active*

__4.__ Pass your feature flag to cargo during your debug build.

Sample VSCode `launch.json` with LLDB:
```json
{
    "version": "0.2.0",
    "configurations": [
        {
            "type": "lldb",
            "request": "launch",
            "name": "Debug",
            "cargo": {
                "args": [
                    "build",
                    "--bin=my_project",
                    "--package=my_project",
                    "--features=my_debug_feature"
                ],
                "filter": {
                    "name": "my_project",
                    "kind": "bin"
                }
            },
            "args": [],
            "cwd": "${workspaceFolder}",
            "env": {
                "CARGO_MANIFEST_DIR": "${workspaceFolder}"
            }
        }
    ]
}
```

## License

Unbug is free and open source. All code in this repository is dual-licensed under either:

- MIT License ([LICENSE-MIT](/LICENSE-MIT) or <http://opensource.org/licenses/MIT>)
- Apache License, Version 2.0 ([LICENSE-APACHE](/LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)

at your option.