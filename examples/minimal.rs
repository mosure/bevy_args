use bevy::prelude::*;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use bevy_args::{
    BevyArgsPlugin,
    Deserialize,
    Parser,
    Serialize,
    ValueEnum,
};


#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}


#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    Serialize,
    Deserialize,
    Reflect,
    ValueEnum,
)]
pub enum MyEnum {
    #[default]
    SomeValue,
    AnotherValue,
}

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

    #[arg(long, value_enum, default_value_t = MyEnum::SomeValue)]
    pub my_enum: MyEnum,
}


pub fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    let mut app = App::new();

    app.add_plugins(BevyArgsPlugin::<MinimalArgs>::default());
    app.add_systems(Startup, print_minimal_args);

    app.run();
}

fn print_minimal_args(args: Res<MinimalArgs>) {
    #[cfg(target_arch = "wasm32")]
    log(format!("{:?}", *args).as_str());

    #[cfg(not(target_arch = "wasm32"))]
    println!("{:?}", *args);
}
