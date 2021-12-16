pub mod error;
pub mod nmap;
pub mod trie;
pub mod byte_map;

#[cfg(test)]
mod tests {
    use std::vec;

    use bson::oid::ObjectId;

    use crate::trie::Trie;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    #[test]
    fn test_trie() {
        let size = 10000000;
        let mut trie = Trie::new(12);
        let mut keys = Vec::with_capacity(size);
        for i in 0..size {
            let key = ObjectId::new().bytes().to_vec();
            if let Err(err) = trie.set(key.clone(), key.clone()) {
                println!("{}", err);
                return;
            }
            keys.push(key)
        }

        for i in 0..size {
            let key = keys[i].clone();
            if let Ok(v1) = trie.get(key.clone()) {
                assert_eq!(v1, key.clone());
            } else {
                assert_eq!(1, 2);
            }
        }
    }
}
