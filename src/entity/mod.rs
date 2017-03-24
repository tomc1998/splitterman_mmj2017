pub mod splitter_man;

use self::splitter_man::SplitterMan;
use game_renderer::RendererController;

#[derive(Clone, Debug)]
pub enum Entity {
  SplitterMan(SplitterMan),
}

impl Entity {
  pub fn update(&mut self) {
    match *self {
      Entity::SplitterMan(ref mut e) => e.update(),
    }
  }

  pub fn render(&self, cont: &mut RendererController) {
    match *self {
      Entity::SplitterMan(ref e) => e.render(cont),
    }
  }
}

