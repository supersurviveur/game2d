#![allow(
    dead_code,
    unused_imports,
    unused_variables,
    unused_mut,
    unused_macros
)]

mod game;
mod map_loader;
mod map_manager;
mod rendering;
pub mod constants;

fn main() {
    println!("Hello, world!");

    let mut game = game::Game::new();
    game.run();
}
