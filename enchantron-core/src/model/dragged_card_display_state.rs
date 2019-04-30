use model::{Point, Rect};

use ui::{Sprite, UiCard};

#[derive(Getters, MutGetters, Setters)]
pub struct DraggedCardDisplayState<S: Sprite> {
    #[get = "pub"]
    card: UiCard<S>,

    #[get = "pub"]
    orig_play_area_ord: Option<usize>,
    #[get = "pub"]
    #[set = "pub"]
    #[get_mut = "pub"]
    play_area_ord: Option<usize>,

    #[get = "pub"]
    rect: Rect,

    #[get = "pub"]
    drag_point_from_top_left_frac: Point,
}

impl<S: Sprite> DraggedCardDisplayState<S> {
    pub fn new(
        card: UiCard<S>,
        original_play_area_ord: Option<usize>,
        rect: &Rect,
        drag_point_in_card: &Point,
    ) -> DraggedCardDisplayState<S> {
        let drag_point_from_top_left_frac = Point {
            x: drag_point_in_card.x / rect.size.width,
            y: drag_point_in_card.y / rect.size.height,
        };

        let mut result = DraggedCardDisplayState {
            card: card,

            orig_play_area_ord: original_play_area_ord,
            play_area_ord: None,

            rect: rect.clone(),

            drag_point_from_top_left_frac: drag_point_from_top_left_frac,
        };

        result.update_card();

        result
    }

    pub fn drag_move(&mut self, drag_x: f64, drag_y: f64) {
        self.rect.top_left.x = drag_x
            - self.drag_point_from_top_left_frac.x * self.rect.size.width;
        self.rect.top_left.y = drag_y
            - self.drag_point_from_top_left_frac.y * self.rect.size.height;

        self.update_card();
    }

    /// Scale the card with a constant aspect ratio so that the width of the
    /// card is the given width and the location and propportional distance from
    /// the top and left of the card to the drag point does not change
    pub fn scale_width_to(&mut self, new_width: f64) {
        let new_height = new_width / self.rect.size.aspect_ratio();
        let new_left = self.rect.top_left.x
            + self.drag_point_from_top_left_frac.x
                * (self.rect.size.width - new_width);
        let new_top = self.rect.top_left.y
            + self.drag_point_from_top_left_frac.y
                * (self.rect.size.height - new_height);
        self.rect.top_left.x = new_left;
        self.rect.top_left.y = new_top;
        self.rect.size.width = new_width;
        self.rect.size.height = new_height;

        self.update_card();
    }

    fn update_card(&mut self) {
        self.card.set_rect(&self.rect);
    }

    pub fn card_center(&self) -> Point {
        self.rect.center()
    }

    pub fn take_card(self) -> UiCard<S> {
        self.card
    }
}
