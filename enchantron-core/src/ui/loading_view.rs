
use ui::{ProgressBar};

pub trait LoadingView {
  type P : ProgressBar;

  fn get_progress_indicator(&self) -> Self::P;
}