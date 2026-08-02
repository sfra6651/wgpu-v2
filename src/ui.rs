use glam::Vec2;

pub enum AnchorPoint {
  TopRight(Vec2),
  TopLeft(Vec2),
  BottomRight(Vec2),
  BottomLeft(Vec2),
}

pub struct UiElement {
  anchor: AnchorPoint,
}
