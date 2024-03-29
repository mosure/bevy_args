use std::marker::PhantomData;

#[cfg(target_arch = "wasm32")]
use std::collections::HashMap;

use bevy::prelude::*;
pub use clap::Parser;
pub use serde::{
    Deserialize,
    Serialize,
};

#[cfg(target_arch = "wasm32")]
use serde_qs::from_str;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;


pub fn parse_args<R: Resource + Parser + Serialize + for<'a> Deserialize<'a>>() -> R {
    #[cfg(target_arch = "wasm32")]
    {
        let window = web_sys::window().unwrap();
        let location = window.location();
        let search = location.search().unwrap();

        let query_string = search.trim_start_matches('?');

        return from_str(query_string).unwrap();
    }

    #[cfg(not(target_arch = "wasm32"))]
    R::parse()
}


pub struct BevyArgsPlugin<R> {
    phantom: PhantomData<fn() -> R>,
}
impl<R> Default for BevyArgsPlugin<R> {
    fn default() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

impl<R: Default + Parser + Resource + Serialize + for<'a> Deserialize<'a>> Plugin for BevyArgsPlugin<R> {
    fn build(&self, app: &mut App) {
        app.insert_resource(R::default());

        app.add_systems(PreStartup, parse_args_system::<R>);
    }
}

fn parse_args_system<R: Resource + Parser + Serialize + for<'a> Deserialize<'a>>(
    mut args: ResMut<R>,
) {
    *args = parse_args::<R>();
}
