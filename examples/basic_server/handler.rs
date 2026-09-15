use http_mambo::types::{HttpRequest, HttpResponse, StatusCode};
use std::collections::HashMap;
/**
 *  A test handler for the home route.
 *  Handle an HTTP request and return an HTTP response.
 */
pub fn home(request: &HttpRequest) -> HttpResponse {
    let body = format!(
        "<html><body><h1>Welcome to HTTP Mambo!</h1><p>You requested: {}</p></body></html>",
        request.request_lines.path
    );

    HttpResponse {
        status_code: StatusCode::OK,
        headers: HashMap::new(),
        body,
    }
}
