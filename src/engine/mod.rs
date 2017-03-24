use std::cell::Cell;
use input::InputHandler;
use entity::Entity;
use game_renderer::Renderer;
use glium;
use glium::backend::glutin_backend::GlutinFacade;
use entity::EHandle;
use time;

pub mod common;

pub use self::common::vec::Vec2f32;

fn init_display() -> GlutinFacade {
  use glium::DisplayBuild;
  glium::glutin::WindowBuilder::new()
    .with_gl(glium::glutin::GlRequest::Specific(
        glium::glutin::Api::OpenGl, (3, 0)))
    .build_glium().unwrap()
}

pub struct Engine {
  pub entity_list: Vec<Cell<Entity>>,
  pub g_renderer: Renderer,
  pub input_handler: InputHandler,
  pub display: GlutinFacade,

  last_update_nanos: u64,
  frame_delta: u64,
  /// The amount of cumulative nanos passed since last frame
  nanos_cumul: u64, 
  /// Size that nanos_cumul needs to be to run a frame
  nanos_per_frame: u64, 

  /// Flag to say whether we should render this frame. Set to true when the
  /// entity updates (to sync with frame limiting)
  should_render: bool,

  last_ehandle: EHandle,
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
      last_ehandle: EHandle(0),

      last_update_nanos: 0,
      frame_delta: 0,
      nanos_cumul: 0,
      nanos_per_frame: 16666666, // 60 FPS
      should_render: false,
    }
  }

  /// Update the counter time and delta in LibState.
  fn update_delta(&mut self) {
    let now = time::precise_time_ns();
    if self.last_update_nanos != 0 {
      self.frame_delta = now - self.last_update_nanos;
    }
    self.nanos_cumul += self.frame_delta;
    self.last_update_nanos = now;
  }

  /// Updates the engine. Returns true if the game should stop.
  pub fn update(&mut self) -> bool {
    self.update_delta();
    if self.nanos_cumul < self.nanos_per_frame { return false; }
    self.nanos_cumul -= self.nanos_per_frame;
    self.should_render = true;

    if self.input_handler.check_input(&self.display) {
      return true;
    }
    let mut ents = Vec::new(); // Entities to append to the entity list at the end of the loop
    let mut to_remove = Vec::new();
    for e in &self.entity_list {
      let mut e_copy = e.get();
      let (remove, mut _ents) = e_copy.update(&*self);
      e.set(e_copy);
      if _ents.is_some() {
        ents.append(&mut _ents.unwrap());
      }
      if remove { to_remove.push(e_copy.get_entity_handle().unwrap()); }
    }
    for e in ents {
      self.add_entity(e);
    }
    return false;
  }

  pub fn render(&mut self) {
    use glium::Surface;
    if !self.should_render { return; }
    self.should_render = false;
    self.g_renderer.update_proj_mat();
    let mut target = self.display.draw();
    target.clear_color(0.1, 0.1, 0.1, 1.0);
    self.g_renderer.render(&mut target, self);
    target.finish().unwrap();
  }

  fn gen_entity_id(&mut self) -> EHandle {
    self.last_ehandle.0 += 1;
    return EHandle(self.last_ehandle.0);
  }

  pub fn add_entity(&mut self, mut e: Entity) {
    e.set_entity_handle(self.gen_entity_id());
    self.entity_list.push(Cell::new(e));
  }
}

