pub mod splitter_man;

use self::splitter_man::SplitterMan;
use game_renderer::{RendererController, Renderer};
use input::InputHandler;
use engine::Engine;
use std::cell::Cell;

#[derive(Clone, Copy, Debug)]
pub struct EHandle(pub u32);

#[derive(Clone, Copy, Debug)]
pub enum Entity {
  SplitterMan(SplitterMan),
}

impl Entity {
  /// Returns a tuple. 
  /// # 1: True if this entity should be removed after the update.
  /// # 2: A list of entities to add after the update.
  pub fn update(&mut self, input: &InputHandler, 
                renderer: &Renderer, engine: &Engine) -> (bool, Option<Vec<Entity>>) {
    match *self {
      Entity::SplitterMan(ref mut e) => e.update(input, renderer, engine),
    }
  }

  pub fn render(&self, cont: &mut RendererController) {
    match *self {
      Entity::SplitterMan(ref e) => e.render(cont),
    }
  }

  pub fn get_entity_handle(&self) -> Option<EHandle> {
    match *self {
      Entity::SplitterMan(ref e) => e.id,
    }
  }

  pub fn set_entity_handle(&mut self, h: EHandle) {
    match *self {
      Entity::SplitterMan(ref mut e) => e.id = Some(h),
    }
  }
}

