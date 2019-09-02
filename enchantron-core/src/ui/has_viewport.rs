use super::Viewport;
use crate::view_types::ViewTypes;

/// Trait for providing a viewport
pub trait HasViewport: 'static {
    type V: Viewport;

    fn get_viewport(&self) -> Self::V;
}
