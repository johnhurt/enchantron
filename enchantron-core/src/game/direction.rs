use crate::model::IPoint;
use std::ops::Deref;

macro_rules! define_directions {
    ($type_name:ident { $( $dir_name:ident : ($x_coord:expr, $y_coord:expr) ),+ } ) => {
        mod hidden {
            use super::*;
            $(
                pub(super) const $dir_name: IPoint = IPoint { x: $x_coord, y: $y_coord };
            )*
        }

        pub enum $type_name {
            $(
                $dir_name
            ),*
        }

        impl Deref for $type_name {
            type Target = IPoint;

            fn deref(&self) -> &IPoint {
                match self {
                    $(
                        $dir_name => &hidden::$dir_name
                    ),*
                }
            }
        }

        impl $type_name {

            pub fn get_point(&self) -> &IPoint {
                self.deref()
            }

        }
    };
}

define_directions!(Direction {
    N: (0, -1),
    NE: (1, -1),
    E: (1, 0),
    SE: (1, 1),
    S: (0, 1),
    SW: (-1, 1),
    W: (-1, 0),
    NW: (-1, -1)
});
