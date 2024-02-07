use sfml::{graphics::{RenderTarget, RenderWindow}, window};

use crate::{constants::{CHUNK_SIZE, TILE_SIZE}, map_loader::MapLoader, rendering::chunk::Chunk};

pub struct MapManager<'a> {
    position: (u32, u32),
    pub map_loader: MapLoader,
    chunks: Vec<Chunk<'a>>,
}

impl<'a> MapManager<'a> {
    pub fn new() -> Self {
        MapManager {
            position: (0, 0),
            map_loader: MapLoader::load_level(0),
            chunks: vec![],
        }
    }

    pub fn generate_chunks(&'a mut self) {
        for y in 0..self.map_loader.map.height / CHUNK_SIZE + 1 {
            for x in 0..self.map_loader.map.width / CHUNK_SIZE + 1 {
                let mut chunk = Chunk::new((x, y), CHUNK_SIZE);
                println!("{} {}", y, x);
                chunk.generate(&self.map_loader);
                self.chunks.push(chunk);
            }
        }
    }

    /// Render the map
    pub fn draw(&mut self, window: &RenderWindow) {
        for chunk in &self.chunks {
            // let mut vertexs = VertexBuffer::new(PrimitiveType::QUADS, 4, VertexBufferUsage::STATIC);
            // vertexs.update(
            //     &[Vertex::new(
            //         Vector2f::new(
            //             ((x - self.position.0 * CHUNK_SIZE) * TILE_SIZE) as f32,
            //             ((y - self.position.1 * CHUNK_SIZE) * TILE_SIZE) as f32,
            //         ),
            //         Color::rgb(255, 0, 0),
            //         Vector2f::new(0.0, 0.0),
            //     )],
            //     0,
            // );
            // self.texture.draw_vertex_buffer(&vertexs, &RenderStates::DEFAULT);
            chunk.draw(window);
        }
    }
}
