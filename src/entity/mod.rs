pub mod splitter_man;

use self::splitter_man::SplitterMan;
use game_renderer::RendererController;
use engine::Engine;

#[derive(Clone, Copy, Debug)]
pub struct EHandle(pub u32);

#[derive(Clone, Copy, Debug)]
pub enum Entity {
  SplitterMan(SplitterMan),
}

macro_rules! empty {
  () => {}
}

/// A macro to codegen a function dispatch.
/// # Usages
/// ## Codegen a function
/// ### Params
/// * `$ent`    - The entity to match.
/// * `$func`   - The identifier of the function to call (i.e. `update`)
/// * `$b_rule` - The borrowing rules for the match arms - i.e. ref mut or ref. 
///               Needs to be surrounded by [], so [ref mut], or [ref].
/// * `$params` - A list of optional parameters to pass to the method.
/// ### Example
/// ```
/// entity_match_and_run!(*self, update, [ref mut], engine);
/// ```
macro_rules! entity_match_and_run {
  ( $ent: expr, $func: ident, [$( $b_rule:tt )*] $(,$arg:tt)* ) => (
    match $ent {
      Entity::SplitterMan($($b_rule)* e) => e.$func($($arg)*)
    }
  );
}

impl Entity {
  /// Returns a tuple. 
  /// # 1: True if this entity should be removed after the update.
  /// # 2: A list of entities to add after the update.
  pub fn update(&mut self, engine: &Engine) -> (bool, Option<Vec<Entity>>) {
    entity_match_and_run!(*self, update, [ref mut], engine)
  }

  pub fn render(&self, cont: &mut RendererController) {
    entity_match_and_run!(*self, render, [ref], cont);
  }

  pub fn get_entity_handle(&self) -> Option<EHandle> {
    entity_match_and_run!(*self, get_id, [ref])
  }

  pub fn set_entity_handle(&mut self, h: EHandle) {
    entity_match_and_run!(*self, set_id, [ref mut], h)
  }
}

