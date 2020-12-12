use crate::model::Point;

#[derive(Debug, Clone, Copy, derive_new::new)]
pub struct RawTouch {
    pub touch_id: i64,
    pub point: Point,
    pub click_count: u8,
}
