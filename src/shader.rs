use glium;
use glium::backend::glutin_backend::GlutinFacade;

pub const VERT_SHADER_SRC : &'static str = r#"
    #version 100

    uniform mat4 proj_mat;

    attribute vec2 pos;

    void main() {
        gl_Position = proj_mat*vec4(pos, 0.0, 1.0);
    }
"#;

pub const FRAG_SHADER_SRC : &'static str = r#"
    #version 100

    void main() {
        gl_FragColor = vec4(1.0, 0.0, 0.0, 1.0);
    }
"#;

/// Just make a program with the default VERT_SHADER_SRC and FRAG_SHADER_SRC.
/// We can abstract this later if we want different shaders.
pub fn make_program(display: &GlutinFacade) -> glium::Program {
  glium::Program::from_source(display, 
                              VERT_SHADER_SRC, 
                              FRAG_SHADER_SRC, None).unwrap()
}
