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
    for (key, value) in query_map {
        // Using serde's ability to deserialize from maps to update struct
        let update_map = [(key, value)].iter().cloned().collect::<HashMap<_, _>>();
        let update_str = serde_urlencoded::to_string(update_map).unwrap();
        let update_instance: R = serde_urlencoded::from_str(&update_str).unwrap_or(instance);
        instance = update_instance;
    }
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
