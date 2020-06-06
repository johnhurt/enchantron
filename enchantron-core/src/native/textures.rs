use super::{HasIntSize, ResourceLoader, Texture};
use crate::view_types::ViewTypes;

const CENTER: usize = 0;

macro_rules! count {
    ($h:expr) => (1);
    ($h:expr, $($t:expr),*) =>
        (1 + count!($($t),*));
}

macro_rules! define_texture_atlas {
  ($texture_type:ident (
      x_tile_count: $x_tile_count:expr,
      y_tile_count: $y_tile_count:expr ) {
          $( $name:ident(
              left: $left:expr,
              top: $top:expr,
              width: $width:expr,
              height: $height:expr
              $(, register: $registration:ident )?) ),*
      }
  ) => {

    #[allow(dead_code)]
    pub struct $texture_type<T: ViewTypes> {
      $(
        $name: T::Texture,
      )*
    }

    impl <T: ViewTypes> $texture_type<T> {
      pub fn new<F>(texture_atlas: T::Texture, progress_callback: F)
          -> $texture_type<T>
          where F : Fn(f64) {
        let tex_width = texture_atlas.get_width();
        let tex_height = texture_atlas.get_height();

        let tile_width = tex_width / ($x_tile_count);
        let tile_height = tex_height / ($y_tile_count);

        let sub_tex_count = (count!($($name),*) ) as f64;

        let mut counter : f64 = 0.;

        progress_callback(counter / sub_tex_count);

        $(
          let $name = texture_atlas.get_sub_texture(
                ($left) * tile_width,
                ($top) * tile_height,
                ($width) * tile_width,
                ($height) * tile_height);

          $(
              if $registration == CENTER {
                $name.set_center_registration(true);
              }
          )?
          counter += 1.;
          progress_callback(counter / sub_tex_count);
        )*

        $texture_type {
          $(
            $name: $name,
          )*
        }
      }

      $(
        #[allow(dead_code)]
        pub fn $name(&self) -> &T::Texture { &self.$name }
      )*
    }

  };
}

define_texture_atlas!(Overworld(x_tile_count: 40, y_tile_count: 36) {
  grass(left: 0, top: 0, width: 1, height: 1),
  dirt(left: 2, top: 32, width: 1, height: 1)
});

define_texture_atlas!(Character(x_tile_count: 17, y_tile_count: 16) {
    south_rest(left: 0, top: 0, width: 1, height: 2, register: CENTER),
    south_step_left(left: 1, top: 0, width: 1, height: 2, register: CENTER),
    south_step_mid(left: 2, top: 0, width: 1, height: 2, register: CENTER),
    south_step_right(left: 3, top: 0, width: 1, height: 2, register: CENTER)
});

pub struct Textures<T: ViewTypes> {
    pub overworld: Overworld<T>,
    pub character: Character<T>,
}

impl<T: ViewTypes> Textures<T> {
    pub fn new(
        texture_loader: &T::ResourceLoader,
        progress_callback: &impl Fn(f64),
    ) -> Textures<T> {
        let overworld = Overworld::new(
            texture_loader.load_texture(String::from("overworld.png")),
            |p| progress_callback(p),
        );

        let character = Character::new(
            texture_loader.load_texture(String::from("character.png")),
            |p| progress_callback(p),
        );

        Textures {
            overworld: overworld,
            character: character,
        }
    }
}
