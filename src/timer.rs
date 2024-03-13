use std::time::Instant;

pub(crate) struct Timer {
    instant: Instant,
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            instant: Instant::now(),
        }
    }
    pub fn _true_every_n_seconds(&self, n: u64) -> bool {
        self.instant.elapsed().as_secs() % n == 0
    }

    pub fn switch_every_n_seconds(&self, n: u64) -> bool {
        self.instant.elapsed().as_secs() % n * 2 >= n
    }

    pub fn switch_every_n_millis(&self, n: u128) -> bool {
        self.instant.elapsed().as_millis() % n * 2 >= n
    }

    pub fn switch_every_n_for_m_millis(&self, n: u128, m: u128) -> bool {
        self.instant.elapsed().as_millis() % n >= m
    }

    pub fn _time_as_millis(&self) -> u128 {
        self.instant.elapsed().as_millis()
    }
}
