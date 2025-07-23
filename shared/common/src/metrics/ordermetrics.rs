use crate::metrics::{ACTIVE_CONNECTIONS, ORDER_PROCESSING_TIME};

pub struct OrderMetrics;

impl OrderMetrics {
    pub fn record_order_processing_time(duration: std::time::Duration) {
        ORDER_PROCESSING_TIME.observe(duration.as_secs_f64());
    }

    pub fn update_active_connections(count: f64) {
        ACTIVE_CONNECTIONS.set(count);
    }
}
