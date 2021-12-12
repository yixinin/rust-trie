pub mod error;
pub mod nmap;
pub mod trie;

use trie::Trie;

#[cfg(test)]
mod tests {
    use crate::trie::Trie;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    #[test]
    fn test_trie() {
        let mut trie = Trie::new(3);
        trie.set(vec![1, 2, 3], 123);
        trie.set(vec![1, 2, 4], 124);

        if let Ok(v1) = trie.get(vec![1, 2, 3]) {
            assert_eq!(v1, 123);
        } else {
            assert_eq!(1, 2);
        }

        if let Ok(v1) = trie.get(vec![1,2,4]) {
            assert_eq!(v1, 124);
        } else {
            assert_eq!(1, 2);
        }

        if let Ok(v1) = trie.get(vec![1,2,4]) {
            assert_eq!(v1, 124);
        } else {
            assert_eq!(1, 1);
        }
    }
}
