use ui::ProgressBar;

pub trait LoadingView: 'static + Sized {
    type P: ProgressBar;

    fn get_progress_indicator(&self) -> Self::P;

    fn transition_to_main_menu_view(&self);
}
