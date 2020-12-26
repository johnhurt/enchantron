use crate::ui::{
    HasLayoutHandlers, HasMagnifyHandlers, HasMultiTouchHandlers, HasViewport,
    SpriteSource,
};
use crate::util::BoxedAny;

pub trait NativeView:
    SpriteSource
    + HasLayoutHandlers
    + HasMultiTouchHandlers
    + HasViewport
    + HasMagnifyHandlers
    + Send
    + Sync
    + Sized
    + 'static
{
    fn unset_presenter(&self);

    fn set_presenter(&self, presenter: BoxedAny);
}
