//! [![License: BSD-3-Clause](https://img.shields.io/badge/license-BSD--3--Clause-blue)](./LICENSE)
//! [![cdumay_error_http on crates.io](https://img.shields.io/crates/v/cdumay_error_http)](https://crates.io/crates/cdumay_error_http)
//! [![cdumay_error_http on docs.rs](https://docs.rs/cdumay_error_http/badge.svg)](https://docs.rs/cdumay_error_http)
//! [![Source Code Repository](https://img.shields.io/badge/Code-On%20GitHub-blue?logo=GitHub)](https://github.com/cdumay/cdumay_error_http)
//!
//! # HTTP Error Converter
//!
//! This crate provides structured mapping from HTTP status codes to custom application error types using the [`cdumay_error`](https://docs.rs/cdumay_error/) crate. It is especially useful when you want to handle HTTP error responses in a standardized and extensible way.
//!
//! ## Features
//!
//! - Maps common HTTP status codes (300–511) to well-defined application-specific errors.
//! - Integrates seamlessly with the `cdumay_error` ecosystem.
//! - Allows contextual error data and custom messages.
//! - Supports conversion from both `u16` and `http::StatusCode`.
//!
//! ## Usage
//!
//! ### Define Error Kinds and Errors
//!
//! The `define_kinds!` macro associates each HTTP status code with:
//! - A custom error code string (e.g., `"HTTP-26760"`)
//! - A numerical HTTP status code
//! - A descriptive error label
//!
//! The `define_errors!` macro maps those kinds into named error types (e.g., `HttpClientError404`, `HttpServerError500`, etc.).
//!
//! ### HTTP Error Conversion
//!
//! Use the `HTTPErrorConverter` to map a numeric status code (or `StatusCode`) to a fully populated `cdumay_error::Error`.
//!
//! ```rust
//! use std::collections::BTreeMap;
//! use serde_value::Value;
//! use http::StatusCode;
//! use cdumay_error_http::HTTPErrorConverter;
//!
//! let mut context = BTreeMap::new();
//! context.insert("url".to_string(), Value::String("https://example.com".to_string()));
//!
//! let error = HTTPErrorConverter::from_status(
//!     StatusCode::NOT_FOUND,
//!     Some("The requested resource could not be located.".to_string()),
//!     context,
//! );
//!
//! println!("{:?}", error);
//! ```
/// Provides utilities for mapping HTTP status codes to structured `cdumay_error::Error` types.
use cdumay_error::{AsError, Error, ErrorConverter, define_errors, define_kinds};
use http::StatusCode;
use std::collections::BTreeMap;

/// Defines error kinds for HTTP status codes.
///
/// Each entry includes:
/// - A unique internal code (e.g. `"HTTP-11298"`)
/// - An HTTP status code (e.g. `300`)
/// - A description (e.g. `"Multiple Choices"`)
///
/// These are grouped based on the standard HTTP categories:
/// - 3xx: Redirection
/// - 4xx: Client Errors
/// - 5xx: Server Errors
define_kinds! {
    // Redirection (3xx)
    MultipleChoices = ("HTTP-11298", 300, "Multiple Choices"),
    MovedPermanently = ("HTTP-23108", 301, "Moved Permanently"),
    Found = ("HTTP-07132", 302, "HttpRedirection302"),
    SeeOther = ("HTTP-16746", 303, "See Other"),
    NotModified = ("HTTP-21556", 304, "Not Modified"),
    UseProxy = ("HTTP-31839", 305, "Use Proxy"),
    TemporaryRedirect = ("HTTP-25446", 307, "Temporary Redirect"),
    PermanentRedirect = ("HTTP-12280", 308, "Permanent Redirect"),

    // Client Errors (4xx)
    BadRequest = ("HTTP-26760", 400, "Bad Request"),
    Unauthorized = ("HTTP-08059", 401, "HttpClientError401"),
    PaymentRequired = ("HTTP-18076", 402, "Payment Required"),
    Forbidden = ("HTTP-23134", 403, "HttpClientError403"),
    NotFound = ("HTTP-18430", 404, "Not HttpRedirection302"),
    MethodNotAllowed = ("HTTP-23585", 405, "Method Not Allowed"),
    NotAcceptable = ("HTTP-04289", 406, "Not Acceptable"),
    ProxyAuthenticationRequired = ("HTTP-17336", 407, "Proxy Authentication Required"),
    RequestTimeout = ("HTTP-00565", 408, "Request Timeout"),
    Conflict = ("HTTP-08442", 409, "HttpClientError409"),
    Gone = ("HTTP-19916", 410, "HttpClientError410"),
    LengthRequired = ("HTTP-09400", 411, "Length Required"),
    PreconditionFailed = ("HTTP-22509", 412, "Precondition Failed"),
    PayloadTooLarge = ("HTTP-10591", 413, "Payload Too Large"),
    UriTooLong = ("HTTP-01377", 414, "URI Too Long"),
    UnsupportedMediaType = ("HTTP-12512", 415, "Unsupported Media Type"),
    RangeNotSatisfiable = ("HTTP-21696", 416, "Range Not Satisfiable"),
    ExpectationFailed = ("HTTP-16872", 417, "Expectation Failed"),
    ImATeapot = ("HTTP-23719", 418, "I'm a teapot"),
    MisdirectedRequest = ("HTTP-26981", 421, "Misdirected Request"),
    UnprocessableEntity = ("HTTP-12568", 422, "Unprocessable Entity"),
    Locked = ("HTTP-32695", 423, "HttpClientError423"),
    FailedDependency = ("HTTP-19693", 424, "Failed Dependency"),
    UpgradeRequired = ("HTTP-22991", 426, "Upgrade Required"),
    PreconditionRequired = ("HTTP-02452", 428, "Precondition Required"),
    TooManyRequests = ("HTTP-12176", 429, "Too Many Requests"),
    RequestHeaderFieldsTooLarge = ("HTTP-07756", 431, "Request Header Fields Too Large"),
    UnavailableForLegalReasons = ("HTTP-12136", 451, "Unavailable For Legal Reasons"),

    // Server Errors (5xx)
    InternalServerError = ("HTTP-09069", 500, "Internal Server Error"),
    NotImplemented = ("HTTP-03394", 501, "Not Implemented"),
    BadGateway = ("HTTP-19734", 502, "Bad Gateway"),
    ServiceUnavailable = ("HTTP-18979", 503, "Service Unavailable"),
    GatewayTimeout = ("HTTP-17595", 504, "Gateway Timeout"),
    HttpVersionNotSupported = ("HTTP-01625", 505, "HTTP Version Not Supported"),
    VariantAlsoNegotiates = ("HTTP-28382", 506, "Variant Also Negotiates"),
    InsufficientStorage = ("HTTP-32132", 507, "Insufficient Storage"),
    LoopDetected = ("HTTP-30770", 508, "Loop Detected"),
    NotExtended = ("HTTP-19347", 510, "Not Extended"),
    NetworkAuthenticationRequired = ("HTTP-31948", 511, "Network Authentication Required"),
}

/// Maps error kinds to usable error types.
///
/// These types can be constructed and used in code and tests.
define_errors! {
    HttpRedirection300 = MultipleChoices,
    HttpRedirection301 = MovedPermanently,
    HttpRedirection302 = Found,
    HttpRedirection303 = SeeOther,
    HttpRedirection304 = NotModified,
    HttpRedirection305 = UseProxy,
    HttpRedirection307 = TemporaryRedirect,
    HttpRedirection308 = PermanentRedirect,
    HttpClientError400 = BadRequest,
    HttpClientError401 = Unauthorized,
    HttpClientError402 = PaymentRequired,
    HttpClientError403 = Forbidden,
    HttpClientError404 = NotFound,
    HttpClientError405 = MethodNotAllowed,
    HttpClientError406 = NotAcceptable,
    HttpClientError407 = ProxyAuthenticationRequired,
    HttpClientError408 = RequestTimeout,
    HttpClientError409 = Conflict,
    HttpClientError410 = Gone,
    HttpClientError411 = LengthRequired,
    HttpClientError412 = PreconditionFailed,
    HttpClientError413 = PayloadTooLarge,
    HttpClientError414 = UriTooLong,
    HttpClientError415 = UnsupportedMediaType,
    HttpClientError416 = RangeNotSatisfiable,
    HttpClientError417 = ExpectationFailed,
    HttpClientError418 = ImATeapot,
    HttpClientError421 = MisdirectedRequest,
    HttpClientError422 = UnprocessableEntity,
    HttpClientError423 = Locked,
    HttpClientError424 = FailedDependency,
    HttpClientError425 = UpgradeRequired,
    HttpClientError428 = PreconditionRequired,
    HttpClientError429 = TooManyRequests,
    HttpClientError431 = RequestHeaderFieldsTooLarge,
    HttpClientError451 = UnavailableForLegalReasons,
    HttpServerError500 = InternalServerError,
    HttpServerError501 = NotImplemented,
    HttpServerError502 = BadGateway,
    HttpServerError503 = ServiceUnavailable,
    HttpServerError504 = GatewayTimeout,
    HttpServerError505 = HttpVersionNotSupported,
    HttpServerError506 = VariantAlsoNegotiates,
    HttpServerError507 = InsufficientStorage,
    HttpServerError508 = LoopDetected,
    HttpServerError510 = NotExtended,
    HttpServerError511 = NetworkAuthenticationRequired,
}

/// Converts HTTP status codes into `cdumay_error::Error` objects.
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
    /// * `text` - Optional custom error message to override the default
    /// * `context` - A BTreeMap containing structured metadata (e.g., request ID, URI, etc.)
    ///
    /// # Returns
    ///
    /// A `cdumay_error::Error` that includes the appropriate kind, optional message, and context.
    ///
    /// Unknown status codes will fall back to `HttpServerError500`.
    pub fn from_u16(status: u16, text: Option<String>, context: BTreeMap<String, serde_value::Value>) -> Error {
        let mut error = match status {
            300 => Error::from(HttpRedirection300::new().set_details(context)),
            301 => Error::from(HttpRedirection301::new().set_details(context)),
            302 => Error::from(HttpRedirection302::new().set_details(context)),
            303 => Error::from(HttpRedirection303::new().set_details(context)),
            304 => Error::from(HttpRedirection304::new().set_details(context)),
            305 => Error::from(HttpRedirection305::new().set_details(context)),
            307 => Error::from(HttpRedirection307::new().set_details(context)),
            308 => Error::from(HttpRedirection308::new().set_details(context)),
            400 => Error::from(HttpClientError400::new().set_details(context)),
            401 => Error::from(HttpClientError401::new().set_details(context)),
            402 => Error::from(HttpClientError402::new().set_details(context)),
            403 => Error::from(HttpClientError403::new().set_details(context)),
            404 => Error::from(HttpClientError404::new().set_details(context)),
            405 => Error::from(HttpClientError405::new().set_details(context)),
            406 => Error::from(HttpClientError406::new().set_details(context)),
            407 => Error::from(HttpClientError407::new().set_details(context)),
            408 => Error::from(HttpClientError408::new().set_details(context)),
            409 => Error::from(HttpClientError409::new().set_details(context)),
            410 => Error::from(HttpClientError410::new().set_details(context)),
            411 => Error::from(HttpClientError411::new().set_details(context)),
            412 => Error::from(HttpClientError412::new().set_details(context)),
            413 => Error::from(HttpClientError413::new().set_details(context)),
            414 => Error::from(HttpClientError414::new().set_details(context)),
            415 => Error::from(HttpClientError415::new().set_details(context)),
            416 => Error::from(HttpClientError416::new().set_details(context)),
            417 => Error::from(HttpClientError417::new().set_details(context)),
            418 => Error::from(HttpClientError418::new().set_details(context)),
            421 => Error::from(HttpClientError421::new().set_details(context)),
            422 => Error::from(HttpClientError422::new().set_details(context)),
            423 => Error::from(HttpClientError423::new().set_details(context)),
            424 => Error::from(HttpClientError424::new().set_details(context)),
            426 => Error::from(HttpClientError425::new().set_details(context)),
            428 => Error::from(HttpClientError428::new().set_details(context)),
            429 => Error::from(HttpClientError429::new().set_details(context)),
            431 => Error::from(HttpClientError431::new().set_details(context)),
            451 => Error::from(HttpClientError451::new().set_details(context)),
            501 => Error::from(HttpServerError501::new().set_details(context)),
            502 => Error::from(HttpServerError502::new().set_details(context)),
            503 => Error::from(HttpServerError503::new().set_details(context)),
            504 => Error::from(HttpServerError504::new().set_details(context)),
            505 => Error::from(HttpServerError505::new().set_details(context)),
            506 => Error::from(HttpServerError506::new().set_details(context)),
            507 => Error::from(HttpServerError507::new().set_details(context)),
            508 => Error::from(HttpServerError508::new().set_details(context)),
            510 => Error::from(HttpServerError510::new().set_details(context)),
            511 => Error::from(HttpServerError511::new().set_details(context)),
            _ => Error::from(HttpServerError500::new().set_details(context)),
        };
        if let Some(txt) = text {
            error.message = txt;
        }
        error
    }

    /// Converts a `http::StatusCode` into a structured `Error`.
    ///
    /// This is a convenience wrapper around [`Self::from_u16`] for working with the
    /// `http` crate’s typed status codes.
    pub fn from_status(status: StatusCode, text: Option<String>, context: BTreeMap<String, serde_value::Value>) -> Error {
        Self::from_u16(status.as_u16(), text, context)
    }
}
