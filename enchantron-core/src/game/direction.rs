use crate::model::IPoint;

const NORTH_POINT: IPoint = IPoint { x: 0, y: -1 };
const EAST_POINT: IPoint = IPoint { x: 1, y: 0 };
const SOUTH_POINT: IPoint = IPoint { x: 0, y: 1 };
const WEST_POINT: IPoint = IPoint { x: -1, y: 0 };

pub enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

impl Direction {
    pub fn get_point(&self) -> &'static IPoint {
        use Direction::*;

        match self {
            NORTH => &NORTH_POINT,
            EAST => &EAST_POINT,
            SOUTH => &SOUTH_POINT,
            WEST => &WEST_POINT,
        }
    }
}
