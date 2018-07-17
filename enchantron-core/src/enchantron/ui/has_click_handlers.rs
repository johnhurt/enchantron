use enchantron::event::HandlerRegistration;

pub trait HasClickHandlers : 'static {
  fn add_click_handler(&mut self, handler: Box<Fn() + 'static>) 
      -> HandlerRegistration;
}