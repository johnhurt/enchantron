use crate::game::{Player, Entity, Services};

/// Presenter for the entity the game ui is currently focused on
pub struct FocusedEntityPresenter {

    /// the player is the default focused entity
    pub player: Player,
    pub focused_entity: Option<Entity>

    pub services: Services

}

impl FocusedEntityPresenter {

    pub fn new(services: Services) -> FocusedEntityPresenter {
        FocusedEntityPresenter {
            player,
            focused_entity: None,
            services: Services
        }
    }

}