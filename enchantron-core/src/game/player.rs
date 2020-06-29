use super::LocationKey;

#[derive(derive_new::new)]
pub struct Player {
    pub location_key: LocationKey,
}
