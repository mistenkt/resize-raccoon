use std::sync::{Arc, atomic::{AtomicBool, AtomicU64}, Mutex};

use crate::profile::Profile;

pub struct AppState {
    pub profiles: Arc<Mutex<Vec<Profile>>>,
    pub process_watcher_enabled: Arc<AtomicBool>,
    pub poll_rate: Arc<AtomicU64>,
}
