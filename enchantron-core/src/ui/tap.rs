use super::DragPoint;

#[derive(Debug, Clone, Copy, derive_new::new)]
pub struct Tap {
    pub tap_count: u8,
    pub point: DragPoint,
}
