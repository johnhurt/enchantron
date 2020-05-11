use super::GameEntitySlotKey;

#[derive(derive_new::new)]
pub struct Player {
    pub location_key: GameEntitySlotKey,
}
