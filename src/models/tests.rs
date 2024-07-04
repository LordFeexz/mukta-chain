#[cfg(test)]
mod tests {
    use crate::models::block::Block;

    #[test]
    fn test_new_block() {
        let previous_hash = String::from("previous_hash");
        let data = String::from("data");

        let block = Block::new(previous_hash.clone(), data.clone());
        assert_eq!(block.previous_hash, previous_hash);
        assert_eq!(block.data, data);
        assert!(!block.id.is_empty());
        assert!(block.timestamp > 0);
    }
}
