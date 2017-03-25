use entity::{Entity, SplitterMan};
use engine::{Engine, Vec2f32};

#[derive(Copy, Clone, Debug)]
pub struct IdleState;
impl IdleState {
  pub fn new() -> IdleState { IdleState }
  fn process(&mut self, 
             e: &mut SplitterMan, 
             engine: &Engine) -> (Option<State>, (bool, Option<Vec<Entity>>, Vec2f32)) {
    let (mut remove, mut ents, new_pos) = (false, None, e.body.pos);
    let mut next_state = None;

    e.check_selection_box(engine);
    if e.selected {
      let res = e.check_for_split(engine);
      remove = res.0;
      ents = res.1;
      let moved = e.check_for_move(engine);
      if moved { next_state = Some(State::Moving(MovingState::new())); }
    }
    return (next_state, (remove, ents, new_pos));
  }
}

#[derive(Copy, Clone, Debug)]
pub struct MovingState;
impl MovingState {
  pub fn new() -> MovingState { MovingState }
  fn process(&mut self, 
             e: &mut SplitterMan, 
             engine: &Engine) -> (Option<State>, (bool, Option<Vec<Entity>>, Vec2f32)) {
    let (mut remove, mut ents, new_pos) = (false, None, e.body.pos);
    let next_state = None;

    e.check_selection_box(engine);
    if e.selected {
      let res = e.check_for_split(engine);
      remove = res.0;
      ents = res.1;
      e.check_for_move(engine);
    }
    e.set_vel_to_target();

    return (next_state, (remove, ents, new_pos));
  }
}

#[derive(Copy, Clone, Debug)]
pub struct SplittingState;
impl SplittingState {
  pub fn new() -> SplittingState { SplittingState }
  fn process(&mut self, 
             _: &mut SplitterMan, 
             _: &Engine) -> (Option<State>, (bool, Option<Vec<Entity>>, Vec2f32)) {
    (None, (false, None, Vec2f32(0.0, 0.0)))
  }
}

#[derive(Copy, Clone, Debug)]
pub struct JoiningState;
impl JoiningState {
  pub fn new() -> JoiningState { JoiningState }
  fn process(&mut self, 
             e: &mut SplitterMan, 
             engine: &Engine) -> (Option<State>, (bool, Option<Vec<Entity>>, Vec2f32))  {
    (None, (false, None, Vec2f32(0.0, 0.0)))
  }
}


#[derive(Copy, Clone, Debug)]
pub enum State {
  Idle(IdleState), 
  Moving(MovingState), 
  /// Not used yet. Will be if we have animations for splitting.
  Splitting(SplittingState),
  Joining(JoiningState),
}

impl State {
  /// Returns the state this state should change to, if necessary. Also can
  /// return whether this entity needs to be destroyed, needs to spawn any more
  /// entities, or needs to be moved to a new position after the entity loop.
  pub fn process(&mut self, e: &mut SplitterMan, engine: &Engine) -> (Option<State>, (bool, Option<Vec<Entity>>, Vec2f32)) {
    match *self {
      State::Idle(ref mut s) => s.process(e, engine),
      State::Moving(ref mut s) => s.process(e, engine),
      State::Splitting(ref mut s) => s.process(e, engine),
      State::Joining(ref mut s) => s.process(e, engine),
    }
  }
}

