<!-- markdownlint-disable -->
<h1 align="center">
    HTTP Mambo
    <br>
</h1>

<p align="center">
    <strong>An HTTP server library written in Rust using Tokio for building fast, simple APIs and web services. It implements networking, HTTP parsing, routing, and request handling from the ground up.</strong>
</p>

<p align="center">
    <a href="#features">Features</a>
    ·
    <a href="#getting-started">Getting Started</a>
    ·
    <a href="#example">Example</a>
    ·
    <a href="#api-overview">API Overview</a>
</p>

---

## Overview

HTTP Mambo is a minimal HTTP library for building simple web services in Rust. It includes a Tokio-based TCP listener, request parsing, response building, and a small router with support for multiple HTTP methods and Path parameters with optional type constraints, such as `/users/:id<u16>`.

## Features

- Async request handling with Tokio
- Simple route registration for `GET`, `POST`, `PUT`, `DELETE`, `PATCH`, `OPTIONS`, and `HEAD`
- Path matching with dynamic segments such as `/users/:id` and type constraints like `/users/:id<u16>`
- Response helpers for status, headers, and body content
- Response bodies using raw bytes for text and binary content

## Getting Started

Add HttpMambo to your Rust project as a Git dependency:

```toml
[dependencies]
http_mambo = {git = "https://github.com/PromiseJoseph/http-mambo"}
```

You can run the included example server directly from the repository:

```bash
cargo run --example basic_server <custom_address>
```

By default, the example listens on `127.0.0.1:8000` if no custom address is provided.

## Example

```rust
use http_mambo::types::{HttpRequest, HttpResponse, Router};

async fn home(request:HttpRequest) -> HttpResponse {
    HttpResponse::new()
        .with_body(format!("Hello from {}", request.request_lines.path))
}

let mut router = Router::new();
router.get("/", home);
router.get("/test/:id<u16>", home);
router.get("/testany/:id", home);
```

See [examples/basic_server/main.rs](examples/basic_server/main.rs) for a complete runnable server.

## API Overview

- [listener.rs](src/listener.rs) binds the TCP listener
- [stream.rs](src/stream.rs) reads requests and writes responses
- [types.rs](src/types.rs) defines the core types for requests, responses, methods,routes, and status codes.
- [http/request.rs](src/http/request.rs) parses incoming HTTP requests
- [http/response.rs](src/http/response.rs) constructs HTTP responses
- [http/router.rs](src/http/router.rs) registers routes and dispatches handlers
- [http/route.rs](src/http/route.rs) matches static and parameterized paths
- [http/status.rs](src/http/status.rs) defines status codes

## Project Structure

```text
src/
    http/
        request.rs
        response.rs
        router.rs
        route.rs
        status.rs
    listener.rs
    stream.rs
    types.rs
examples/
    basic_server/
```

## License

MIT License. See [LICENSE](LICENSE) for details.
