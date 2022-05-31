# Dose

> Your daily dose of structs and functions.

## Usage

You should call the init macro at the root of your project `lib.rs` to create the trait that will be used by all providers.
This is a current limitation of rust and this may become unnecessary in future versions.

```rust
dose::init!();
```
