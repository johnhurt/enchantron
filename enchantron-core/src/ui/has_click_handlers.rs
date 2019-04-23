use ui::{ HandlerRegistration, ClickHandler };

pub trait HasClickHandlers : 'static {
  type R : HandlerRegistration;

  fn add_click_handler(&self, handler: ClickHandler ) -> Self::R;
}