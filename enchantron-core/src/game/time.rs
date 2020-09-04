use tokio::runtime::Handle;
use tokio::time::{delay_for, Duration};

#[derive(Clone, Debug)]
pub struct Time(Handle);

fn u64_millis_to_secs_f64(millis: u64) -> f64 {
    millis as f64 / 1000.
}

impl Time {
    pub fn new(runtime_handle: Handle) -> Time {
        Time(runtime_handle)
    }

    pub fn now(&self) -> f64 {
        u64_millis_to_secs_f64(self.0.elapsed_millis())
    }

    pub async fn sleep(&self, secs: f64) {
        delay_for(Duration::from_secs_f64(secs)).await
    }
}
