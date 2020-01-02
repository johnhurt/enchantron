use super::BaseView;
use crate::ui::ProgressBar;

pub trait LoadingView: 'static + Send + Sync + Sized + BaseView {
    type P: ProgressBar;

    fn get_progress_indicator(&self) -> Self::P;

    fn transition_to_main_menu_view(&self);
}
