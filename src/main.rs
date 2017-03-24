#[macro_use]
extern crate glium;

/// Module to handle rendering entities and tiles.
pub mod game_renderer;
/// Handles shader programs. Not sure if I need this, just nice to keep all the
/// hard coded shader source out the way.
pub mod shader;

pub mod entity;
pub mod engine;

use entity::Entity;
use entity::splitter_man::SplitterMan;


fn main() {
  // Create engine
  let mut engine = engine::Engine::new();

  // Add test entity
  engine.add_entity(Entity::SplitterMan(SplitterMan::new(100.0, 100.0, 16)));

  loop {
    if engine.update() { return }
    engine.render();
  }
}
