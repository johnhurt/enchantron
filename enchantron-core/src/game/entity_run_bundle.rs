use super::{Entity, EntityData, EntityMessage, Services};
use tokio::sync::mpsc::Receiver;

/// Collection of all the things needed to run an entity
#[derive(Debug, derive_new::new)]
pub struct EntityRunBundle {
    pub entity: Entity,
    pub entity_data: EntityData,
    pub entity_message_source: Receiver<EntityMessage>,
    pub services: Services,
}
