use super::TerrainType;
use crate::model::{IPoint, IRect};
use crate::util::ValueRect;

pub trait TerrainProvider {
    fn get_for(&self, position: &IPoint) -> TerrainType;
    fn get_for_rect(&self, rect: &IRect) -> ValueRect<TerrainType>;
}
