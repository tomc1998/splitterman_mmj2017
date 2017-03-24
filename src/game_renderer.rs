use glium;
use glium::backend::glutin_backend::GlutinFacade;

use engine::Engine;
use shader::make_program;

#[derive(Copy, Clone)]
struct Vertex {
  pos: [f32; 2],
}
impl Vertex {
  #[inline(always)]
  fn new(x: f32, y: f32) -> Vertex {
    Vertex { pos: [x, y], }
  }
}
implement_vertex!(Vertex, pos);

/// Controller for the renderer. Contains convenience methods for vertex data.
pub struct RendererController {
  data: Vec<Vertex>,
}

impl RendererController {
  pub fn new(buf_cap: usize) -> RendererController {
    RendererController { data: Vec::with_capacity(buf_cap), }
  }

  /// Create rectangle draw data and add it to the buffer.
  pub fn rect(&mut self, x: f32, y: f32, w: f32, h: f32) {
    self.data.push(Vertex::new(x, y));
    self.data.push(Vertex::new(x+w, y));
    self.data.push(Vertex::new(x+w, y+h));
    self.data.push(Vertex::new(x, y));
    self.data.push(Vertex::new(x, y+h));
    self.data.push(Vertex::new(x+w, y+h));
  }
}

pub struct Renderer {
  program: glium::Program,
  vbo: glium::VertexBuffer<Vertex>,
  proj_mat: [[f32; 4]; 4],
}

impl Renderer {
  pub fn new(display: &GlutinFacade) -> Renderer {
    let (w, h) = display.get_window().unwrap().get_inner_size().unwrap();

    Renderer {
      program: make_program(display),
      vbo: glium::VertexBuffer::empty_dynamic(display, 1024).unwrap(),
      proj_mat: 
      [[2.0/w as f32, 0.0,           0.0, -0.0],
       [0.0,         -2.0/h as f32,  0.0,  0.0],
       [0.0,          0.0,          -1.0,  0.0],
      [-1.0,          1.0,           0.0,  1.0]],
    }
  }

  pub fn render(&self, target: &mut glium::Frame, engine: &Engine) {
    use glium::Surface;

    let mut controller = RendererController::new(self.vbo.len());

    for e in &engine.entity_list {
      e.render(&mut controller);
    }

    if controller.data.len() == 0 { return }
    while controller.data.len() > self.vbo.len() { controller.data.pop(); }
    self.vbo.slice(0..controller.data.len()).unwrap().write(&controller.data);

    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let uniforms = uniform! {
      proj_mat: self.proj_mat,
    };

    target.draw(&self.vbo, 
                &indices, 
                &self.program, 
                &uniforms,
                &Default::default()).unwrap()
  }
}
