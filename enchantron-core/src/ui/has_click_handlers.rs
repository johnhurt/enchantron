use ui::HandlerRegistration;

pub trait HasClickHandlers : 'static {
  type R : HandlerRegistration;

  fn add_click_handler(&self, handler: Box<Fn() + 'static>) -> Self::R;
}