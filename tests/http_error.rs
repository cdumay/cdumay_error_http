#[cfg(test)]
mod tests {
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
        let error = HTTPErrorConverter::from_u16(404, sample_context());
        assert_eq!(error.code(), 404);
        assert_eq!(error.message(), "Not Found");
        assert!(error.details().contains_key("key"));
    }

    #[test]
    fn test_fallback_on_unknown_status_code() {
        let error = HTTPErrorConverter::from_u16(999, sample_context());
        assert_eq!(error.code(), 500); // Fallback is 500
    }

    #[test]
    fn test_context_is_attached() {
        let mut context = BTreeMap::new();
        context.insert("foo".to_string(), Value::String("bar".to_string()));
        let error = HTTPErrorConverter::from_u16(403, context.clone());
        assert_eq!(error.details().get("foo"), Some(&Value::String("bar".to_string())));
    }

    #[test]
    fn test_all_supported_statuses_have_a_mapping() {
        // Check that none of these panic
        for code in 300..=511 {
            let _ = HTTPErrorConverter::from_u16(code, BTreeMap::new());
        }
    }
}
