#[cfg(test)]
mod tests {
    use http::StatusCode;
    use serde_value::Value;
    use std::collections::BTreeMap;
    use cdumay_error_http::HTTPErrorConverter;

    fn sample_context() -> BTreeMap<String, Value> {
        let mut context = BTreeMap::new();
        context.insert("key".to_string(), Value::String("value".to_string()));
        context
    }

    #[test]
    fn test_known_status_code() {
        let error = HTTPErrorConverter::from_u16(404, None, sample_context());
        assert_eq!(error.kind.message_id(), "HTTP-18430");
        assert_eq!(error.kind.code(), 404);
        assert_eq!(error.message, "Not HttpRedirection302");
        assert!(error.details.unwrap().contains_key("key"));
    }

    #[test]
    fn test_fallback_on_unknown_status_code() {
        let error = HTTPErrorConverter::from_u16(999, None, sample_context());
        assert_eq!(error.kind.message_id(), "HTTP-09069"); // Fallback is 500
        assert_eq!(error.kind.code(), 500);
    }

    #[test]
    fn test_custom_message_is_set() {
        let custom_message = "Something went wrong".to_string();
        let error = HTTPErrorConverter::from_status(
            StatusCode::INTERNAL_SERVER_ERROR,
            Some(custom_message.clone()),
            sample_context(),
        );
        assert_eq!(error.message, custom_message);
    }

    #[test]
    fn test_context_is_attached() {
        let mut context = BTreeMap::new();
        context.insert("foo".to_string(), Value::String("bar".to_string()));
        let error = HTTPErrorConverter::from_u16(403, None, context.clone());
        assert_eq!(error.details.unwrap().get("foo"), Some(&Value::String("bar".to_string())));
    }

    #[test]
    fn test_all_supported_statuses_have_a_mapping() {
        // Check that none of these panic
        for code in 300..=511 {
            let _ = HTTPErrorConverter::from_u16(code, None, BTreeMap::new());
        }
    }
}
