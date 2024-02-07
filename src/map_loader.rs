use std::any::Any;

use tiled::{LayerTile, Loader, Map, Tile};

use crate::constants::STATIC_LAYERS;

pub struct MapLoader {
    pub map: Map,
    collisions: Vec<(u32, u32, u32, u32)>,
}

impl MapLoader {
    /// Load a map and preprocess some data like collisions
    pub fn load_level(level: u32) -> Self {
        let mut loader = Loader::new();
        let mut map = loader
            .load_tmx_map(format!("assets/lvl{level}/map.tmx"))
            .expect("Error while opening the map file");

        // Create a 2D vector which contains true if there is collisions, false if not
        for tileset in map.tilesets() {}

        MapLoader {
            map,
            collisions: vec![],
        }
    }

    pub fn get_tiles(&self, x_start: u32, y_start: u32, size: u32) -> Vec<(u32, u32, LayerTile)> {
        let mut tiles = vec![];

        for layer in self.map.layers() {
            if let Some(layer) = layer.as_tile_layer() {
                for x in x_start..x_start + size {
                    for y in y_start..y_start + size {
                        if let Some(tile) =
                            layer.get_tile(x.try_into().unwrap(), y.try_into().unwrap())
                        {
                            tiles.push((x, y, tile));
                        }
                    }
                }
            }
        }

        tiles
    }
}
