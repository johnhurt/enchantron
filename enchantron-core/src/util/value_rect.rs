use crate::model::IRect;

/// Representation of a set of values over rectangular region
pub struct ValueRect<T> {
    pub rect: IRect,
    values: Vec<Vec<T>>,
    pub x_stride: usize,
    pub y_stride: usize,
}

impl<T> ValueRect<T> {
    pub fn new(rect: &IRect) -> ValueRect<T> {
        let values: Vec<Vec<T>> = (0..rect.size.width)
            .map(|_| Vec::with_capacity(rect.size.height))
            .collect();

        ValueRect {
            rect: rect.clone(),
            values,
        }
    }

    pub fn get_column(&self, col: usize) -> Option<&Vec<T>> {
        self.values.get(col)
    }

    pub fn get_column_mut(&mut self, col: usize) -> Option<&mut Vec<T>> {
        self.values.get_mut(col)
    }
}
