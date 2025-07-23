use std::time::Instant;

use rocket::{
    Data, Request, Response,
    fairing::{Fairing, Info, Kind},
};

use crate::metrics::{HTTP_REQUEST_DURATION, HTTP_REQUESTS_TOTAL};

pub struct MetricsMiddleware;

#[rocket::async_trait]
impl Fairing for MetricsMiddleware {
    fn info(&self) -> Info {
        Info {
            name: "Metrics Middleware",
            kind: Kind::Request | Kind::Response,
        }
    }

    async fn on_request(&self, req: &mut Request<'_>, _data: &mut Data<'_>) {
        req.local_cache(|| Instant::now());
    }

    async fn on_response<'r>(&self, req: &'r Request<'_>, _res: &mut Response<'r>) {
        let start_time = req.local_cache(|| Instant::now());
        let duration = start_time.elapsed().as_secs_f64();

        HTTP_REQUESTS_TOTAL.inc();
        HTTP_REQUEST_DURATION.observe(duration);
    }
}
