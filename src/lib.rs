pub mod node;
pub mod trie;
pub mod error;
pub use node::TrieNode;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4); 
    }
}
