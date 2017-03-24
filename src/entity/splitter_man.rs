use game_renderer::RendererController;
use input::InputHandler;

#[derive(Clone, Debug)]
pub struct SplitterMan {
  pub pos: [f32; 2],

  /// Splitter man's 'split level'. i.e. how many clones he can split into.
  /// Lowest value = 1.
  pub size: u32,

  /// Has the player selected this splitter man?
  pub selected: bool,
}

impl SplitterMan {
  pub fn new(x: f32, y: f32, size: u32) -> SplitterMan {
    SplitterMan { pos: [x, y], size: size, selected: false, }
  }

  pub fn update(&mut self, input: &InputHandler) {
    if input.selection.is_some() {
      println!("Hey");
      // Test for collision between input rect and splitterman rect
      let rad = self.get_size();
      let sel = input.selection.unwrap();
      if self.pos[0] - rad < sel[2] 
        && self.pos[0] + rad > sel[0] 
          && self.pos[1] - rad < sel[3] 
          && self.pos[1] + rad > sel[1] {
            self.selected = true;
          }
      else {
        self.selected = false;
      }
    }
  }

  /// Get entity's visual size (radius)
  pub fn get_size(&self) -> f32 {
    (self.size as f32).sqrt() * 8.0
  }

  pub fn render(&self, cont: &mut RendererController) {
    let rad = self.get_size();
    let c : (f32, f32, f32, f32);
    if self.selected {
      c = (0.0, 1.0, 1.0, 1.0);
    }
    else {
      c = (1.0, 0.0, 0.0, 1.0);
    }
    cont.rect(self.pos[0] - rad, self.pos[1] - rad, rad*2.0, rad*2.0, c.0, c.1, c.2, c.3);
  }
}
