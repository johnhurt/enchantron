pub trait HasMutableZLevel: 'static {
    fn set_z_level(&self, z_level: f64);
}
