use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
    utils::BoxedFuture,
};
use serde::Deserialize;
#[derive(Debug, Deserialize, TypeUuid)]
#[uuid = "988026cf-8c68-415a-88a5-e515b26c8841"]
pub struct Pattern {
    pub size: [usize; 2],
    pub pattern_pixels: Vec<[u8; 4]>,
}
//TODO: Create a loader for these sometime
impl Pattern {
    pub fn new(pattern_pixels: Vec<[u8; 4]>) -> Self {
        Self {
            pattern_pixels: pattern_pixels,
            size: [0, 0],
        }
    }
}
