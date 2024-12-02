use tracing::level_filters::LevelFilter;
use tracing_subscriber::{
    fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt as _, Layer as _,
};

use crate::AppState;

pub fn init(_state: &AppState) {
    let layer = Layer::new().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();
}
