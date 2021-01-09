use crate::game::{
    Entity, EntityMessage, EntityService, MessageService, Player, Services,
};
use crate::ui::{Tap, TapEvent, TouchEvent};

/// Presenter for the entity the game ui is currently focused on
pub struct FocusedEntityPresenter {
    /// the player is the default focused entity
    pub player: Player,
    pub focused_entity: Option<Entity>,

    pub entity_service: EntityService,
    pub message_service: MessageService,
}

impl FocusedEntityPresenter {
    pub fn new(services: Services) -> FocusedEntityPresenter {
        let entity_service = services.entity_service();

        FocusedEntityPresenter {
            player: entity_service.get_player(),
            focused_entity: None,
            entity_service,
            message_service: services.message_service(),
        }
    }

    pub async fn on_touch_event(&mut self, touch_event: &TouchEvent) {
        if let Some(tap_event) = TapEvent::from_touch_event(touch_event) {
            self.on_tap(tap_event).await;
        }
    }

    async fn on_tap(&mut self, tap_event: TapEvent) {
        let TapEvent {
            tap: Tap { point, .. },
            other_tap_opt,
        } = tap_event;

        self.message_service
            .send_message(
                &self.player.entity,
                EntityMessage::GoalSet(point.viewport_point),
            )
            .await;

        if let Some(Tap {
            point: other_point, ..
        }) = other_tap_opt
        {
            self.message_service
                .send_message(
                    &self.player.entity,
                    EntityMessage::GoalSet(other_point.viewport_point),
                )
                .await;
        }
    }
}
