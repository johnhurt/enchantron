use crate::view::NativeView;

pub trait TransitionService: Send + Sync + 'static {
    type V: NativeView;

    fn transition_to(&self, view: &Self::V, drop_current: bool);

    fn transition_and_drop_current(&self, view: &Self::V) {
        self.transition_to(view, true)
    }
}
