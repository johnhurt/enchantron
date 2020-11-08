use crate::game::{Entity, EntityService, Player, Services};
use crate::ui::{TapEvent, TouchEvent};

/// Presenter for the entity the game ui is currently focused on
pub struct FocusedEntityPresenter {
    /// the player is the default focused entity
    pub player: Player,
    pub focused_entity: Option<Entity>,

    pub entity_service: EntityService,
}

impl FocusedEntityPresenter {
    pub fn new(services: Services) -> FocusedEntityPresenter {
        let entity_service = services.entity_service();

        FocusedEntityPresenter {
            player: entity_service.get_player(),
            focused_entity: None,
            entity_service,
        }
    }

    pub fn on_touch_event(
        &mut self,
        touch_event: TouchEvent,
    ) -> Option<TouchEvent> {
        if let Some(tap_event) = TapEvent::from_touch_event(touch_event) {
            None
        } else {
            Some(touch_event)
        }
    }
}
