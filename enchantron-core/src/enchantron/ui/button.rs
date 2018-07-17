use enchantron::ui;

pub struct Button {
  pub click_handlers: Box<ui::HasClickHandlers>,
  pub text: Box<ui::HasText>
}


