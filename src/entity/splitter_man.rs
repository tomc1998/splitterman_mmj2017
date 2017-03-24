use game_renderer::RendererController;
use input;
use engine::{Vec2f32, Engine};
use entity::{Entity, EHandle};

#[derive(Clone, Debug, Copy)]
pub struct SplitterMan {
  pub id: Option<EHandle>, 

  pub pos: Vec2f32,

  pub vel: Vec2f32,

  /// The splitter man's target location in world coordinates
  pub target: Option<Vec2f32>,

  /// Splitter man's 'split level'. i.e. how many clones he can split into.
  /// Lowest value = 1.
  pub size: u32,

  /// Splitter man's radius in world size
  pub rad: f32,

  /// Has the player selected this splitter man?
  pub selected: bool,

  pub speed: f32,
}

impl SplitterMan {
  pub fn new(x: f32, y: f32, size: u32) -> SplitterMan {
    SplitterMan { 
      pos: Vec2f32(x, y), 
      vel: Vec2f32(0.0, 0.0), 
      target: None, 
      size: size, 
      rad: SplitterMan::calc_size(size),
      selected: false, 
      speed: (64.0 - size as f32).sqrt() * 0.15 + 2.0 ,
      id: None,
    }
  }

  /// Processes collisions. Returns the new position of the entity, according to collisions.
  pub fn process_coll(&mut self, e: &Engine) -> Vec2f32 {
    let new_pos = self.pos + self.vel;
    let mut final_pos = new_pos;
    for e in &e.entity_list {
      let e = e.get();
      if e.get_entity_handle().unwrap().0 == self.id.unwrap().0  { 
        continue; 
      }
      let (pos, rad) = e.get_body();
      let dis = (new_pos - pos).len();
      let overlap = self.rad + rad - dis;
      if overlap > 0.0 {
        // Adjust position accordingly half way (assume other entity will do
        // the same, to result in a perfect resolution)
        // Can optimise, we've already calculated distance but we're doing it again (.nor())
        final_pos = self.pos + *(pos - new_pos).nor().scale(-overlap/2.0);
      }
    }
    return final_pos;
  }

  /// Returns a tuple. 
  /// # 1: True if this entity should be removed after the update.
  /// # 2: A list of entities to add after the update.
  /// # 3: The position this entity should be moved to after the loop.
  pub fn update(&mut self, e: &Engine) -> (bool, Option<Vec<Entity>>, Vec2f32) {
    if self.target.is_some() {
      let t = self.target.unwrap();
      let mut dir = t - self.pos;
      if dir.len2() < self.speed*self.speed {
        self.pos = self.target.unwrap();
        self.vel = Vec2f32(0.0, 0.0);
      }
      else {
        self.vel = *dir.nor().scale(self.speed);
      }
    }
    let new_pos = self.process_coll(e);

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
      if self.pos.0 - rad < sel_1.0
        && self.pos.0 + rad > sel_0.0
          && self.pos.1 - rad < sel_1.1
          && self.pos.1 + rad > sel_0.1 {
            self.selected = true;
          }
      else {
        self.selected = false;
      }
    }
    if self.selected {
      if e.input_handler.inputs.get(&input::Control::Move).unwrap().down {
        // Set target
        self.target = Some(e.g_renderer.camera.screen_to_world(e.input_handler.mouse_pos.0, e.input_handler.mouse_pos.1));
      }
      else if self.size > 1 && e.input_handler.inputs.get(&input::Control::Split).unwrap().just_down {
        let next_size = self.size / 2;
        let rad = SplitterMan::calc_size(next_size);
        let mut child_1 = SplitterMan::new(self.pos.0 - rad, self.pos.1, next_size);
        let mut child_2 = SplitterMan::new(self.pos.0 + rad, self.pos.1, next_size);
        child_1.selected = true;
        child_2.selected = true;
        return (true, Some(vec![
                           Entity::SplitterMan(child_1),
                           Entity::SplitterMan(child_2)]),
                           new_pos);
      }
    }
    return (false, None, new_pos);
  }

  /// Get entity's visual size (radius)
  #[inline(always)]
  pub fn get_size(&self) -> f32 {
    self.rad
  }

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
    cont.rect(self.pos.0 - rad, self.pos.1 - rad, rad*2.0, rad*2.0, c.0, c.1, c.2, c.3);
  }

  pub fn get_id(&self) -> Option<EHandle> { self.id }
  pub fn set_id(&mut self, new_handle: EHandle) { self.id = Some(new_handle) }
  pub fn get_body(&self) -> (Vec2f32, f32) { (self.pos, self.rad) }
  pub fn set_pos(&mut self, pos: Vec2f32) { self.pos = pos; }
}
