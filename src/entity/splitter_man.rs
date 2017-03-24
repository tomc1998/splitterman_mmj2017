use game_renderer::RendererController;

#[derive(Clone, Debug)]
pub struct SplitterMan {
  pub pos: [f32; 2],

  /// Splitter man's 'split level'. i.e. how many clones he can split into.
  /// Lowest value = 1.
  pub size: u32,
}

impl SplitterMan {
  pub fn new(x: f32, y: f32, size: u32) -> SplitterMan {
    SplitterMan { pos: [x, y], size: size, }
  }

  pub fn update(&mut self) {
  }

  /// Get entity's visual size (radius)
  pub fn get_size(&self) -> f32 {
    (self.size as f32).sqrt() * 8.0
  }

  pub fn render(&self, cont: &mut RendererController) {
    let rad = self.get_size();
    cont.rect(self.pos[0] - rad, self.pos[1] - rad, rad*2.0, rad*2.0);
  }
}
