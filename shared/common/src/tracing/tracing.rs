use std::error::Error;

pub fn init_tracing(service_name: &str) -> Result<(), Box<dyn Error>> {
    opentelemetry_jaeger::
}