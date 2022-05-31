# Dose

> Your daily dose of structs and functions.

## Features

* Blazingly Fast
* Dependency Injection
* Statically Checked 
* No Runtime Dependency Graph

Unfortunately we do not support runtime dependency cycle detection ... yet.

## Usage

You should call the init macro at the root of your project `lib.rs` to create the trait that will be used by all providers.
This is a current limitation of rust and this may become unnecessary in future versions.

```rust
dose::init!();
```
