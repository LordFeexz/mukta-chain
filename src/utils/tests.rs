#[cfg(test)]
mod tests {
    use crate::utils::generate_id::generate_uuid;

    #[test]
    fn test_new_generated_id() {
        let id = generate_uuid();
        assert!(!id.is_empty());
        assert!(uuid::Uuid::parse_str(&id).is_ok());
    }
}
