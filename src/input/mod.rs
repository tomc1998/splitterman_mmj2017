use std::collections::BTreeMap;
use glium::glutin::{Event, ElementState, VirtualKeyCode, MouseButton};
use glium::backend::glutin_backend::GlutinFacade;
use engine::Vec2f32;

#[derive(Clone, PartialEq, Hash)]
pub enum InputType {
  Key(VirtualKeyCode),
  Mouse(MouseButton),
}

#[derive(Clone, Hash)]
pub struct Input {
  pub input: InputType,
  pub down: bool,
  pub just_down: bool,
}

impl Input {
  pub fn new_key_input(k: VirtualKeyCode) -> Input {
    Input { input: InputType::Key(k), down: false, just_down: false }
  }
  pub fn new_mouse_input(m: MouseButton) -> Input {
    Input { input: InputType::Mouse(m), down: false, just_down: false }
  }

  pub fn pressed(&mut self) { self.down = true; self.just_down = true; }
  pub fn released(&mut self) { self.down = false; self.just_down = false; }
}

#[derive(Ord, Eq, PartialOrd, PartialEq, Hash)]
pub enum Control {
  Split, Select, Move
}

pub struct InputHandler {
  /// Box currently being dragged
  pub curr_box: Option<[Vec2f32; 2]>,
  /// Will be set for 1 frame after curr_box is stopped dragging.
  pub selection: Option<[Vec2f32; 2]>,
  pub mouse_pos: (i32, i32),
  pub inputs: BTreeMap<Control, Input>,
}

impl InputHandler {
  pub fn new() -> InputHandler {
    let mut i = InputHandler { 
      curr_box: None,
      selection: None,
      mouse_pos: (0, 0),
      inputs: BTreeMap::new(),
    };

    i.inputs.insert(Control::Select, Input::new_mouse_input(MouseButton::Left));
    i.inputs.insert(Control::Move, Input::new_mouse_input(MouseButton::Right));
    i.inputs.insert(Control::Split, Input::new_key_input(VirtualKeyCode::Space));

    return i;
  }

  /// Returns true if user requests close
  fn record_key_input(&mut self, state: ElementState, 
                      keycode: VirtualKeyCode) -> bool {
    for (_, mut input) in &mut self.inputs.iter_mut() {
      if input.input != InputType::Key(keycode) { continue; }
      match state {
        ElementState::Pressed => input.pressed(),
        ElementState::Released => input.released(),
      }
    }
    if keycode == VirtualKeyCode::Escape { true } else { false }
  }

  fn record_mouse_input(&mut self, state: ElementState, 
                      button: MouseButton) {
    for (_, mut input) in &mut self.inputs.iter_mut() {
      if input.input != InputType::Mouse(button) { continue; }
      match state {
        ElementState::Pressed => input.pressed(),
        ElementState::Released => input.released(),
      }
    }
  }

  fn reset_just_pressed(&mut self) {
    for (_, mut input) in self.inputs.iter_mut() {
      input.just_down = false;
    }
  }

  // Check input. Return true if used requested to quit. 
  pub fn check_input(&mut self, display: &GlutinFacade) -> bool {
    self.reset_just_pressed();
    self.selection = None;
    for e in display.poll_events() {
      match e {
        Event::Closed => return true,
        Event::KeyboardInput(state, _, keycode) => {
          if keycode.is_none() { return false; }
          let keycode = keycode.unwrap();
          return self.record_key_input(state, keycode);
        },
        Event::MouseInput(state, button) => self.record_mouse_input(state, button),
        Event::MouseMoved(x, y) => self.mouse_pos = (x, y),
        _ => (),
      }
    }

    self.process_input();
    return false;
  }

  /// Do actual processing of controls data, rather than just recording values.
  fn process_input(&mut self) {
    let c_select = self.inputs.get(&Control::Select).unwrap();
    if c_select.just_down {
      self.curr_box = Some([Vec2f32(self.mouse_pos.0 as f32, self.mouse_pos.1 as f32),
                           Vec2f32(self.mouse_pos.0 as f32, self.mouse_pos.1 as f32)]);
    }
    else if self.curr_box.is_some() {
      if !c_select.down { 
        self.selection = self.curr_box;
        self.curr_box = None; 
      }
      else {
        let mut b = self.curr_box.as_mut().unwrap();
        b[1].0 = self.mouse_pos.0 as f32;
        b[1].1 = self.mouse_pos.1 as f32;
      }
    }
  }
}

