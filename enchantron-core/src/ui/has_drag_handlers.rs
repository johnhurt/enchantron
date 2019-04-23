use ui::{ HandlerRegistration, DragHandler };

pub trait HasDragHandlers : 'static {
  type R : HandlerRegistration;

  fn add_drag_handler(&self, handler: DragHandler) -> Self::R;
}