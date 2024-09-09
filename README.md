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

## Examples

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

let Some(out_var) = my_var else {
    // fail! pauses and logs an error message, will also only trigger once
    // fail! will continue to log in non-debug builds
    unbug::fail!("failed to get some option");
    return;
};

let Some(other_out_var) = my_var else {
    // fail_always! will do the same, but will trigger every time
    unbug::fail_always!("failed to get some option");
    return;
};
```

## Installation

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