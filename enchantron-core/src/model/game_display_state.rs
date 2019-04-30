use native::Texture;

use model::{
    Card, CardFindResponse, DraggedCardDisplayState, Point, Rect, Size,
};

use ui::{Sprite, UiCard};

const MAX_DIST_FROM_PLAY_AREA_FRAC_OF_CARD_HEIGHT: f64 = 0.5;

#[derive(Getters, MutGetters, Setters)]
pub struct GameDisplayState<S>
where
    S: Sprite,
{
    #[get = "pub"]
    #[get_mut = "pub"]
    size: Size,
    #[get = "pub"]
    #[set = "pub"]
    card_aspect_ratio: f64,
    #[get = "pub"]
    #[set = "pub"]
    border_thickness: f64,

    #[get = "pub"]
    #[get_mut = "pub"]
    cards_in_play: Vec<UiCard<S>>,

    #[get = "pub"]
    #[get_mut = "pub"]
    supply_cards: Vec<UiCard<S>>,

    #[get = "pub"]
    #[set = "pub"]
    #[get_mut = "pub"]
    card_in_flight: Option<DraggedCardDisplayState<S>>,

    /// List of points that represent the places that cards can be added
    /// to the playing area
    #[get = "pub"]
    #[set = "pub"]
    #[get_mut = "pub"]
    bind_points: Vec<(f64, Vec<(f64, usize)>)>,

    #[get = "pub"]
    #[get_mut = "pub"]
    supply_card_size: Size,
    #[get = "pub"]
    #[set = "pub"]
    supply_card_spacing: f64,
    #[get = "pub"]
    #[set = "pub"]
    supply_cards_per_row: f64,

    #[get = "pub"]
    #[set = "pub"]
    supply_row_count: f64,
    #[get = "pub"]
    #[get_mut = "pub"]
    supply_rect: Rect,

    #[get = "pub"]
    #[get_mut = "pub"]
    play_card_size: Size,
    #[get = "pub"]
    #[set = "pub"]
    play_card_slots: f64,
    #[get = "pub"]
    #[set = "pub"]
    play_card_spacing: f64,
    #[get = "pub"]
    #[set = "pub"]
    play_cards_per_row: f64,

    #[get = "pub"]
    #[set = "pub"]
    play_area_row_count: f64,
    #[get = "pub"]
    #[get_mut = "pub"]
    play_area_rect: Rect,

    #[get = "pub"]
    #[set = "pub"]
    game_area_rect: Rect,
    #[get = "pub"]
    #[set = "pub"]
    card_area_rect: Rect,
    #[get = "pub"]
    #[set = "pub"]
    tex_area_rect: Rect,
}

impl<S> Default for GameDisplayState<S>
where
    S: Sprite,
{
    fn default() -> GameDisplayState<S> {
        GameDisplayState {
            size: Size::default(),

            card_aspect_ratio: 0.,
            border_thickness: 0.,

            cards_in_play: Vec::default(),
            supply_cards: Vec::default(),
            card_in_flight: Option::default(),

            bind_points: Vec::default(),

            supply_card_size: Size::default(),
            supply_card_spacing: 0.,
            supply_cards_per_row: 0.,

            play_card_size: Size::default(),
            play_card_spacing: 0.,
            play_cards_per_row: 0.,
            play_card_slots: 0.,

            supply_row_count: 0.,
            supply_rect: Rect::default(),

            play_area_row_count: 0.,
            play_area_rect: Rect::default(),

            game_area_rect: Rect::default(),
            card_area_rect: Rect::default(),
            tex_area_rect: Rect::default(),
        }
    }
}

impl<S, T> GameDisplayState<S>
where
    T: Texture,
    S: Sprite<T = T>,
{
    /// Get the bind point closest to the given point or none if the point
    /// is too far away from the play area
    pub fn get_closest_bind_point(
        &self,
        point: &Point,
        current_bind_point: &Option<usize>,
    ) -> Option<usize> {
        let mut min_row_dist: f64 = 1_000_000.;
        let mut best_row: Option<&Vec<(f64, usize)>> = None;

        let x = &point.x;
        let y = &point.y;

        let threshold_dist = if current_bind_point.is_some() {
            self.play_card_size.height
                * MAX_DIST_FROM_PLAY_AREA_FRAC_OF_CARD_HEIGHT
        } else {
            0.
        };

        if self.play_area_rect.distance_to(point) > threshold_dist {
            return None;
        }

        for (row, columns) in &self.bind_points {
            let row_dist = (row - y).abs();
            if row_dist < min_row_dist {
                min_row_dist = row_dist;
                best_row = Some(columns)
            }
        }

        match best_row {
            None => None,
            Some(columns) => {
                let mut min_col_dist: f64 = 1_000_000.;
                let mut best_point: Option<usize> = None;
                for (col, bind_point) in columns {
                    let col_dist = (col - x).abs();

                    if col_dist < min_col_dist {
                        min_col_dist = col_dist;
                        best_point = Some(bind_point.clone());
                    }
                }

                best_point
            }
        }
    }

    /// Get the rectangle that the supply card with the current index should
    /// be displayed within.  This method will return none if the i >= size
    /// of the supply card array
    pub fn get_supply_card_rect_by_index(&self, i: &usize) -> Option<Rect> {
        if *i >= self.supply_cards.len() {
            return None;
        }

        let col = (i % (self.supply_cards_per_row as usize)) as f64;
        let row = (i / (self.supply_cards_per_row as usize)) as f64;
        let left = self.supply_rect.top_left.x
            + (self.supply_card_size.width + self.supply_card_spacing) * col;
        let top = self.supply_rect.top_left.y
            + (self.supply_card_size.height + self.supply_card_spacing) * row;

        Some(Rect {
            top_left: Point { x: left, y: top },
            size: self.supply_card_size.clone(),
        })
    }

    pub fn get_supply_card_rect_by_card(&self, card: &Card) -> Option<Rect> {
        let card_index_opt = self
            .supply_cards
            .iter()
            .enumerate()
            .filter_map(|(i, ui_card)| {
                if ui_card.card() == card {
                    Some(i)
                } else {
                    None
                }
            })
            .nth(0);

        match card_index_opt {
            Some(i) => self.get_supply_card_rect_by_index(&i),
            _ => None,
        }
    }

    pub fn get_play_card_rect_by_index(&self, i: &usize) -> Option<Rect> {
        if *i >= (self.play_card_slots as usize) {
            return None;
        }

        let col = (i % (self.play_cards_per_row as usize)) as f64;
        let row = (i / (self.play_cards_per_row as usize)) as f64;
        let left = self.play_area_rect.top_left.x
            + (self.play_card_size.width + self.play_card_spacing) * col;
        let top = self.play_area_rect.top_left.y
            + (self.play_card_size.height + self.play_card_spacing) * row;

        Some(Rect {
            top_left: Point { x: left, y: top },
            size: self.play_card_size.clone(),
        })
    }

    /// Get the supply card at the given point if there is one in a tuple with
    /// the point within the card where the touch falls
    fn get_supply_card_at(&self, point: &Point) -> Option<CardFindResponse> {
        let card_row = ((point.y - self.supply_rect.top_left.y)
            / (self.supply_card_size.height + self.supply_card_spacing))
            .floor();

        if card_row < 0. || card_row > self.supply_row_count - 0.5 {
            return None;
        }

        let card_y = point.y
            - self.supply_rect.top_left.y
            - card_row
                * (self.supply_card_size.height + self.supply_card_spacing);

        if card_y > self.supply_card_size.height {
            return None;
        }

        let card_col = ((point.x - self.supply_rect.top_left.x)
            / (self.supply_card_size.width + self.supply_card_spacing))
            .floor();

        if card_col < 0. {
            return None;
        }

        let card_index =
            (card_row * self.supply_cards_per_row + card_col).round() as usize;

        if card_index >= self.supply_cards.len() {
            return None;
        }

        let card_x = point.x
            - self.supply_rect.top_left.x
            - card_col
                * (self.supply_card_size.width + self.supply_card_spacing);

        if card_x > self.supply_card_size.width {
            return None;
        }

        self.supply_cards
            .get(card_index)
            .map(|card| CardFindResponse {
                card: card.card().clone(),
                play_area_ord: None,
                card_rect: self
                    .get_supply_card_rect_by_index(&card_index)
                    .unwrap(),
                point_in_card: Point::new(card_x, card_y),
            })
    }

    /// Get the play card at the given point if there is one
    pub fn get_play_card_at(&self, point: &Point) -> Option<CardFindResponse> {
        let card_row = ((point.y - self.play_area_rect.top_left.y)
            / (self.play_card_size.height + self.play_card_spacing))
            .floor();

        if card_row < 0. || card_row > self.play_area_row_count - 0.5 {
            return None;
        }

        let card_y = point.y
            - self.play_area_rect.top_left.y
            - card_row * (self.play_card_size.height + self.play_card_spacing);

        if card_y > self.play_card_size.height {
            return None;
        }

        let card_col = ((point.x - self.play_area_rect.top_left.x)
            / (self.play_card_size.width + self.play_card_spacing))
            .floor();

        if card_col < 0. {
            return None;
        }

        let card_index =
            (card_row * self.play_cards_per_row + card_col).round() as usize;

        if card_index >= self.cards_in_play.len() {
            return None;
        }

        let card_x = point.x
            - self.play_area_rect.top_left.x
            - card_col * (self.play_card_size.width + self.play_card_spacing);

        if card_x > self.play_card_size.width {
            return None;
        }

        self.cards_in_play
            .get(card_index)
            .map(|card| CardFindResponse {
                card: card.card().clone(),
                play_area_ord: Some(card_index),
                card_rect: self
                    .get_play_card_rect_by_index(&card_index)
                    .unwrap(),
                point_in_card: Point::new(card_x, card_y),
            })
    }

    /// Get the card at the given point or none if no card is found at the given
    /// Point
    pub fn get_card_at(&self, point: &Point) -> Option<CardFindResponse> {
        self.get_supply_card_at(point)
            .or(self.get_play_card_at(point))
    }

    pub fn to_string(&self) -> String {
        cards_to_string(&self.cards_in_play.iter().map(UiCard::card).collect())
    }
}

fn cards_to_string(cards: &Vec<&Card>) -> String {
    let mut result = String::with_capacity(cards.len());

    for ref card in cards {
        match card {
            Card::Number(num, _) => {
                result.push((((*num as u8) + ('0' as u8)) as char));
            }
            Card::Plus => result.push('+'),
            Card::Minus => result.push('-'),
            Card::Times => result.push('*'),
            Card::Divide => result.push('/'),
            Card::ParenL => result.push('('),
            Card::ParenR => result.push(')'),
            Card::Power => result.push('^'),
            Card::Radical => result.push('√'),
            Card::Inverse => result.push_str("^-1"),
            Card::Factorial => result.push('!'),
            Card::Decimal => result.push('.'),
        };
    }

    result
}

#[test]
fn test_to_str() {
    let mut cards = vec![
        required_number_card!(4),
        required_number_card!(4),
        required_number_card!(4),
        required_number_card!(4),
    ];

    assert_eq!(cards_to_string(&cards.iter().collect()), "4444");

    cards = vec![
        required_number_card!(4),
        Card::Divide,
        required_number_card!(4),
        Card::Plus,
        required_number_card!(4),
        Card::Times,
        Card::ParenL,
        Card::Radical,
        Card::Decimal,
        required_number_card!(4),
        Card::ParenR,
        Card::Inverse,
    ];

    assert_eq!(cards_to_string(&cards.iter().collect()), "4/4+4*(√.4)^-1");
}
