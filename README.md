# BUG

## Repro

Run

```
cargo build
```

Check `my-crate/src/generated/sample.rs` and see the following formatting:

```rust
#[allow(dead_code)]
#[allow(clippy::uninlined_format_args)]
impl BlockStorageAllocateBlockAsyncCallbackResult {
    fn decode(src: u32) -> Result<Self, DecodeError> {
        let result = match src {
            1 => BlockStorageAllocateBlockAsyncCallbackResult::Ok,
            2 => BlockStorageAllocateBlockAsyncCallbackResult::AlreadyExists,
            3 => BlockStorageAllocateBlockAsyncCallbackResult::Error,
            _ => return Err(DecodeError::new(format!(
                "Decode BlockStorageAllocateBlockAsyncCallbackResult: Unsupported enum value: {}",
                src
            ))),
        };
        Ok(result)
    }
}
```

Run

```
cargo fmt
```

Check `my-crate/src/generated/sample.rs` and see the following formatting:

```rust
#[allow(dead_code)]
#[allow(clippy::uninlined_format_args)]
impl BlockStorageAllocateBlockAsyncCallbackResult {
    fn decode(src: u32) -> Result<Self, DecodeError> {
        let result = match src {
            1 => BlockStorageAllocateBlockAsyncCallbackResult::Ok,
            2 => BlockStorageAllocateBlockAsyncCallbackResult::AlreadyExists,
            3 => BlockStorageAllocateBlockAsyncCallbackResult::Error,
            _ => {
                return Err(DecodeError::new(format!(
                "Decode BlockStorageAllocateBlockAsyncCallbackResult: Unsupported enum value: {}",
                src
            )))
            }
        };
        Ok(result)
    }
}
```

The same output is produced when running `rustfmt fmt_bug\my-crate\src\generated\sample.rs --verbose`

But when doing this from `build.rs` in a workspace, it produces the first output above.
