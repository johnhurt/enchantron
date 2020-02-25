use crate::model::{IPoint, IRect, ISize, UPoint};
use std::cmp::{Eq, PartialEq};
use std::ops::AddAssign;

/// Representation of a set of values over rectangular region
#[derive(Getters, Debug)]
pub struct ValueRect<T> {
    #[get = "pub"]
    rect: IRect,

    values: Vec<T>,

    #[get = "pub"]
    x_stride: usize,

    #[get = "pub"]
    y_stride: usize,

    #[get = "pub"]
    values_width: usize,

    #[get = "pub"]
    values_height: usize,
}

impl<T> ValueRect<T> {
    pub fn new_from_rect(
        rect: IRect,
        x_stride: usize,
        y_stride: usize,
        mut filler: impl FnMut(&IPoint) -> T,
    ) -> ValueRect<T> {
        debug_assert!(rect.size.width % x_stride == 0);
        debug_assert!(rect.size.height % y_stride == 0);

        let values_width = rect.size.width / x_stride;
        let values_height = rect.size.height / y_stride;
        let total_values = values_height * values_width;

        let mut coord = IPoint::default();

        let mut values = Vec::<T>::with_capacity(total_values);

        let filler_ref = &mut filler;

        (0..values_height).for_each(|v_y| {
            coord.x = &rect.top_left.y + (v_y * y_stride) as i64;
            (0..values_width).for_each(|v_x| {
                coord.y = &rect.top_left.x + (v_x * x_stride) as i64;
                values.push(filler_ref(&coord))
            });
        });

        ValueRect {
            rect,
            values,
            x_stride,
            y_stride,
            values_width,
            values_height,
        }
    }

    pub fn new_from_point_and_strides(
        top_left: IPoint,
        x_stride: usize,
        y_stride: usize,
        x_stride_count: usize,
        y_stride_count: usize,
        filler: impl FnMut(&IPoint) -> T,
    ) -> ValueRect<T> {
        let rect = IRect {
            top_left,
            size: ISize::new(
                x_stride * x_stride_count,
                y_stride * y_stride_count,
            ),
        };
        ValueRect::new_from_rect(rect, x_stride, y_stride, filler)
    }

    pub fn get_by_point<'a>(
        &'a self,
        value_coordinate: &UPoint,
    ) -> Option<&'a T> {
        self.get(value_coordinate.x, value_coordinate.y)
    }

    pub fn get<'a>(&'a self, value_x: usize, value_y: usize) -> Option<&'a T> {
        self.values.get(value_y * self.values_width + value_x)
    }

    pub fn map<V>(&self, mapper: impl Fn(&T) -> V) -> ValueRect<V> {
        let len = self.values_width * self.values_height;
        let mut values: Vec<V> = Vec::with_capacity(len);

        // Look, Mom. I used unsafe for the first time
        unsafe {
            let mut result_vec_prt = values.as_mut_ptr();
            values.set_len(len);

            for source_val in &self.values {
                result_vec_prt.write(mapper(&source_val));
                result_vec_prt = result_vec_prt.add(1);
            }
        }

        ValueRect {
            rect: self.rect.clone(),
            values,
            x_stride: self.x_stride,
            y_stride: self.y_stride,
            values_width: self.values_width,
            values_height: self.values_height,
        }
    }

    /// Get the raw value vector. These values are stored in a linear array
    /// by columns then rows (opposite to how image data is laid out)
    pub fn get_raw_values_mut(&mut self) -> &mut Vec<T> {
        &mut self.values
    }

    pub fn for_each_value_coord(&self, mut actor: impl FnMut(&UPoint, &T)) {
        let mut point = UPoint::default();
        let actor_ref = &mut actor;
        let values_width = self.values_width;

        self.values.iter().enumerate().for_each(|(i, v)| {
            point.x = i % values_width;
            point.y = i / values_width;
            actor_ref(&point, v)
        })
    }

    pub fn for_each_mut(&mut self, mut actor: impl FnMut(&IPoint, &mut T)) {
        let mut point = IPoint::default();
        let actor_ref = &mut actor;
        let values_width = self.values_width;
        let x_stride = self.x_stride;
        let y_stride = self.y_stride;
        let top_left_x = self.rect.top_left.x;
        let top_left_y = self.rect.top_left.y;

        self.values.iter_mut().enumerate().for_each(|(i, v)| {
            point.x = ((i % values_width) * x_stride) as i64 + top_left_x;
            point.y = ((i / values_width) * y_stride) as i64 + top_left_y;
            actor_ref(&point, v)
        })
    }
}

impl<T> ValueRect<T>
where
    T: Default + Clone,
{
    pub fn new_from_rect_with_defaults(
        rect: IRect,
        x_stride: usize,
        y_stride: usize,
    ) -> ValueRect<T> {
        debug_assert!(rect.size.width % x_stride == 0);
        debug_assert!(rect.size.height % y_stride == 0);

        let values_width = rect.size.width / x_stride;
        let values_height = rect.size.height / y_stride;
        let total_values = values_height * values_width;

        let values = vec![T::default(); total_values];

        ValueRect {
            rect,
            values,
            x_stride,
            y_stride,
            values_width,
            values_height,
        }
    }

    pub fn new_from_point_and_strides_with_defaults(
        top_left: IPoint,
        x_stride: usize,
        y_stride: usize,
        x_stride_count: usize,
        y_stride_count: usize,
    ) -> ValueRect<T> {
        let rect = IRect {
            top_left,
            size: ISize::new(
                x_stride * x_stride_count,
                y_stride * y_stride_count,
            ),
        };

        ValueRect::<T>::new_from_rect_with_defaults(rect, x_stride, y_stride)
    }
}

impl<T> PartialEq for ValueRect<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &ValueRect<T>) -> bool {
        self.x_stride == other.x_stride
            && self.y_stride == other.y_stride
            && self.rect == other.rect
            && self.values == other.values
    }
}

impl<'a, T> AddAssign<&'a ValueRect<T>> for ValueRect<T>
where
    T: AddAssign<&'a T> + 'static,
{
    fn add_assign(&mut self, rhs: &'a ValueRect<T>) {
        debug_assert_eq!(rhs.rect, self.rect);
        debug_assert_eq!(rhs.x_stride, self.x_stride);
        debug_assert_eq!(rhs.y_stride, self.y_stride);

        self.values.iter_mut().zip(rhs.values.iter()).for_each(
            |(dest_value, src_value)| {
                *dest_value += src_value;
            },
        );
    }
}
