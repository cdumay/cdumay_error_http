//! [![License: BSD-3-Clause](https://img.shields.io/badge/license-BSD--3--Clause-blue)](./LICENSE)
//! [![cdumay_error_http on crates.io](https://img.shields.io/crates/v/cdumay_error_http)](https://crates.io/crates/cdumay_error_http)
//! [![cdumay_error_http on docs.rs](https://docs.rs/cdumay_error_http/badge.svg)](https://docs.rs/cdumay_error_http)
//! [![Source Code Repository](https://img.shields.io/badge/Code-On%20GitHub-blue?logo=GitHub)](https://github.com/cdumay/cdumay_error_http)
//!
//! This crate provides structured mapping from HTTP status codes to custom application error types using the [`cdumay_core`](https://docs.rs/cdumay_core/) crate. It is especially useful when you want to handle HTTP error responses in a standardized and extensible way.
//!
//! ## Features
//!
//! - Maps common HTTP status codes (300–511) to well-defined application-specific errors.
//! - Integrates seamlessly with the `cdumay_core` ecosystem.
//! - Allows contextual error data and custom messages.
//! - Supports conversion from `u16`
//!
//! ## Usage
//!
//! ### Define Error Kinds and Errors
//!
//! The `define_kinds!` macro associates each HTTP status code with:
//! - A numerical HTTP status code
//! - A descriptive error label
//!
//! The `define_errors!` macro maps those kinds into named error types (e.g., `NotFound`, `TooManyRequests`, etc.).
//!
//! ```rust
//! use std::collections::BTreeMap;
//! use serde_value::Value;
//! use cdumay_error_http::HTTPErrorConverter;
//!
//! let mut context = BTreeMap::new();
//! context.insert("url".to_string(), Value::String("https://example.com".to_string()));
//!
//! let error = HTTPErrorConverter::from_u16(404, context);
//! println!("{:?}", error);
//! ```
/// Provides utilities for mapping HTTP status codes to structured `cdumay_core::Error` types.
use cdumay_core::{Error, define_errors, define_kinds};
use std::collections::BTreeMap;

define_kinds! {
    Redirection = (300, "Redirect"),
    ClientError = (400, "Client Error"),
    ServerError = (500, "Server Error"),
}

define_errors! {
    MultipleChoices = (Redirection,  300, "Multiple Choices"),
    MovedPermanently = (Redirection,  301, "Moved Permanently"),
    Found = (Redirection,  302, "Found"),
    SeeOther = (Redirection,  303, "See Other"),
    NotModified = (Redirection,  304, "Not Modified"),
    UseProxy = (Redirection,  305, "Use Proxy"),
    TemporaryRedirect = (Redirection,  307, "Temporary Redirect"),
    PermanentRedirect = (Redirection,  308, "Permanent Redirect"),
    BadRequest = (ClientError,  400, "Bad Request"),
    Unauthorized = (ClientError,  401, "Unauthorized"),
    PaymentRequired = (ClientError,  402, "Payment Required"),
    Forbidden = (ClientError,  403, "Forbidden"),
    NotFound = (ClientError,  404, "Not Found"),
    MethodNotAllowed = (ClientError,  405, "Method Not Allowed"),
    NotAcceptable = (ClientError,  406, "Not Acceptable"),
    ProxyAuthenticationRequired = (ClientError,  407, "Proxy Authentication Required"),
    RequestTimeout = (ClientError,  408, "Request Timeout"),
    Conflict = (ClientError,  409, "Conflict"),
    Gone = (ClientError,  410, "Gone"),
    LengthRequired = (ClientError,  411, "Length Required"),
    PreconditionFailed = (ClientError,  412, "Precondition Failed"),
    PayloadTooLarge = (ClientError,  413, "Payload Too Large"),
    UriTooLong = (ClientError,  414, "URI Too Long"),
    UnsupportedMediaType = (ClientError,  415, "Unsupported Media Type"),
    RangeNotSatisfiable = (ClientError,  416, "Range Not Satisfiable"),
    ExpectationFailed = (ClientError,  417, "Expectation Failed"),
    ImATeapot = (ClientError,  418, "I'm a teapot"),
    MisdirectedRequest = (ClientError,  421, "Misdirected Request"),
    UnprocessableEntity = (ClientError,  422, "Unprocessable Entity"),
    Locked = (ClientError,  423, "Locked"),
    FailedDependency = (ClientError,  424, "Failed Dependency"),
    UpgradeRequired = (ClientError,  426, "Upgrade Required"),
    PreconditionRequired = (ClientError,  428, "Precondition Required"),
    TooManyRequests = (ClientError,  429, "Too Many Requests"),
    RequestHeaderFieldsTooLarge = (ClientError,  431, "Request Header Fields Too Large"),
    UnavailableForLegalReasons = (ClientError,  451, "Unavailable For Legal Reasons"),
    InternalServerError = (ServerError,  500, "Internal Server Error"),
    NotImplemented = (ServerError,  501, "Not Implemented"),
    BadGateway = (ServerError,  502, "Bad Gateway"),
    ServiceUnavailable = (ServerError,  503, "Service Unavailable"),
    GatewayTimeout = (ServerError,  504, "Gateway Timeout"),
    HttpVersionNotSupported = (ServerError,  505, "HTTP Version Not Supported"),
    VariantAlsoNegotiates = (ServerError,  506, "Variant Also Negotiates"),
    InsufficientStorage = (ServerError,  507, "Insufficient Storage"),
    LoopDetected = (ServerError,  508, "Loop Detected"),
    NotExtended = (ServerError,  510, "Not Extended"),
    NetworkAuthenticationRequired = (ServerError,  511, "Network Authentication Required"),
}

/// Converts HTTP status codes into `cdumay_core::Error` objects.
///
/// This struct provides helper methods to convert numeric status codes or `http::StatusCode`
/// into rich error objects with optional message overrides and structured context.
pub struct HTTPErrorConverter;

impl HTTPErrorConverter {
    /// Converts a `u16` HTTP status code into a structured `Error`.
    ///
    /// # Arguments
    ///
    /// * `status` - HTTP status code (e.g., 404)
    /// * `context` - A BTreeMap containing structured metadata (e.g., request ID, URI, etc.)
    ///
    /// # Returns
    ///
    /// A `cdumay_core::Error` that includes the appropriate kind, optional message, and context.
    ///
    /// Unknown status codes will fall back to `HttpServerError500`.
    pub fn from_u16(status: u16, context: BTreeMap<String, serde_value::Value>) -> Error {
        match status {
            300 => MultipleChoices::new().with_details(context).into(),
            301 => MovedPermanently::new().with_details(context).into(),
            302 => Found::new().with_details(context).into(),
            303 => SeeOther::new().with_details(context).into(),
            304 => NotModified::new().with_details(context).into(),
            305 => UseProxy::new().with_details(context).into(),
            307 => TemporaryRedirect::new().with_details(context).into(),
            308 => PermanentRedirect::new().with_details(context).into(),
            400 => BadRequest::new().with_details(context).into(),
            401 => Unauthorized::new().with_details(context).into(),
            402 => PaymentRequired::new().with_details(context).into(),
            403 => Forbidden::new().with_details(context).into(),
            404 => NotFound::new().with_details(context).into(),
            405 => MethodNotAllowed::new().with_details(context).into(),
            406 => NotAcceptable::new().with_details(context).into(),
            407 => ProxyAuthenticationRequired::new().with_details(context).into(),
            408 => RequestTimeout::new().with_details(context).into(),
            409 => Conflict::new().with_details(context).into(),
            410 => Gone::new().with_details(context).into(),
            411 => LengthRequired::new().with_details(context).into(),
            412 => PreconditionFailed::new().with_details(context).into(),
            413 => PayloadTooLarge::new().with_details(context).into(),
            414 => UriTooLong::new().with_details(context).into(),
            415 => UnsupportedMediaType::new().with_details(context).into(),
            416 => RangeNotSatisfiable::new().with_details(context).into(),
            417 => ExpectationFailed::new().with_details(context).into(),
            418 => ImATeapot::new().with_details(context).into(),
            421 => MisdirectedRequest::new().with_details(context).into(),
            422 => UnprocessableEntity::new().with_details(context).into(),
            423 => Locked::new().with_details(context).into(),
            424 => FailedDependency::new().with_details(context).into(),
            426 => UpgradeRequired::new().with_details(context).into(),
            428 => PreconditionRequired::new().with_details(context).into(),
            429 => TooManyRequests::new().with_details(context).into(),
            431 => RequestHeaderFieldsTooLarge::new().with_details(context).into(),
            451 => UnavailableForLegalReasons::new().with_details(context).into(),
            501 => NotImplemented::new().with_details(context).into(),
            502 => BadGateway::new().with_details(context).into(),
            503 => ServiceUnavailable::new().with_details(context).into(),
            504 => GatewayTimeout::new().with_details(context).into(),
            505 => HttpVersionNotSupported::new().with_details(context).into(),
            506 => VariantAlsoNegotiates::new().with_details(context).into(),
            507 => InsufficientStorage::new().with_details(context).into(),
            508 => LoopDetected::new().with_details(context).into(),
            510 => NotExtended::new().with_details(context).into(),
            511 => NetworkAuthenticationRequired::new().with_details(context).into(),
            _ => InternalServerError::new().with_details(context).into(),
        }
    }
}
