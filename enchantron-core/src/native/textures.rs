
use native::{Texture, TextureLoader};

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

define_texture_atlas!(Card(x_tile_count: 1, y_tile_count: 1) {
  card(left: 0, top: 0, width: 1, height: 1)
});

define_texture_atlas!(Symbols (x_tile_count: 6, y_tile_count: 6) {
  zero(left: 0, top: 0, width: 1, height: 1),
  one(left: 1, top: 0, width: 1, height: 1),
  two(left: 2, top: 0, width: 1, height: 1),
  three(left: 3, top: 0, width: 1, height: 1),
  four(left: 4, top: 0, width: 1, height: 1),
  five(left: 5, top: 0, width: 1, height: 1),
  six(left: 0, top: 1, width: 1, height: 1),
  seven(left: 1, top: 1, width: 1, height: 1),
  eight(left: 2, top: 1, width: 1, height: 1),
  nine(left: 3, top: 1, width: 1, height: 1),
  plus(left: 4, top: 1, width: 1, height: 1),
  minus(left: 5, top: 1, width: 1, height: 1),
  times(left: 0, top: 2, width: 1, height: 1),
  divide(left: 1, top: 2, width: 1, height: 1),
  power(left: 2, top: 2, width: 1, height: 1),
  radical(left: 3, top: 2, width: 1, height: 1),
  paren_l(left: 4, top: 2, width: 1, height: 1),
  paren_r(left: 5, top: 2, width: 1, height: 1),
  inverse(left: 0, top: 3, width: 1, height: 1),
  factorial(left: 1, top: 3, width: 1, height: 1),
  decimal(left: 2, top: 3, width: 1, height: 1)
});

pub struct Textures<T: Texture> {
  card: T,
  symbols: Symbols<T>
}

impl <T: Texture> Textures<T> {

  pub fn new(texture_loader: &TextureLoader<T = T>, progress_callback: &Fn(f64))
      -> Textures<T> {

    let card_atlas = Card::new(texture_loader.load_texture(
        String::from("Card.png")), |p| progress_callback(p / 2.));
    let symbols_atlas = Symbols::new(texture_loader.load_texture(
        String::from("Symbols.png")), |p| progress_callback(0.5 + p / 2.));

    Textures{
      card: card_atlas.card,
      symbols: symbols_atlas
    }
  }

  pub fn card(&self) -> &T {
    &self.card
  }

  pub fn symbols(&self) -> &Symbols<T> {
    &self.symbols
  }

}