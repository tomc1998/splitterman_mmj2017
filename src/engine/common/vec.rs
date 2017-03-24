use std::ops::*;

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct Vec2f32 (pub f32, pub f32);

impl Vec2f32 {
  pub fn new() -> Vec2f32 { Vec2f32(0.0, 0.0) }
  pub fn new_from_copy(copy: &Vec2f32) -> Vec2f32 { Vec2f32(copy.0, copy.1) }

  /// Set this vector to another one.
  pub fn set(&mut self, target: &Vec2f32) {
    self.0 = target.0;
    self.1 = target.1;
  }
}

impl AddAssign for Vec2f32 {
  fn add_assign(&mut self, other: Vec2f32) {
    self.0 += other.0;
    self.1 += other.1;
  }
}

impl Add for Vec2f32 {
  type Output = Vec2f32;
  fn add(self, other: Vec2f32) -> Self::Output {
    Vec2f32(self.0 + other.0, self.1 + other.1)
  }
}

impl SubAssign for Vec2f32 {
  fn sub_assign(&mut self, other: Vec2f32) {
    self.0 -= other.0;
    self.1 -= other.1;
  }
}

impl Sub for Vec2f32 {
  type Output = Vec2f32;
  fn sub(self, other: Vec2f32) -> Self::Output {
    Vec2f32(self.0 - other.0, self.1 - other.1)
  }
}

impl Vec2f32 {
  pub fn len(&self) -> f32 {
    (self.0.powi(2) + self.1.powi(2)).sqrt()
  }
  pub fn len2(&self) -> f32 {
    self.0.powi(2) + self.1.powi(2)
  }

  /// Normalise the vector. Return a mutable reference to self, so you can
  /// chain functions.
  pub fn nor(&mut self) -> &mut Vec2f32 {
    let len = self.len();
    self.0 /= len;
    self.1 /= len;
    return self;
  }

  /// Scale the vector by a given amount. Return a mutable reference to self,
  /// so you can chain functioins.
  pub fn scale(&mut self, amount: f32) -> &mut Vec2f32 {
    self.0 *= amount;
    self.1 *= amount;
    return self;
  }
}
