use crate::types::{Handler, HttpMethod, HttpRequest, HttpResponse, Route, Router};
use std::collections::HashMap;

impl Router {
    pub fn new() -> Self {
        Self { routes: Vec::new() }
    }

    pub fn add_route(&mut self, route: Route) {
        self.routes.push(route);
    }

    pub fn handle_request(&self, request: &HttpRequest) -> HttpResponse {
        for route in &self.routes {
            if route.path == request.request_lines.path
                && route.method == request.request_lines.method
            {
                return (route.handler)(request);
            }
        }

        let mut path_found = false; // Flag to check if the path was found but the method was not allowed

        let mut allowed_methods: Vec<HttpMethod> = Vec::new(); // Vector to store allowed methods for the found path

        for route in &self.routes {
            if route.path == request.request_lines.path {
                path_found = true;
                allowed_methods.push(route.method.clone()); // Store the allowed method for the found path
                if route.method == request.request_lines.method {
                    return (route.handler)(request);
                }
            }
        }

        if path_found {
            return HttpResponse {
                status_code: 405,
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
            status_code: 404,
            headers: HashMap::new(),
            body: "Not Found".to_string(),
        }
    }
    fn route(path: &str, handler: Handler, method: HttpMethod) -> Route {
        Route {
            path: path.to_string(),
            method: method,
            handler,
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
