use model::{Card, GameSetup};

#[derive(Default)]
pub struct GameState {
    setup: GameSetup,
    cards_in_play: Vec<Card>,
}

impl GameState {
    pub fn new(setup: GameSetup) -> GameState {
        GameState {
            cards_in_play: setup.required_cards().clone(),
            setup: setup,
        }
    }

    pub fn setup(&self) -> &GameSetup {
        &self.setup
    }

    pub fn cards_in_play(&self) -> &Vec<Card> {
        &self.cards_in_play
    }
}
