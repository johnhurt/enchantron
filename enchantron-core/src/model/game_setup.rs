use model::Card;

#[derive(Default)]
pub struct GameSetup {
    goal: i64,
    required_cards: Vec<Card>,
    supply_cards: Vec<Card>,
}

impl GameSetup {
    pub fn simple_new(goal: i64, required_cards: Vec<i64>) -> GameSetup {
        GameSetup::new(
            goal,
            required_cards
                .iter()
                .map(|v| required_number_card!(*v))
                .collect(),
            all_non_number_cards!(),
        )
    }

    pub fn new(
        goal: i64,
        required_cards: Vec<Card>,
        supply_cards: Vec<Card>,
    ) -> GameSetup {
        GameSetup {
            goal: goal,
            required_cards: required_cards,
            supply_cards: supply_cards,
        }
    }

    pub fn goal(&self) -> i64 {
        self.goal
    }

    pub fn required_cards(&self) -> &Vec<Card> {
        &self.required_cards
    }

    pub fn supply_cards(&self) -> &Vec<Card> {
        &self.supply_cards
    }
}
