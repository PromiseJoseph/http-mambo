use crate::types::{Handler, HttpMethod, HttpRequest, HttpResponse, Route, Router, StatusCode};
use std::collections::HashMap;

impl Router {
    pub fn new() -> Self {
        Self { routes: Vec::new() }
    }

    fn add_route(&mut self, route: Route) {
        self.routes.push(route);
    }

    // Create a new route
    fn route(path: &str, handler: Handler, method: HttpMethod) -> Route {
        Route {
            path: path.to_string(),
            method: method,
            handler,
        }
    }

    // Handles  incoming HTTP requests and return the appropriate response
    pub fn handle_request(&self, request: &HttpRequest) -> HttpResponse {
        let mut path_found = false; // Flag to check if the path was found but the method was not allowed

        // let mut method_found = false;
        let mut allowed_methods: Vec<HttpMethod> = Vec::new(); // Vector to store allowed methods for the found path

        for route in &self.routes {
            println!(
                "Route path: {}, method: {:?}, request path: {}, request method: {:?}",
                route.path, route.method, request.request_lines.path, request.request_lines.method
            ); // Debugging output to check the route and request details, to be removed later

            if route.path_matches(&request) {
                path_found = true;
                allowed_methods.push(route.method.clone()); // Store the allowed method for the found path

                if route.method == request.request_lines.method {
                    return (route.handler)(request);
                }
            }
        }

        if path_found {
            return HttpResponse {
                status_code: StatusCode::METHOD_NOT_ALLOWED,
                headers: HashMap::from([(
                    "Allow".to_string(),
                    allowed_methods
                        .iter()
                        .map(|m| format!("{:?}", m))
                        .collect::<Vec<String>>()
                        .join(", "),
                )]),
                body: "Method Not Allowed".to_string(),
            };
        }
        HttpResponse {
            status_code: StatusCode::NOT_FOUND,
            headers: HashMap::new(),
            body: "Not Found".to_string(),
        }
    }

    pub fn get(&mut self, path: &str, handler: Handler) {
        self.add_route(Self::route(path, handler, HttpMethod::GET));
    }

    pub fn post(&mut self, path: &str, handler: Handler) {
        self.add_route(Self::route(path, handler, HttpMethod::POST));
    }

    pub fn put(&mut self, path: &str, handler: Handler) {
        self.add_route(Self::route(path, handler, HttpMethod::PUT));
    }

    pub fn delete(&mut self, path: &str, handler: Handler) {
        self.add_route(Self::route(path, handler, HttpMethod::DELETE));
    }

    pub fn patch(&mut self, path: &str, handler: Handler) {
        self.add_route(Self::route(path, handler, HttpMethod::PATCH));
    }

    pub fn options(&mut self, path: &str, handler: Handler) {
        self.add_route(Self::route(path, handler, HttpMethod::OPTIONS));
    }

    pub fn head(&mut self, path: &str, handler: Handler) {
        self.add_route(Self::route(path, handler, HttpMethod::HEAD));
    }
}
