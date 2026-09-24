use http_mambo::types::{HttpRequest, HttpResponse};
use std::collections::HashMap;
/**
 *  A test handler for the home route.
 *  Handle an HTTP request and return an HTTP response.
 */
pub async fn home(request: HttpRequest) -> HttpResponse {
    let body = format!(
        "<html><body><h1>Welcome to HTTP Mambo!</h1><p>You requested: {}</p></body></html>",
        request.request_lines.path
    );

    let mut headers = HashMap::new();
    headers.insert("Content-Type".to_string(), "text/html".to_string());
    //headers.insert("Content-Length".to_string(), 200.to_string()); // should ignored by the response builder

    HttpResponse::new().with_body(body).with_headers(headers)
    //or
    //HttpResponse::with_body(HttpResponse::new(), body)
    //or
    //HttpResponse::with_status_and_body(StatusCode::OK, body)
}
