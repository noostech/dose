# Dose

> Your daily dose of structs and functions.

## Features

* Simplicity 👌
* Blazingly Fast 🏎️💨
* Lazy Instantiation 🦥
* Statically Checked 🕵️‍♀️
* Dependency Injection 💉
* Declarative Interface  ✨ 
* No Runtime Dependency Graph 🎉

Unfortunately we do not support runtime dependency cycle detection ... yet.

## Usage

### Initialization

You need to initialize the crate with the `init!` macro at the root of your project in `lib.rs`.
This is a current limitation of rust and this may become unnecessary in future versions.

```rust
dose::init!();
```

### Registering Providers

You can register providers with the `provide` attribute.

```rust
#[provide]
fn provide_something(context: &mut Context<Config>) -> Something {
    Something::new(...)
}
```

The `&mut Context<Config>` is always passed as argument and can be used to resolve other types.
The config is also available from the context `context.config`.

Each time the type `Something` is `get!`, the function above will be called.
If you want the same instance to be used, the attribute parameter `singleton` needs to be set to `true`.

```rust
#[provide(singleton=true)]
fn provide_something(context: &mut Context<Config>) -> Something {
    Something::new(...)
}
```

In this case, each time the type `Something` is `get!`, the instance will be cloned.
This really becomes a singleton if type `Something` is wrapped in an `std::sync::Arc` or a `std::rc::Rc`.

```rust
#[provide(singleton=true)]
fn provide_something(context: &mut Context<Config>) -> Arc<Something> {
    Arc::new(Something::new(...))
}
```

### Resolving Instances

The `get!` macro is the way to get the instance of a type.

```rust
let mut context = dose::Context::new(config);
let my_type: MyType = dose::get!(context);
```

This macro can also be used inside providers.

```rust
struct MyType {
    a: TypeA,
    b: TypeB,
}

#[provide]
fn provide_my_type(context: &mut Context<Config>) -> MyType {
    MyType {
        a: get!(context), // Type infered to TypeA
        b: get!(context), // Type infered to TypeB
    }
}
```

Note that if the provider of `TypeA` or `TypeB` is not registered, a compilation error will occur.

## Good Practices

### Module Separation

Providers should be defined in modules specifically created for declaring how each type is instantiated.

### Context Lifecycle

The `Context<Config>` should be deleted (`drop`) before the application is started.

Using out of scope.
```rust
use dose::{Context, get}

... // declare all modules

fn create_server(config: Config) -> Server {
    let mut context = Context::new(config);
    get!(context) 
}

#[tokio::main]
async fn start_server_implicit() {
    let server = create_server(Config::load(...));
    // Implicitly drop the context because it becomes out of scope
    server.start().await
}

#[tokio::main]
async fn start_server_explicit() {
    let mut context = Context::new(config);
    let server: Server = get!(context);
    // Explicitly drop the context using std::mem::drop
    std::mem::drop(context);
    server.start().await
}

```

### Providers Single Purpose

Keep providers simple, only creation logic should be in there.
No algorithms or domain logic should be executed.

### Config

The config provided in the context can be anything, but a simple struct without functions and public fields can be used.
Each provider should only require the context and the config to execute properly.
