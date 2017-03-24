use glium;
use glium::backend::glutin_backend::GlutinFacade;

pub const VERT_SHADER_SRC : &'static str = r#"
  #version 130

  uniform mat4 proj_mat;

  in vec2 pos;
  in vec4 col;

  out vec4 v_col;

  void main() {
    v_col = col;
    gl_Position = proj_mat*vec4(pos, 0.0, 1.0);
  }
"#;

pub const FRAG_SHADER_SRC : &'static str = r#"
  #version 130
  precision highp float;

  in vec4 v_col;

  out vec4 color;

  void main() {
    color = v_col;
  }
"#;

/// Just make a program with the default VERT_SHADER_SRC and FRAG_SHADER_SRC.
/// We can abstract this later if we want different shaders.
pub fn make_program(display: &GlutinFacade) -> glium::Program {
  glium::Program::from_source(display, 
                              VERT_SHADER_SRC, 
                              FRAG_SHADER_SRC, None).unwrap()
}
