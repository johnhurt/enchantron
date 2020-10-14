pub trait EntityPresenter {
    type View;

    fn create_view(&self) -> Self::View;
}
