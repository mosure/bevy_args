use std::marker::PhantomData;

#[cfg(target_arch = "wasm32")]
use std::collections::HashMap;

use bevy::prelude::*;
use clap::Parser;
use serde::{
    Deserialize,
    Serialize,
};

#[cfg(target_arch = "wasm32")]
use serde_qs::from_str;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;


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

        app.add_systems(PreStartup, parse_args::<R>);
    }
}


fn parse_args<R: Resource + Parser + Serialize + for<'a> Deserialize<'a>>(
    mut args: ResMut<R>,
) {
    #[cfg(target_arch = "wasm32")]
    {
        let window = web_sys::window().unwrap();
        let location = window.location();
        let search = location.search().unwrap();

        let query_string = search.trim_start_matches('?');

        *args = from_str(query_string).unwrap();
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        *args = R::parse();
    }
}
