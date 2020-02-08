#[derive(Clone)]
pub struct CornerValues<T> {
    pub top_left: T,
    pub top_right: T,
    pub bottom_left: T,
    pub bottom_right: T,
}

impl<T> CornerValues<T> {}

impl<T> Default for CornerValues<T>
where
    T: Default,
{
    fn default() -> CornerValues<T> {
        CornerValues {
            top_left: T::default(),
            top_right: T::default(),
            bottom_left: T::default(),
            bottom_right: T::default(),
        }
    }
}
