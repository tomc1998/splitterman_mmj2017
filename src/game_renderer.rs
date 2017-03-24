use glium;
use glium::backend::glutin_backend::GlutinFacade;

use engine::Engine;
use shader::make_program;

#[derive(Copy, Clone)]
struct Vertex {
  pos: [f32; 2],
  /// R, G, B, A
  col: [f32; 4],
}
impl Vertex {
  #[inline(always)]
  fn new(x: f32, y: f32, r: f32, g: f32, b: f32, a: f32) -> Vertex {
    Vertex { pos: [x, y], col: [r, g, b, a]}
  }
}
implement_vertex!(Vertex, pos, col);

/// Controller for the renderer. Contains convenience methods for vertex data.
pub struct RendererController {
  data: Vec<Vertex>,
}

impl RendererController {
  pub fn new(buf_cap: usize) -> RendererController {
    RendererController { data: Vec::with_capacity(buf_cap), }
  }

  /// Create rectangle draw data and add it to the buffer.
  pub fn rect(&mut self, x: f32, y: f32, w: f32, h: f32, r: f32, g: f32, b: f32, a: f32) {
    self.data.push(Vertex::new(x, y, r, g, b, a));
    self.data.push(Vertex::new(x+w, y, r, g, b, a));
    self.data.push(Vertex::new(x+w, y+h, r, g, b, a));
    self.data.push(Vertex::new(x, y, r, g, b, a));
    self.data.push(Vertex::new(x, y+h, r, g, b, a));
    self.data.push(Vertex::new(x+w, y+h, r, g, b, a));
  }
}

pub struct Renderer {
  program: glium::Program,
  vbo: glium::VertexBuffer<Vertex>,
  proj_mat: [[f32; 4]; 4],
  hud_vbo: glium::VertexBuffer<Vertex>,
  hud_proj_mat: [[f32; 4]; 4],
}

impl Renderer {
  pub fn new(display: &GlutinFacade) -> Renderer {
    let (w, h) = display.get_window().unwrap().get_inner_size().unwrap();

    Renderer {
      program: make_program(display),
      vbo: glium::VertexBuffer::empty_dynamic(display, 1024).unwrap(),
      hud_vbo: glium::VertexBuffer::empty_dynamic(display, 1024).unwrap(),
      proj_mat: 
      [[2.0/w as f32, 0.0,           0.0, -0.0],
       [0.0,         -2.0/h as f32,  0.0,  0.0],
       [0.0,          0.0,          -1.0,  0.0],
      [-1.0,          1.0,           0.0,  1.0]],
      hud_proj_mat: 
      [[2.0/w as f32, 0.0,           0.0, -0.0],
       [0.0,         -2.0/h as f32,  0.0,  0.0],
       [0.0,          0.0,          -1.0,  0.0],
      [-1.0,          1.0,           0.0,  1.0]],
    }
  }

  pub fn render(&self, target: &mut glium::Frame, engine: &Engine) {
    use glium::Surface;

    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let params = glium::DrawParameters {
      blend: glium::Blend::alpha_blending(),
      .. Default::default()
    };

    // Render entities and tiles
    let mut controller = RendererController::new(self.vbo.len());

    for e in &engine.entity_list {
      e.render(&mut controller);
    }

    if controller.data.len() == 0 { return }
    while controller.data.len() > self.vbo.len() { controller.data.pop(); }
    while controller.data.len() < self.vbo.len() { controller.data.push(Vertex::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0)); }
    self.vbo.invalidate();
    self.vbo.slice(0..controller.data.len()).unwrap().write(&controller.data);

    let uniforms = uniform! {
      proj_mat: self.proj_mat,
    };

    target.draw(&self.vbo,
                &indices, 
                &self.program, 
                &uniforms,
                &params).unwrap();

    // Render HUD
    controller = RendererController::new(self.hud_vbo.len());

    // Selection box
    if engine.input_handler.curr_box.is_some() {
      let b = engine.input_handler.curr_box.as_ref().unwrap();
      controller.rect(b[0], b[1], b[2] - b[0], b[3] - b[1], 0.0, 1.0, 1.0, 0.4);
    }

    // Mouse
    let m = engine.input_handler.mouse_pos;
    controller.rect(m.0 as f32, m.1 as f32, 4.0, 4.0, 1.0, 1.0, 1.0, 1.0);

    if controller.data.len() == 0 { return }
    while controller.data.len() > self.hud_vbo.len() { controller.data.pop(); }
    while controller.data.len() < self.vbo.len() { controller.data.push(Vertex::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0)); }
    self.vbo.invalidate();
    self.hud_vbo.slice(0..controller.data.len()).unwrap().write(&controller.data);


    let uniforms = uniform! {
      proj_mat: self.hud_proj_mat,
    };

    target.draw(&self.hud_vbo,
                &indices, 
                &self.program, 
                &uniforms,
                &params).unwrap();


  }
}
