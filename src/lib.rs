use serde::{Deserialize, Serialize};

pub mod htmx;

#[derive(Serialize, Deserialize, Clone)]
pub struct AppState {}
