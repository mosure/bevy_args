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
fn parse_query_string(query: &str) -> HashMap<String, String> {
    query
        .split('&')
        .filter_map(|pair| {
            let mut iter = pair.split('=');
            match (iter.next(), iter.next()) {
                (Some(key), Some(value)) => Some((key.to_string(), value.to_string())),
                _ => None,
            }
        })
        .collect()
}

#[cfg(target_arch = "wasm32")]
fn update_struct<R>(mut instance: R, query_map: HashMap<String, String>) -> R
where
    R: Serialize + for<'de> Deserialize<'de>,
{
    let mut instance_json = serde_json::to_value(&instance).unwrap();

    for (key, value) in query_map {
        if let Some(field) = instance_json.get_mut(&key) {
            *field = serde_json::Value::String(value);
        }
    }

    instance = serde_json::from_value(instance_json).unwrap();
    instance
}


pub fn parse_args<R: Resource + Parser + Serialize + for<'a> Deserialize<'a>>() -> R {
    #[cfg(target_arch = "wasm32")]
    {
        let window = web_sys::window().unwrap();
        let location = window.location();
        let search = location.search().unwrap();

        let query_string = search.trim_start_matches('?');
        let query_map = parse_query_string(query_string);

        let default = R::parse();
        update_struct(default, query_map)
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
        app.insert_resource(parse_args::<R>());
    }
}
