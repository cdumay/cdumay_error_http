# cdumay_error_http

[![License: BSD-3-Clause](https://img.shields.io/badge/license-BSD--3--Clause-blue)](./LICENSE)
[![cdumay_error_http on crates.io](https://img.shields.io/crates/v/cdumay_error_http)](https://crates.io/crates/cdumay_error_http)
[![cdumay_error_http on docs.rs](https://docs.rs/cdumay_error_http/badge.svg)](https://docs.rs/cdumay_error_http)
[![Source Code Repository](https://img.shields.io/badge/Code-On%20GitHub-blue?logo=GitHub)](https://github.com/cdumay/cdumay_error_http)

This crate provides structured mapping from HTTP status codes to custom application error types using the [`cdumay_error`](https://docs.rs/cdumay_error/) crate. It is especially useful when you want to handle HTTP error responses in a standardized and extensible way.

### Features

- Maps common HTTP status codes (300–511) to well-defined application-specific errors.
- Integrates seamlessly with the `cdumay_error` ecosystem.
- Allows contextual error data and custom messages.
- Supports conversion from both `u16` and `http::StatusCode`.

### Usage

#### Define Error Kinds and Errors

The `define_kinds!` macro associates each HTTP status code with:
- A custom error code string (e.g., `"HTTP-26760"`)
- A numerical HTTP status code
- A descriptive error label

The `define_errors!` macro maps those kinds into named error types (e.g., `HttpClientError404`, `HttpServerError500`, etc.).

#### HTTP Error Conversion

Use the `HTTPErrorConverter` to map a numeric status code (or `StatusCode`) to a fully populated `cdumay_error::Error`.

```rust
use std::collections::BTreeMap;
use serde_value::Value;
use http::StatusCode;
use cdumay_error_http::HTTPErrorConverter;

let mut context = BTreeMap::new();
context.insert("url".to_string(), Value::String("https://example.com".to_string()));

let error = HTTPErrorConverter::from_status(
    StatusCode::NOT_FOUND,
    Some("The requested resource could not be located.".to_string()),
    context,
);

println!("{:?}", error);
```
