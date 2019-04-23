use model::{
  Rect,
  Card,
};
use native::{ Texture, Textures };
use ui::{
  Sprite,
  SpriteSource,
  HasMutableVisibility
};

const SYMBOL_WIDTH_FRAC : f64 = 0.6;

#[derive(Getters,Setters)]
pub struct UiCard<S>
    where
        S: Sprite {
  card_sprite: S,
  symbol_sprite: S,
  symbol_texture_aspect_ratio: f64,

  #[get = "pub"] card: Card,

  #[get = "pub"] required_play_card: bool,
  #[get = "pub"] #[set = "pub"] play_area_ord: Option<usize>
}

impl <T,S> UiCard<S>
    where
        T: Texture,
        S: Sprite<T = T> {

  pub fn new(
      card: Card,
      play_area_ord: Option<usize>,
      required_play_card: bool,
      textures: &Textures<T>,
      sprite_source: &SpriteSource<T = T, S = S>)
          -> UiCard<S> {

    let card_sprite = sprite_source.create_sprite();
    let symbol_sprite = sprite_source.create_sprite();

    card_sprite.set_texture(textures.card());

    let symbol_texture = match card {
      Card::Number(val, _) => {
        match val {
          0 => textures.symbols().zero(),
          1 => textures.symbols().one(),
          2 => textures.symbols().two(),
          3 => textures.symbols().three(),
          4 => textures.symbols().four(),
          5 => textures.symbols().five(),
          6 => textures.symbols().six(),
          7 => textures.symbols().seven(),
          8 => textures.symbols().eight(),
          9 => textures.symbols().nine(),
          _ => panic!("Invalid number {}", val)
        }
      },
      Card::Decimal => textures.symbols().decimal(),
      Card::Plus => textures.symbols().plus(),
      Card::Minus => textures.symbols().minus(),
      Card::Times => textures.symbols().times(),
      Card::Divide => textures.symbols().divide(),
      Card::Power => textures.symbols().power(),
      Card::Radical => textures.symbols().radical(),
      Card::ParenL => textures.symbols().paren_l(),
      Card::ParenR => textures.symbols().paren_r(),
      Card::Inverse => textures.symbols().inverse(),
      Card::Factorial => textures.symbols().factorial()
    };

    symbol_sprite.set_texture(symbol_texture);

    UiCard {
      card: card,
      play_area_ord: play_area_ord,
      required_play_card: required_play_card,
      card_sprite: card_sprite,
      symbol_sprite: symbol_sprite,
      symbol_texture_aspect_ratio: symbol_texture.get_aspect_ratio()
    }

  }

  /// Set the location and size of this card immediately.  This will set the
  /// location and shape of the card background sprite and the symbol sprite
  /// of this card will be aspet scaled to fit in the middle
  pub fn set_rect(&self, rect: &Rect) {
    self.set_rect_animated(rect, 0.);
  }

  /// Animate the movement of this card from its current location to the
  /// given location after the given number of seconds.  This will set the
  /// location and shape of the card background sprite and the symbol sprite
  /// of this card will be aspet scaled to fit in the middle
  pub fn set_rect_animated(&self, rect: &Rect, duration_seconds: f64) {
    let left = rect.top_left.x;
    let top = rect.top_left.y;
    let width = rect.size.width;
    let height = rect.size.height;

    self.card_sprite.set_size_animated(width, height, duration_seconds);
    self.card_sprite.set_location_animated(left, top, duration_seconds);

    let sym_ar = self.symbol_texture_aspect_ratio;

    let sym_width = width * SYMBOL_WIDTH_FRAC;
    let sym_height = sym_width / sym_ar;
    let sym_left = left + width / 2.0 - sym_width / 2.0;
    let sym_top = top + height / 2.0 - sym_height / 2.0;

    self.symbol_sprite.set_size_animated(
        sym_width,
        sym_height,
        duration_seconds);
    self.symbol_sprite.set_location_animated(
        sym_left,
        sym_top,
        duration_seconds);
  }
}

impl <S> HasMutableVisibility for UiCard<S> where S : Sprite {

  fn set_visible(&self, visible: bool) {
    self.card_sprite.set_visible(visible);
    self.symbol_sprite.set_visible(visible);
  }

}
