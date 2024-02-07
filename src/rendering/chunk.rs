use sfml::{
    graphics::{
        Color, Image, PrimitiveType, RenderStates, RenderTarget, RenderTexture, Texture, Vertex,
        VertexBuffer, VertexBufferUsage,
    },
    system::Vector2f,
    SfBox,
};
use tiled::LayerTile;

use crate::{
    constants::{CHUNK_SIZE, TILE_SIZE},
    map_loader::MapLoader,
};

pub struct Chunk<'a> {
    position: (u32, u32),
    /// Position of the chunk in the map (in chunks)
    size: u32,
    tiles: Vec<(u32, u32, LayerTile<'a>)>,
    image: Image,
    pub texture: RenderTexture,
}

impl<'a> Chunk<'a> {
    pub fn new(position: (u32, u32), size: u32) -> Self {
        // Initialize an texture with the size of the chunk
        let mut image = Image::new(size * TILE_SIZE, size * TILE_SIZE);
        let texture = RenderTexture::new(size * TILE_SIZE, size * TILE_SIZE).unwrap();

        Chunk {
            position,
            size,
            tiles: vec![],
            image,
            texture,
        }
    }

    /// Generate the chunk :
    /// - Load the tiles and put them in the texture
    pub fn generate(&mut self, map_loader: &'a MapLoader) {
        self.tiles = map_loader.get_tiles(
            self.position.0 * CHUNK_SIZE,
            self.position.1 * CHUNK_SIZE,
            CHUNK_SIZE,
        );
    }

    pub fn draw(&self, target: &dyn RenderTarget) {
        for (x, y, tile) in &self.tiles {
            let tileset = tile.get_tile().unwrap().tileset();
            let tileset_image = tileset.image.as_ref().unwrap();
            let tileset_image = Image::from_file(tileset_image.source.to_str().unwrap()).unwrap();

            let tile_x = tileset.margin
                + (tileset.spacing + tileset.tile_width) * (tile.id() % tileset.columns);
            let tile_y = tileset.margin
                + (tileset.spacing + tileset.tile_height) * (tile.id() / tileset.columns);

            // create a vertex array to render the tile inside the texture
            let mut vertexs = VertexBuffer::new(PrimitiveType::QUADS, 4, VertexBufferUsage::STATIC);
            vertexs.update(
                &[Vertex::new(
                    Vector2f::new(
                        ((x - self.position.0 * CHUNK_SIZE) * TILE_SIZE) as f32,
                        ((y - self.position.1 * CHUNK_SIZE) * TILE_SIZE) as f32,
                    ),
                    Color::rgb(255, 0, 0),
                    Vector2f::new(0.0, 0.0),
                )],
                0,
            );
            self.texture
                .draw_vertex_buffer(&vertexs, &RenderStates::DEFAULT);
        }
    }
}
