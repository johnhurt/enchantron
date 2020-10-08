use crate::application_context::Ao;
use crate::game::{constants, Direction, Time};
use crate::model::{IPoint, Point};
use crate::native::RuntimeResources;
use crate::ui::{
    HasMutableLocation, HasMutableSize, HasMutableVisibility, HasMutableZLevel,
    Sprite,
};
use crate::view_types::ViewTypes;

const UNIT_TERRAIN_TILE_LENGTH_F64: f64 =
    constants::UNIT_TERRAIN_TILE_LENGTH as f64;

/// This adjusts where the player's sprite is placed relative to the sprite's
/// origin. The player's textures are all referenced from the center, but the
/// tiled terrain is referenced from the top-left corner of every tile
const PLAYER_TEXTURE_OFFSET: Point = Point {
    x: UNIT_TERRAIN_TILE_LENGTH_F64 / 2.,
    y: -UNIT_TERRAIN_TILE_LENGTH_F64 / 8.,
};

/// Get the point that's halfway from the given starting point in the given
/// direction
fn get_halfway_point_in_texture_coordinates(
    start: &IPoint,
    dir: &Direction,
) -> Point {
    (start * UNIT_TERRAIN_TILE_LENGTH_F64)
        + (dir.get_point() * (UNIT_TERRAIN_TILE_LENGTH_F64 / 2.))
}

/// Get the point adjacent to the given point in the given direction in texture
/// coordinates
fn get_final_point_in_texture_coordinates(
    start: &IPoint,
    dir: &Direction,
) -> Point {
    (start * UNIT_TERRAIN_TILE_LENGTH_F64)
        + (dir.get_point() * UNIT_TERRAIN_TILE_LENGTH_F64)
}

/// Convert the given start time and speed into a duration length
fn get_animation_duration(
    start_time: f64,
    speed_in_tiles_per_second: f64,
    time: &Time,
) -> f64 {
    0.5 / speed_in_tiles_per_second - (time.now() - start_time)
}

pub trait PlayerView: 'static + Send + Sync + Unpin {
    fn rest(&self);
    fn start_walk(
        &self,
        direction: Direction,
        start_tile: &IPoint,
        start_time: f64,
        speed: f64,
    );
    fn finish_walk(
        &self,
        direction: Direction,
        start_tile: &IPoint,
        start_time: f64,
        speed: f64,
    );
}

pub struct PlayerViewImpl<T: ViewTypes> {
    bound_sprite: T::Sprite,
    runtime_resources: Ao<RuntimeResources<T>>,
    time: Time,
}

impl<T: ViewTypes> PlayerViewImpl<T> {
    pub fn new(
        bound_sprite: T::Sprite,
        runtime_resources: Ao<RuntimeResources<T>>,
        time: Time,
    ) -> PlayerViewImpl<T> {
        bound_sprite
            .set_texture(runtime_resources.textures().gist.south_rest());
        bound_sprite.set_visible(true);
        bound_sprite.set_z_level(constants::ENTITY_Z_LEVEL);
        bound_sprite.set_size(32., 32.);
        bound_sprite.set_location_point(&PLAYER_TEXTURE_OFFSET);

        PlayerViewImpl {
            bound_sprite,
            runtime_resources,
            time,
        }
    }
}

impl<T: ViewTypes> PlayerView for PlayerViewImpl<T> {
    fn start_walk(
        &self,
        direction: Direction,
        start_tile: &IPoint,
        start_time: f64,
        speed: f64,
    ) {
        let midpoint =
            get_halfway_point_in_texture_coordinates(start_tile, &direction);

        let duration = get_animation_duration(start_time, speed, &self.time);

        self.bound_sprite.animate(
            &self.runtime_resources.animations().player_walk_south,
            1. / 6.,
        );

        self.bound_sprite.set_location_point_animated(
            &(midpoint + &PLAYER_TEXTURE_OFFSET),
            duration,
        );
    }

    fn finish_walk(
        &self,
        direction: Direction,
        start_tile: &IPoint,
        start_time: f64,
        speed: f64,
    ) {
        let destination =
            get_final_point_in_texture_coordinates(start_tile, &direction);

        let duration = get_animation_duration(start_time, speed, &self.time);

        self.bound_sprite.set_location_point_animated(
            &(destination + &PLAYER_TEXTURE_OFFSET),
            duration,
        );
    }

    fn rest(&self) {
        self.bound_sprite
            .set_texture(self.runtime_resources.textures().gist.south_rest());
    }
}
