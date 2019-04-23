
use model::{
  Card,
  Rect,
  Point
};

pub struct CardFindResponse {
  pub card: Card,
  pub play_area_ord: Option<usize>,
  pub card_rect: Rect,
  pub point_in_card: Point
}