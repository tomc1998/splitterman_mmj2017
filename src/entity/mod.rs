pub mod splitter_man;

use self::splitter_man::SplitterMan;
use game_renderer::RendererController;
use input::InputHandler;

#[derive(Clone, Debug)]
pub enum Entity {
  SplitterMan(SplitterMan),
}

impl Entity {
  pub fn update(&mut self, input: &InputHandler) {
    match *self {
      Entity::SplitterMan(ref mut e) => e.update(input),
    }
  }

  pub fn render(&self, cont: &mut RendererController) {
    match *self {
      Entity::SplitterMan(ref e) => e.render(cont),
    }
  }
}

