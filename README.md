# bevy_args
bevy plugin to parse command line arguments and URL query parameters


## command line arguments
`cargo run --example=minimal -- --my_string hello --my_int 42 --my_bool`

## URL query parameters
`http://localhost:8080/?my_string=hello&my_int=42&my_bool=true`


## minimal example

```rust
use bevy_args::BevyArgsPlugin;


#[derive(
    Default,
    Debug,
    Resource,
    Serialize,
    Deserialize,
    Parser,
)]
#[command(about = "a minimal example of bevy_args", version, long_about = None)]
pub struct MinimalArgs {
    #[arg(long, default_value = "hello")]
    pub my_string: String,

    #[arg(long, default_value = "42")]
    pub my_int: i32,

    #[arg(long)]
    pub my_bool: bool,
}


pub fn main() {
    let mut app = App::new();

    app.add_plugins(BevyArgsPlugin::<MinimalArgs>::default());
    app.add_systems(Startup, print_minimal_args);

    app.run();
}

fn print_minimal_args(args: Res<MinimalArgs>) {
    println!("{:?}", *args);
}
```


## compatible bevy versions

| `bevy_args` | `bevy` |
| :--                       | :--    |
| `1.0`               | `0.12` |
