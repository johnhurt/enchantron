use super::{Texture, TextureLoader};

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
              height: $height:expr) ),*
      }
  ) => {

    #[allow(dead_code)]
    pub struct $texture_type<T: Texture> {
      $(
        $name: T,
      )*
    }

    impl <T: Texture> $texture_type<T> {
      pub fn new<F>(texture_atlas: T, progress_callback: F)
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
        pub fn $name(&self) -> &T { &self.$name }
      )*
    }

  };
}

define_texture_atlas!(Overworld(x_tile_count: 40, y_tile_count: 36) {
  grass(left: 0, top: 0, width: 1, height: 1)
});

pub struct Textures<T: Texture> {
    pub overworld: Overworld<T>,
}

impl<T: Texture> Textures<T> {
    pub fn new(
        texture_loader: &TextureLoader<T = T>,
        progress_callback: &Fn(f64),
    ) -> Textures<T> {
        let overworld = Overworld::new(
            texture_loader.load_texture(String::from("overworld.png")),
            |p| progress_callback(p),
        );
        Textures {
            overworld: overworld,
        }
    }
}
