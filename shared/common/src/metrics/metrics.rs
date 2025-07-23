use lazy_static::lazy_static;
use prometheus::{Counter, Encoder, Gauge, Histogram, Registry, TextEncoder, register_histogram};
use rocket::{
    get,
    http::Status,
    response::{
        content,
        status::{self, Custom},
    },
};

lazy_static! {
    static ref REGISTRY: Registry = Registry::new();
    pub static ref HTTP_REQUESTS_TOTAL: Counter =
        Counter::new("http_requests_total", "Total number of HTTP requests")
            .expect("metric can be created");
    pub static ref HTTP_REQUEST_DURATION: Histogram = register_histogram!(
        "http_request_duration_seconds",
        "HTTP request duration in seconds"
    )
    .expect("metric can be created");
    pub static ref ACTIVE_CONNECTIONS: Gauge = Gauge::new(
        "active_database_connections",
        "Number of active database connections"
    )
    .expect("metric can be created");
    pub static ref ORDER_PROCESSING_TIME: Histogram = register_histogram!(
        "order_processing_duration_seconds",
        "Time taken to process an order"
    )
    .expect("metric can be created");
}

pub fn init_metrics() {
    REGISTRY
        .register(Box::new(HTTP_REQUESTS_TOTAL.clone()))
        .unwrap();

    REGISTRY
        .register(Box::new(HTTP_REQUEST_DURATION.clone()))
        .unwrap();

    REGISTRY
        .register(Box::new(ACTIVE_CONNECTIONS.clone()))
        .unwrap();

    REGISTRY
        .register(Box::new(ORDER_PROCESSING_TIME.clone()))
        .unwrap();
}

#[get("/metrics")]
pub fn metrics() -> Result<content::RawText<String>, status::Custom<String>> {
    let encoder = TextEncoder::new();
    let metric_families = REGISTRY.gather();

    let mut buffer = Vec::new();

    encoder
        .encode(&metric_families, &mut buffer)
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    let response_string = String::from_utf8(buffer)
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    Ok(content::RawText(response_string))
}
