pub mod nmap;
pub mod trie;
pub mod error;
pub use trie::TrieNode;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4); 
    }
}
