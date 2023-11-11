use std::sync::{
    atomic::{AtomicBool, AtomicU64},
    Arc, Mutex,
};

use crate::profile::Profile;

pub struct AppState {
    pub profiles: Arc<Mutex<Vec<Profile>>>,
    pub process_watcher_enabled: Arc<AtomicBool>,
    pub poll_rate: Arc<AtomicU64>,
}
