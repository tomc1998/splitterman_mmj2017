use game_renderer::RendererController;
use input;
use engine::{Vec2f32, Engine};
use entity::{Entity, EHandle, EntityBody};

pub mod state;
pub use self::state::{State, IdleState, MovingState, SplittingState, JoiningState};

#[derive(Clone, Copy, Debug)]
pub struct SplitterMan {
  pub id: Option<EHandle>, 

  pub body: EntityBody,

  pub state: State,

  /// The splitter man's target location in world coordinates
  pub target: Option<Vec2f32>,

  /// Splitter man's 'split level'. i.e. how many clones he can split into.
  /// Lowest value = 1.
  pub size: u32,

  /// Has the player selected this splitter man?
  pub selected: bool,

  pub speed: f32,
}

impl SplitterMan {
  pub fn new(x: f32, y: f32, size: u32) -> SplitterMan {
    SplitterMan { 
      body: EntityBody {
        pos: Vec2f32(x, y), 
        vel: Vec2f32(0.0, 0.0), 
        rad: SplitterMan::calc_size(size),
      },
      state: State::Idle(IdleState::new()),
      size: size, 
      target: None, 
      selected: false, 
      speed: (64.0 - size as f32).sqrt() * 0.15 + 2.0 ,
      id: None,
    }
  }

  /// Processes collisions. Returns the new position of the entity, according to collisions.
  fn process_coll(&mut self, e: &Engine) -> Vec2f32 {
    let new_pos = self.body.pos + self.body.vel;
    let mut final_pos = new_pos;
    for e in &e.entity_list {
      let e = e.get();
      if e.get_entity_handle().unwrap().0 == self.id.unwrap().0  { 
        continue; 
      }
      let body = e.get_body();
      let (pos, rad) = (body.pos, body.rad);
      let dis = (new_pos - pos).len();
      let overlap = self.body.rad + rad - dis;
      if overlap > 0.0 {
        // Adjust position accordingly half way (assume other entity will do
        // the same, to result in a perfect resolution)
        // Can optimise, we've already calculated distance but we're doing it again (.nor())
        final_pos = self.body.pos + *(pos - new_pos).nor().scale(-overlap/2.0);
      }
    }
    return final_pos;
  }

  /// Check if the box select has his this entity.
  /// # Returns
  /// True if this entity was selected
  fn check_selection_box(&mut self, e: &Engine) -> bool {
    if e.input_handler.selection.is_some() {
      // Test for collision between input rect and splitterman rect
      let rad = self.get_size();
      let mut sel = e.input_handler.selection.unwrap();
      // Make sure sel isn't malformed (sel[0], sel[1] is the top left)
      if sel[0].0 > sel[1].0 { let tmp = sel[1].0; sel[1].0 = sel[0].0; sel[0].0 = tmp }
      if sel[0].1 > sel[1].1 { let tmp = sel[1].1; sel[1].1 = sel[0].1; sel[0].1 = tmp }
      // get selection as world coords
      let sel_0 = e.g_renderer.camera.screen_to_world(sel[0].0 as i32, sel[0].1 as i32);
      let sel_1 = e.g_renderer.camera.screen_to_world(sel[1].0 as i32, sel[1].1 as i32);
      if self.body.pos.0 - rad < sel_1.0
        && self.body.pos.0 + rad > sel_0.0
          && self.body.pos.1 - rad < sel_1.1
          && self.body.pos.1 + rad > sel_0.1 {
            self.selected = true;
            return true;
          }
      else {
        self.selected = false;
      }
    }
    return false;
  }

  /// Get the velocity to move towards the target
  fn set_vel_to_target(&mut self) {
    if self.target.is_some() {
      let t = self.target.unwrap();
      let mut dir = t - self.body.pos;
      if dir.len2() < self.speed*self.speed {
        self.body.pos = self.target.unwrap();
        self.body.vel = Vec2f32(0.0, 0.0);
      }
      else {
        self.body.vel = *dir.nor().scale(self.speed);
      }
    }
  }

  /// Checks if the player has pressed the split control, and whether this
  /// entity can split any further.. If so, splits this entity. 
  /// # Returns
  /// The standard return value to notify whether this entity should be removed
  /// at the end of the update, and to add the split children to the entity
  /// list (if this entity splits).
  fn check_for_split(&mut self, e: &Engine) -> (bool, Option<Vec<Entity>>) {
    if self.size > 1 && e.input_handler.inputs.get(&input::Control::Split).unwrap().just_down {
      let next_size = self.size / 2;
      let rad = SplitterMan::calc_size(next_size);
      let mut child_1 = SplitterMan::new(self.body.pos.0 - rad, self.body.pos.1, next_size);
      let mut child_2 = SplitterMan::new(self.body.pos.0 + rad, self.body.pos.1, next_size);
      child_1.selected = true;
      child_2.selected = true;
      return (true, Some(vec![
                         Entity::SplitterMan(child_1),
                         Entity::SplitterMan(child_2)]));
    }
    return (false, None);
  }

  /// Checks for if the player has set a target dest for this entity.
  /// # Returns
  /// True if the player has moved this entity. False otherwise.
  fn check_for_move(&mut self, e: &Engine) -> bool {
    if self.selected {
      if e.input_handler.inputs.get(&input::Control::Move).unwrap().down
        && !e.input_handler.inputs.get(&input::Control::Select).unwrap().down {
          // Set target
          self.target = Some(e.g_renderer.camera.screen_to_world(e.input_handler.mouse_pos.0, e.input_handler.mouse_pos.1));
          return true;
        }
    }
    return false;
  }

  /// Process this entity's current state object. Can change state if required.
  /// Will return the data needed to return from update().
  fn process_state(&mut self, e: &Engine) -> (bool, Option<Vec<Entity>>, Vec2f32) {
    let mut state_copy = self.state;
    let (next_state, ret) = state_copy.process(self, e);
    if next_state.is_some() { self.state = next_state.unwrap(); }
    else { self.state = state_copy; }
    return ret;
  }

  /// Returns a tuple. 
  /// # 1: True if this entity should be removed after the update.
  /// # 2: A list of entities to add after the update.
  /// # 3: The position this entity should be moved to after the loop.
  pub fn update(&mut self, e: &Engine) -> (bool, Option<Vec<Entity>>, Vec2f32) {
    // Process state machine
    let (remove, ents, new_pos) = self.process_state(e);

    // Process movement, if movement was not already ordered from the state machine
    let final_pos;
    if new_pos == self.body.pos { final_pos = self.process_coll(e); }
    else { final_pos = new_pos; }

    return (remove, ents, final_pos);
  }

  /// Get entity's visual size (radius)
  #[inline(always)]
  pub fn get_size(&self) -> f32 { self.body.rad }

  /// Calculate a radius given an integer size
  #[inline(always)]
  pub fn calc_size(size: u32) -> f32 { (size as f32).sqrt() * 8.0 }

  pub fn render(&self, cont: &mut RendererController) {
    let rad = self.get_size();
    let c : (f32, f32, f32, f32);
    if self.selected {
      c = (0.0, 1.0, 1.0, 1.0);
    }
    else {
      c = (1.0, 0.0, 0.0, 1.0);
    }
    cont.rect(self.body.pos.0 - rad, self.body.pos.1 - rad, rad*2.0, rad*2.0, c.0, c.1, c.2, c.3);
  }

  pub fn get_id(&self) -> Option<EHandle> { self.id }
  pub fn set_id(&mut self, new_handle: EHandle) { self.id = Some(new_handle) }
  pub fn get_body(&self) -> EntityBody { self.body }
  pub fn set_pos(&mut self, pos: Vec2f32) { self.body.pos = pos; }
}
