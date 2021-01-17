use crate::application_context::Ao;
use crate::event::*;
use crate::game::constants;
use crate::model::{IPoint, IRect, ISize, Rect, UPoint, URect};
use crate::native::{RuntimeResources, SystemInterop};
use crate::ui::{
    HasMutableLocation, HasMutableSize, HasMutableVisibility, Sprite,
    SpriteSource, TerrainTextureProvider, TerrainUpdateInfo, ViewportInfo,
};
use crate::view_types::ViewTypes;
use futures::pin_mut;
use std::iter;
use tokio::select;
use tokio::stream::StreamExt;

const TERRAIN_TEXTURE_SIDE_LENGTH: usize = 256;
const TERRAIN_TEXTURE_SIDE_LENGTH_F64: f64 = TERRAIN_TEXTURE_SIDE_LENGTH as f64;
const TERRAIN_TEXTURE_SIZE: ISize = ISize {
    width: TERRAIN_TEXTURE_SIDE_LENGTH,
    height: TERRAIN_TEXTURE_SIDE_LENGTH,
};
const UNIT_ZOOM_LEVEL_TILE_LENGTH: usize = constants::UNIT_TERRAIN_TILE_LENGTH;
const UNIT_ZOOM_LEVEL_TILE_LENGTH_F64: f64 = UNIT_ZOOM_LEVEL_TILE_LENGTH as f64;

/// Get the minimum terrain sprite rect needed to cover the given terrain rect
/// at the given zoom level as well as the size of the 2-D array of sprites
/// needed to cover it.
fn get_min_sprite_covering(terrain_rect: &IRect) -> (IRect, ISize) {
    let sprite_width_in_tiles = TERRAIN_TEXTURE_SIDE_LENGTH as i64;

    let mut top_left = IPoint::new(
        terrain_rect.top_left.x.div_euclid(sprite_width_in_tiles),
        terrain_rect.top_left.y.div_euclid(sprite_width_in_tiles),
    );

    // shift the top left up and left by one to include an extra area of
    // terrain
    top_left.x -= 1;
    top_left.y -= 1;

    top_left *= sprite_width_in_tiles;

    let partial_terrain_size =
        terrain_rect.bottom_right_inclusive() - &top_left;

    // increase the array size by 2 in each dimension to account for inclusivity
    // and adding an extra boundary of terrain tiles to the top and bottom

    let mut sprite_array_size = partial_terrain_size / sprite_width_in_tiles;

    sprite_array_size.x += 2;
    sprite_array_size.y += 2;

    let terrain_size = sprite_array_size * sprite_width_in_tiles;

    (
        IRect {
            top_left,
            size: terrain_size.to_size().unwrap(),
        },
        sprite_array_size.to_size().unwrap(),
    )
}

pub struct TerrainPresenter<T: ViewTypes> {
    event_bus: EventBus,
    terrain_texture_provider: TerrainTextureProvider<T>,
    listener_registrations: Vec<ListenerRegistration>,
    sprite_group: T::SpriteGroup,
    terrain_sprites: Vec<Vec<T::Sprite>>,
    terrain_sprites_size: ISize,
    sprite_terrain_coverage: IRect,
    top_left_sprite: UPoint,
}

impl<T> TerrainPresenter<T>
where
    T: ViewTypes,
{
    pub fn new<S>(
        event_bus: EventBus,
        sprite_source: &S,
        runtime_resources: Ao<RuntimeResources<T>>,
        system_interop: Ao<T::SystemInterop>,
    ) -> TerrainPresenter<T>
    where
        S: SpriteSource<T = T::Texture, S = T::Sprite, G = T::SpriteGroup>,
    {
        let terrain_texture_provider = TerrainTextureProvider::new(
            runtime_resources,
            system_interop.get_resource_loader(),
        );

        TerrainPresenter {
            event_bus,
            terrain_texture_provider,
            listener_registrations: Vec::new(),
            sprite_group: sprite_source.create_group(),
            terrain_sprites: Default::default(),
            terrain_sprites_size: Default::default(),
            sprite_terrain_coverage: Default::default(),
            top_left_sprite: Default::default(),
        }
    }

    pub async fn run(mut self) {
        let end_event = self.event_bus.register_for_one::<StopGameRequested>();
        let (listener_registration, event_stream) =
            self.event_bus.register_to_watch::<ViewportChange>();

        self.listener_registrations.push(listener_registration);

        pin_mut!(event_stream);
        pin_mut!(end_event);

        info!("Terrain presenter started");
        self.event_bus.post(TerrainPresenterStarted::new());

        while let Some(ViewportChange { new_viewport }) = select! {
            viewport_info_opt = event_stream.next() => viewport_info_opt,
            _ = &mut end_event => None
        } {
            self.on_viewport_change(&new_viewport).await;
        }

        info!("Terrain Presenter Stopped");
    }

    async fn on_viewport_change(&mut self, viewport_info: &ViewportInfo) {
        let terrain_update_info_opt =
            self.terrain_updates_required(viewport_info);

        if terrain_update_info_opt.is_none() {
            return;
        }

        let terrain_update_info = terrain_update_info_opt.unwrap();

        let valid_sprite_rect = {
            let size_increased = self.check_sprite_array_size_increased(
                &terrain_update_info.sprite_array_size,
            );

            if size_increased {
                debug!("Size increased");
                self.increase_size_for(terrain_update_info)
            } else {
                debug!("Size not increased");
                let (top_left, new_valid_rect) = self
                    .calculate_new_valid_sprites(&terrain_update_info)
                    .unwrap_or_default();

                self.update_terrain_sprite_info(terrain_update_info, top_left);

                new_valid_rect
            }
        };

        let sprite_width =
            TERRAIN_TEXTURE_SIDE_LENGTH_F64 * UNIT_ZOOM_LEVEL_TILE_LENGTH_F64;

        self.update_terrain_sprites(valid_sprite_rect, |sprite, point| {
            sprite.set_visible(false);

            let texture_terrain_rect = IRect {
                top_left: *point,
                size: TERRAIN_TEXTURE_SIZE,
            };

            sprite.set_texture(
                &self.terrain_texture_provider.get_texture_for_rect(
                    &texture_terrain_rect,
                    &TERRAIN_TEXTURE_SIZE,
                ),
            );

            sprite.set_size(sprite_width, sprite_width);
            sprite.set_visible(true);
        });
    }

    /// Update the invalid terrain sprites
    fn update_terrain_sprites(
        &self,
        new_valid_rect: URect,
        sprite_updater: impl Fn(&T::Sprite, &IPoint),
    ) {
        trace!("Updating terrain sprites");

        // hit all the partial rows to the right of the valid region

        let new_valid_size = &new_valid_rect.size;
        let valid_top_left = &new_valid_rect.top_left;

        let action = |x: usize, y: usize| {
            let (sprite, terrain_point) =
                self.get_sprite_at(valid_top_left, &x, &y);
            sprite.set_location_point(
                &(&terrain_point * UNIT_ZOOM_LEVEL_TILE_LENGTH_F64),
            );
            sprite_updater(sprite, &terrain_point);
        };

        for y in 0..new_valid_size.height {
            for x in new_valid_size.width..self.terrain_sprites_size.width {
                action(x, y);
            }
        }

        // hit all the complete rows below the valid region

        for y in new_valid_size.height..self.terrain_sprites_size.height {
            for x in 0..self.terrain_sprites_size.width {
                action(x, y);
            }
        }
    }

    /// Get the terrain rect required to cover the given viewport rect based on
    /// the current size of the terrain sprites array.
    fn viewport_rect_to_terrain_rect(&self, viewport_rect: &Rect) -> IRect {
        let tile_size_f64 = UNIT_ZOOM_LEVEL_TILE_LENGTH_F64;

        let viewport_top_left = &viewport_rect.top_left;
        let viewport_bottom_right = viewport_top_left + &viewport_rect.size;

        let top_left = IPoint {
            x: (viewport_top_left.x / tile_size_f64).floor() as i64,
            y: (viewport_top_left.y / tile_size_f64).floor() as i64,
        };

        let bottom_right = IPoint {
            x: (viewport_bottom_right.x / tile_size_f64).ceil() as i64,
            y: (viewport_bottom_right.y / tile_size_f64).ceil() as i64,
        };

        let size = (bottom_right - &top_left).to_size().expect("bad size");

        IRect { top_left, size }
    }

    /// return true if the current size of the 2d vector array is bigger than
    /// or equal to the size given in both height and width
    fn check_sprite_array_size_increased(
        &self,
        min_sprite_array_size: &ISize,
    ) -> bool {
        min_sprite_array_size.width > self.terrain_sprites_size.width
            || min_sprite_array_size.height > self.terrain_sprites_size.height
    }

    /// Determine if the terrain sprites need updating, and return the new terrain
    /// rect, if an update is needed
    fn terrain_updates_required(
        &self,
        viewport_info: &ViewportInfo,
    ) -> Option<TerrainUpdateInfo> {

        if (viewport_info.viewport_scale >= 6.) {
            return None;
        }

        let viewport_rect = &viewport_info.viewport_rect;

        let mut terrain_rect =
            self.viewport_rect_to_terrain_rect(viewport_rect);

        let (min_covered_terrain, mut min_sprite_array_size) =
            get_min_sprite_covering(&terrain_rect);

        let tiles_per_sprite = TERRAIN_TEXTURE_SIDE_LENGTH;
        let tiles_per_sprite_i64 = tiles_per_sprite as i64;

        // If the minimum size of the sprite array is less than the current
        // size of the sprite array in either dimension, center the minimum
        // terrain rect within a larger terrain rect that has the same size as
        // what can be covered by the current sprite array
        if min_sprite_array_size.width <= self.terrain_sprites_size.width {
            let dw =
                self.terrain_sprites_size.width - min_sprite_array_size.width;
            terrain_rect.top_left.x -= (dw as i64 / 2) * tiles_per_sprite_i64;
            terrain_rect.size.width =
                self.terrain_sprites_size.width * tiles_per_sprite;
            min_sprite_array_size.width = self.terrain_sprites_size.width;
        }

        if min_sprite_array_size.height <= self.terrain_sprites_size.height {
            let dh =
                self.terrain_sprites_size.height - min_sprite_array_size.height;
            terrain_rect.top_left.y -= (dh as i64 / 2) * tiles_per_sprite_i64;
            terrain_rect.size.height =
                self.terrain_sprites_size.height * tiles_per_sprite;
            min_sprite_array_size.height = self.terrain_sprites_size.height;
        }

        Some(TerrainUpdateInfo {
            zoom_level: 0,
            terrain_rect: min_covered_terrain,
            sprite_length_in_tiles: tiles_per_sprite,
            sprite_array_size: min_sprite_array_size,
            zoom_level_changed: false,
        })
    }

    /// Increase the size of the 2d array of terrain sprites to accommodate the
    /// given size
    fn increase_size_for(
        &mut self,
        terrain_update_info: TerrainUpdateInfo,
    ) -> URect {
        let min_sprite_array_size = &terrain_update_info.sprite_array_size;

        debug!(
            "Increasing terrain sprites cache to {:?}",
            min_sprite_array_size
        );

        let width_inc = min_sprite_array_size
            .width
            .checked_sub(self.terrain_sprites_size.width)
            .unwrap_or_default();
        let height_inc = min_sprite_array_size
            .height
            .checked_sub(self.terrain_sprites_size.height)
            .unwrap_or_default();

        if width_inc > 0 {
            self.increase_row_width_by(width_inc);
        }

        if height_inc > 0 {
            self.increase_row_count_by(height_inc);
        }

        let (new_top_left, valid_rect) = self
            .calculate_new_valid_sprites(&terrain_update_info)
            .unwrap_or_default();

        self.update_terrain_sprite_info(terrain_update_info, new_top_left);

        valid_rect
    }

    /// Add the two points together within the bounds of the sprite grid system
    fn sprite_grid_add(&self, lhs: &UPoint, rhs: &IPoint) -> UPoint {
        let sprite_grid_width = self.terrain_sprites_size.width as i64;
        let sprite_grid_height = self.terrain_sprites_size.height as i64;

        UPoint::new(
            (lhs.x as i64 + rhs.x).rem_euclid(sprite_grid_width) as usize,
            (lhs.y as i64 + rhs.y).rem_euclid(sprite_grid_height) as usize,
        )
    }

    /// Determine which portion if any of the existing valid terrain sprites will
    /// remain valid when the viewport shifts to the given terrain rect, and
    /// return the new top-left point within the sprite array, and the region of
    /// the sprite array that's still valid
    fn calculate_new_valid_sprites(
        &self,
        terrain_update_info: &TerrainUpdateInfo,
    ) -> Option<(UPoint, URect)> {
        let new_terrain_rect = &terrain_update_info.terrain_rect;
        let tiles_per_sprite = terrain_update_info.sprite_length_in_tiles;

        self.sprite_terrain_coverage
            .intersection(new_terrain_rect)
            .map(|itx_rect| {
                // Determine the top left and size of the valid region
                let mut valid_top_left_shift =
                    &itx_rect.top_left - &self.sprite_terrain_coverage.top_left;

                valid_top_left_shift /= tiles_per_sprite as i64;

                let valid_sprites_top_left = self.sprite_grid_add(
                    &self.top_left_sprite,
                    &valid_top_left_shift,
                );

                let new_valid_rect = URect {
                    top_left: valid_sprites_top_left,
                    size: itx_rect.size / tiles_per_sprite,
                };

                // determine the new top left of the viewport rect

                let mut new_top_left_shift = &new_terrain_rect.top_left
                    - &self.sprite_terrain_coverage.top_left;

                new_top_left_shift /= tiles_per_sprite as i64;

                let new_top_left = self.sprite_grid_add(
                    &self.top_left_sprite,
                    &new_top_left_shift,
                );

                trace!("valid sprite area = {}", new_valid_rect.area());

                (new_top_left, new_valid_rect)
            })
    }

    /// Update the information about the terrain sprites that describes which part
    /// of the terrain is being shown and where in the sprite array the top-left
    /// corner is
    fn update_terrain_sprite_info(
        &mut self,
        update_info: TerrainUpdateInfo,
        top_left_sprite: UPoint,
    ) {
        self.top_left_sprite = top_left_sprite;
        self.sprite_terrain_coverage = update_info.terrain_rect;
    }

    /// Increase the size of all the existing rows in the terrain to the given
    /// width
    fn increase_row_width_by(&mut self, cols_to_add: usize) {
        let insert_point = self.top_left_sprite.x;
        let sprite_group = &self.sprite_group;

        let sprite_source = || {
            let result = sprite_group.create_sprite();
            result.set_visible(true);
            result
        };

        self.terrain_sprites.iter_mut().for_each(|row| {
            let to_insert = iter::repeat_with(sprite_source).take(cols_to_add);

            row.splice(insert_point..insert_point, to_insert)
                .for_each(|_| {});
        });

        self.top_left_sprite.x += cols_to_add;
        self.terrain_sprites_size.width += cols_to_add;
    }

    /// Increase the number of rows in the sprite array by the given number
    fn increase_row_count_by(&mut self, rows_to_add: usize) {
        let total_columns = self.terrain_sprites_size.width;
        let sprite_group = &self.sprite_group;

        let sprite_source = || {
            let result = sprite_group.create_sprite();
            result.set_visible(true);
            result
        };

        let new_rows = iter::repeat_with(|| {
            iter::repeat_with(sprite_source)
                .take(total_columns)
                .collect()
        })
        .take(rows_to_add);

        let insert_point = &self.top_left_sprite.y;

        self.terrain_sprites
            .splice(insert_point..insert_point, new_rows);

        self.top_left_sprite.y += rows_to_add;
        self.terrain_sprites_size.height += rows_to_add;
    }

    fn get_sprite<'a>(&'a self, point: &UPoint) -> Option<&'a T::Sprite> {
        self.terrain_sprites
            .get(point.y)
            .and_then(|row| row.get(point.x))
    }

    /// Assume that the given point p is a point within the bounds of the
    /// sprite grid, get the offset from origin of the sprite grid (the top left)
    fn sprite_grid_offset_from_origin(&self, p: &UPoint) -> IPoint {
        let grid_width = self.terrain_sprites_size.width as i64;
        let grid_height = self.terrain_sprites_size.height as i64;

        IPoint::new(
            (p.x as i64 - self.top_left_sprite.x as i64).rem_euclid(grid_width),
            (p.y as i64 - self.top_left_sprite.y as i64)
                .rem_euclid(grid_height),
        )
    }

    /// Get the sprite at the given point in the sprite grid using the given natural
    /// origin.  Also return the terrain coordinates of the sprite based on the
    /// configured top-left sprite in the grid and the sprite-terrain coverage
    fn get_sprite_at<'a>(
        &'a self,
        natural_origin: &UPoint,
        natural_x: &usize,
        natural_y: &usize,
    ) -> (&'a T::Sprite, IPoint) {
        let real_point = UPoint::new(
            (natural_origin.x + natural_x) % self.terrain_sprites_size.width,
            (natural_origin.y + natural_y) % self.terrain_sprites_size.height,
        );

        let sprite = self.get_sprite(&real_point).unwrap_or_else(|| {
            error!("Invalid sprite_coordinate {:?}", real_point);
            panic!("Index out of bounds error in terrain sprites array");
        });

        let offset = self.sprite_grid_offset_from_origin(&real_point)
            * (TERRAIN_TEXTURE_SIDE_LENGTH as i64);

        let terrain_point = &self.sprite_terrain_coverage.top_left + &offset;

        (sprite, terrain_point)
    }
}
