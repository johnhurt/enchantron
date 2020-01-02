use crate::util::BoxedAny;

pub trait BaseView: 'static {
    fn initialize_pre_bind(&self);

    fn initialize_post_bind(&self, presenter: BoxedAny);
}
