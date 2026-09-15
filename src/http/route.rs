use crate::types::{HttpRequest, Route};

impl Route {
    // Check if the route matches the request based on the request parts and the route parts e.g /example/123 should match /example/:id
    pub fn path_matches(&self, request: &HttpRequest) -> bool {
        let route_parts: Vec<&str> = self.path.split('/').collect();
        let request_parts: Vec<&str> = request.request_lines.path.split('/').collect();

        if route_parts.len() != request_parts.len() {
            return false;
        }

        for (route_part, request_part) in route_parts.iter().zip(request_parts.iter()) {
            if route_part.starts_with(':') {
                continue;
            }

            if route_part != request_part {
                return false;
            }
        }

        true
    }
}
