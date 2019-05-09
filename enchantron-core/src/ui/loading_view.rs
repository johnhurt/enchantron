use super::ProgressBar;

pub trait LoadingView: 'static + Send + Sync + Sized {
    type P: ProgressBar;

    fn get_progress_indicator(&self) -> Self::P;

    fn transition_to_main_menu_view(&self);
}
