use std::ops::Deref;
use std::sync::Arc;
use std::time::SystemTime;
use tokio::time::{delay_for, Duration};

#[derive(Clone)]
pub struct Time(Arc<Inner>);

#[derive(derive_new::new)]
pub struct Inner {}

impl Time {
    pub fn new() -> Time {
        Time(Arc::new(Inner::new()))
    }
}

impl Deref for Time {
    type Target = Inner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Inner {
    pub fn now(&self) -> f64 {
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Uh, time went backwards?")
            .as_secs_f64()
    }

    pub async fn sleep(&self, secs: f64) {
        delay_for(Duration::from_secs_f64(secs)).await
    }
}
