use input::InputHandler;
use entity::Entity;
use game_renderer::Renderer;
use glium;
use glium::backend::glutin_backend::GlutinFacade;

fn init_display() -> GlutinFacade {
  use glium::DisplayBuild;
  glium::glutin::WindowBuilder::new()
    .with_gl(glium::glutin::GlRequest::Specific(
        glium::glutin::Api::OpenGl, (3, 0)))
    .build_glium().unwrap()
}

pub struct Engine {
  pub entity_list: Vec<Entity>,
  pub g_renderer: Renderer,
  pub input_handler: InputHandler,
  pub display: GlutinFacade,
}

impl Engine {
  pub fn new() -> Engine {
    use glium::glutin::CursorState;
    let display = init_display();
    display.get_window().unwrap().set_cursor_state(CursorState::Grab).unwrap();
    let g_renderer = Renderer::new(&display);
    Engine { 
      g_renderer: g_renderer, 
      entity_list: Vec::new(),
      display: display,
      input_handler: InputHandler::new(),
    }
  }

  /// Updates the engine. Returns true if the game should stop.
  pub fn update(&mut self) -> bool {
    if self.input_handler.check_input(&self.display) {
      return true;
    }
    for e in &mut self.entity_list {
      e.update();
    }
    return false;
  }

  pub fn render(&mut self) {
    use glium::Surface;
    let mut target = self.display.draw();
    target.clear_color(0.1, 0.1, 0.1, 1.0);
    self.g_renderer.render(&mut target, &self);
    target.finish().unwrap();
  }

  pub fn add_entity(&mut self, e: Entity) {
    self.entity_list.push(e);
  }
}

