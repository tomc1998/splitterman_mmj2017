use glium;
use glium::backend::glutin_backend::GlutinFacade;
use engine::Vec2f32;
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

pub struct Camera {
  /// Position (center) of camera viewport in world coords
  pos: [f32; 2],
  /// Size of camera viewport in world coords
  size: [f32; 2],
  /// Size of the inner screen in pixels
  screen_size: [i32; 2],
}
impl Camera {
  pub fn new(w: f32, h: f32, screen_w: i32, screen_h: i32) -> Camera {
    Camera { pos: [0.0, 0.0], size: [w, h], screen_size: [screen_w, screen_h] }
  }

  pub fn gen_proj_mat(&self) -> [[f32; 4]; 4] {
    let (l, r, t, b) = (self.pos[0] - self.size[0]/2.0, self.pos[0] + self.size[0]/2.0, 
                        self.pos[1] - self.size[1]/2.0, self.pos[1] + self.size[1]/2.0);
    let tx = -(r+l)/(r-l);
    let ty = -(t+b)/(t-b);
    return [[2.0/(r-l) as f32, 0.0,           0.0, -0.0],
    [0.0,         2.0/(t-b) as f32,  0.0,  0.0],
    [0.0,          0.0,          -1.0,  0.0],
    [tx,          ty,           0.0,  1.0]];
  }

  /// Convert screen coords to world coords.
  pub fn screen_to_world(&self, x: i32, y: i32) -> Vec2f32 {
    // Get distance from the centre
    let x_dis = x - self.screen_size[0]/2;
    let y_dis = y - self.screen_size[1]/2;
    // Convert distnace to world distance
    let x_dis_w = x_dis as f32 * (self.size[0]/self.screen_size[0] as f32);
    let y_dis_w = y_dis as f32 * (self.size[1]/self.screen_size[1] as f32);
    // Convert distance to pos
    return Vec2f32(self.pos[0] + x_dis_w, self.pos[1] + y_dis_w);
  }
}

pub struct Renderer {
  program: glium::Program,
  pub camera: Camera,
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
      camera: Camera::new(w as f32, h as f32, w as i32, h as i32),
      vbo: glium::VertexBuffer::empty_dynamic(display, 65536).unwrap(),
      hud_vbo: glium::VertexBuffer::empty_dynamic(display, 1024).unwrap(),
      proj_mat: [[2.0/w as f32, 0.0,           0.0, -0.0],
      [0.0,         -2.0/h as f32,  0.0,  0.0],
      [0.0,          0.0,          -1.0,  0.0],
      [-1.0,          1.0,           0.0,  1.0]],
      hud_proj_mat: [[2.0/w as f32, 0.0,           0.0, -0.0],
      [0.0,         -2.0/h as f32,  0.0,  0.0],
      [0.0,          0.0,          -1.0,  0.0],
      [-1.0,          1.0,           0.0,  1.0]],
    }
  }

  /// Update projection mat to reflect camera
  pub fn update_proj_mat(&mut self) {
    self.proj_mat = self.camera.gen_proj_mat();
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
      e.get().render(&mut controller);
    }

    if controller.data.len() == 0 { return }
    controller.data.resize(self.vbo.len(), Vertex::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0));
    self.vbo.write(&controller.data);

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
      controller.rect(b[0].0, b[0].1, b[1].0 - b[0].0, b[1].1 - b[0].1, 0.0, 1.0, 1.0, 0.4);
    }

    // Mouse
    let m = engine.input_handler.mouse_pos;
    controller.rect(m.0 as f32, m.1 as f32, 4.0, 4.0, 1.0, 1.0, 1.0, 1.0);

    if controller.data.len() == 0 { return }
    controller.data.resize(self.hud_vbo.len(), Vertex::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0));
    self.hud_vbo.write(&controller.data);


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
