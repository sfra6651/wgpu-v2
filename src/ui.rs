use glam::Vec2;

#[derive(Debug, Copy, Clone)]
pub enum Anchor {
  Center,
  TopLeft,
  TopRight,
  BottomLeft,
  BottomRight,
}

#[derive(Debug, Copy, Clone)]
pub struct AnchorPoint {
  pub anchor: Anchor,
  pub pos: Vec2,
}

impl AnchorPoint {
  pub fn new(anchor: Anchor, pos: Vec2) -> Self {
    Self { anchor, pos }
  }
}

#[derive(Debug, Copy, Clone)]
pub struct RenderPos(pub Vec2);

#[derive(Debug, Copy, Clone)]
pub struct UiSize(pub Vec2);
