use super::TerrainType;
use crate::model::IPoint;

pub trait TerrainProvider {
    fn get_for(&self, position: &IPoint) -> TerrainType;
}
