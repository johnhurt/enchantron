use super::BaseView;

pub trait LoadingView: 'static + Send + Sync + Sized + BaseView {
    fn transition_to_main_menu_view(&self);
}
