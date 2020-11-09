use super::RawTouchPoint;

#[derive(Debug, Clone, Copy, derive_new::new)]
pub struct RawTouch {
    pub touch_id: i64,
    pub point: RawTouchPoint,
    pub click_count: u8,
}
