
macro_rules! required_number_card {
  ($num:expr) => {
    Card::Number(Card::verify_number($num), true);
  };
}

macro_rules! define_cards {
  ($($op:ident),*) => {

    #[derive(Clone, PartialEq)]
    pub enum Card {
      Number(i64, bool),
      $(
        $op,
      )*
    }

    $(
    macro_rules! $op {
      () => {
        Card::$op
      };
    }
    )*

    macro_rules! all_non_number_cards {
      () => {
        vec![$( $op!()),*]
      };
    }
  };
}

define_cards!(
  Plus,
  Minus,
  Times,
  Divide,
  ParenL,
  ParenR,
  Decimal,
  Power,
  Radical,
  Inverse,
  Factorial
);

impl Card {
  pub fn verify_number(num: i64) -> i64 {
    if num > 9 || num < 0 {
      panic!("Number cards can only have a value between 1 and 9 (inclusive)");
    }
    num
  }

  pub fn is_required_in_play(&self) -> bool {
    match self {
      Card::Number(_, required) => required.clone(),
      _ => false
    }
  }
}