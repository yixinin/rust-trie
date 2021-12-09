pub mod error;
pub mod nmap;
pub mod trie;

#[cfg(test)]
mod tests {
    use std::borrow::Borrow;
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::error;
    use crate::trie::{Trie, TrieNode};
    use crate::{nmap, trie::Container};

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_nmap() {
        let mut m = nmap::Nmap::<i32>::new();
        m.set(
            1,
            Some(Rc::new(RefCell::new(TrieNode::new(1, Vec::new(), 10)))),
        );
        m.set(
            2,
            Some(Rc::new(RefCell::new(TrieNode::new(2, Vec::new(), 20)))),
        );
        assert_eq!(m.get(2).is_none(), false);
        if let Some(node) = m.get(2) {
            assert_eq!(node.as_ref().borrow().val, 20)
        }
        println!("{:?}", m.get(2))
    }
    #[test]
    fn test_trie() {
        let mut trie = Trie::<i32>::new(3);
        trie.set(vec![1, 2, 3], 10);
        trie.set(vec![2, 2, 3], 20);
        let val = trie.get(vec![1, 2, 3]);
        match val {
            Ok(v) => {
                assert_eq!(v, 10);
            }
            Err(err) => {
                assert_eq!(1, 2)
            }
        }
    }
}
