use crate::ui::{
    HasLayoutHandlers, HasMagnifyHandlers, HasMultiTouchHandlers, HasViewport,
    SpriteSource,
};
use crate::util::BoxedAny;

pub trait BaseView:
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
    fn initialize_pre_bind(&self);

    fn initialize_post_bind(&self, presenter: BoxedAny);
}
