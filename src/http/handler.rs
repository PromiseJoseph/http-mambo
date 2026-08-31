use crate::types::HttpRequest;
use crate::types::HttpResponse;
/**
 *  Handle an HTTP request and return an HTTP response.
 */
pub fn handle_request(parsed_req: &HttpRequest) -> HttpResponse {
    let path = &parsed_req.request_lines.path;

    // sample routing logic based on the request path.. to be updated.
    match path.as_str() {
        "/" => {
            let res_body: String = format!(
                r#"
                <!DOCTYPE html>
                <html>
                <head>
                    <title> HttpMambo</title>
                </head>
                <body>
                    <h1>Welcome to HttpMambo!</h1>
                    <p>This is the home page.</p>
                </body>
                </html>                
            "#
            );
            HttpResponse {
                status_code: 200,
                headers: std::collections::HashMap::new(),
                body: res_body,
            }
        }

        _ => {
            let res_body: String = format!(
                r#"
                <!DOCTYPE html>
                <html>
                <head>
                    <title> HttpMambo</title>
                </head>
                <body>
                    <h1>404 Not Found</h1>
                    <p>The requested resource was not found on this server.</p>
                </body>
                </html>                
            "#
            );
            HttpResponse {
                status_code: 404,
                headers: std::collections::HashMap::new(),
                body: res_body,
            }
        }
    }
}
