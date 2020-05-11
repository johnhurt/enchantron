use crate::game::{constants, Direction};
use crate::model::IPoint;
use crate::ui::{
    HasMutableLocation, HasMutableSize, HasMutableVisibility, HasMutableZLevel,
    Sprite, SpriteGroup, SpriteSource,
};
use crate::view_types::ViewTypes;

pub trait PlayerView: 'static + Send + Sync + Unpin {
    fn start_walk(&self, direction: Direction, speed: f64);
    fn finish_walk(&self, direction: Direction, speed: f64);
}

pub struct PlayerViewImpl<T: ViewTypes> {
    bound_sprite: T::Sprite,
    player_texture: T::Texture,
}

impl<T: ViewTypes> PlayerViewImpl<T> {
    pub fn new(
        bound_sprite: T::Sprite,
        player_texture: T::Texture,
    ) -> PlayerViewImpl<T> {
        PlayerViewImpl {
            bound_sprite,
            player_texture,
        }
    }
}

impl<T: ViewTypes> PlayerView for PlayerViewImpl<T> {
    fn start_walk(&mut self, direction: Direction, speed: f64) {
        let animation_duration = self
            .bound_sprite
            .map(|sprite| sprite.set_location_animated()());
    }

    fn finish_walk(&mut self, direction: Direction, speed: f64) {}
}
