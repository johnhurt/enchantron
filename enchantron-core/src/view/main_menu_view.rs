use super::BaseView;

pub trait MainMenuView: 'static + Sized + Send + Sync + BaseView {
    fn transition_to_game_view(&self);
}
