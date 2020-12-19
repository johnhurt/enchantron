use crate::ui::{
    HasLayoutHandlers, HasMagnifyHandlers, HasMultiTouchHandlers, HasViewport,
    SpriteSource,
};

use super::BaseView;

pub trait GameView: BaseView + Sync + Send + 'static {}
