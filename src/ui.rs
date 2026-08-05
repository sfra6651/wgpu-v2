use glam::Vec2;

#[derive(Debug, Copy, Clone)]
pub enum AnchorPoint {
  TopRight(Vec2),
  TopLeft(Vec2),
  BottomRight(Vec2),
  BottomLeft(Vec2),
}
