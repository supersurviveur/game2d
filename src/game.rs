use sfml::{
    graphics::{RenderTarget, RenderWindow},
    window::Event,
};

use crate::{map_loader::MapLoader, map_manager::MapManager};

pub struct Game<'a> {
    window: RenderWindow,
    map_manager: MapManager<'a>
}

impl<'a> Game<'a> {
    pub fn new() -> Self {
        let mut window = RenderWindow::new(
            (1200, 800),
            "Game2D",
            sfml::window::Style::DEFAULT,
            &Default::default(),
        );
        window.set_vertical_sync_enabled(true);

        Game {
            window,
            map_manager: MapManager::new()
        }
    }
    pub fn run(&'a mut self) {
        self.map_manager.generate_chunks();
        while self.window.is_open() {
            while let Some(ev) = self.window.poll_event() {
                match ev {
                    Event::Closed => self.window.close(),
                    Event::Resized { width, height } => {
                        // Resize the view to the window size
                        self.window.set_view(&sfml::graphics::View::new(
                            sfml::system::Vector2::new(width as f32 / 2.0, height as f32 / 2.0),
                            sfml::system::Vector2::new(width as f32, height as f32),
                        ));
                    }
                    _ => {}
                }
            }
            self.window.clear(sfml::graphics::Color::BLUE);
            self.map_manager.draw(&self.window);
            self.window.display();
        }
    }
}
