use super::Gor;
use tokio::runtime::Runtime;
use tokio::time::{sleep, Duration};

#[derive(Clone, Debug)]
pub struct Time(Gor<Runtime>);

fn u64_millis_to_secs_f64(millis: u64) -> f64 {
    millis as f64 / 1000.
}

impl Time {
    pub fn new(runtime_handle: Gor<Runtime>) -> Time {
        Time(runtime_handle)
    }

    pub fn now(&self) -> f64 {
        u64_millis_to_secs_f64(self.0.elapsed_millis())
    }

    pub async fn sleep(&self, secs: f64) {
        sleep(Duration::from_secs_f64(secs)).await
    }

    pub async fn sleep_until(&self, wake_time: f64) {
        self.sleep((wake_time - self.now()).max(0.)).await
    }
}
