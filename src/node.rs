#![allow(unused_variables)]
use crate::error::TrieError;

pub trait Container<T, C> {
    fn new() -> Self;
    fn set(&self, k: u8, v: TrieNode<T, C>);
    fn get(&self, k: u8) -> Result<*const TrieNode<T, C>, TrieError>;
}

pub struct TrieNode<T, C> {
    key: Vec<u8>,
    node_key: u8,
    val: T,
    prev: *const TrieNode<T, C>,
    next: *const TrieNode<T, C>,
    children: C,
}

impl<T, C> TrieNode<T, C>
where
    C: Container<T, C>,
{
    pub fn new(k: u8, key: Vec<u8>, val: T) -> TrieNode<T, C> {
        TrieNode {
            node_key: k,
            val: val,
            key: key,
            prev: std::ptr::null(),
            next: std::ptr::null(),
            children: C::new(),
        }
    }
}

pub struct Trie<T, C> {
    key_size: usize,
    root: *const TrieNode<T, C>,
    head: *const TrieNode<T, C>,
    tail: *const TrieNode<T, C>,
    size: usize,
}

impl<T, C> Trie<T, C>
where
    T: Copy + Clone,
    C: Container<T, C>,
{
    pub fn new(key_size: usize) -> Trie<T, C> {
        Trie {
            key_size,
            root: std::ptr::null(),
            head: std::ptr::null(),
            tail: std::ptr::null(),
            size: 0,
        }
    }

    pub fn get(&self, key: Vec<u8>) -> Result<T, TrieError> {
        if key.len() != self.key_size {
            return Err(TrieError::new(1, "key size not expect"));
        }
        let mut cur = self.root;
        for k in key {
            if cur.is_null() {
                return Err(TrieError::new(2, "not found"));
            }
            unsafe {
                cur = (*cur).children.get(k)?;
                if cur.is_null() {
                    return Err(TrieError::new(2, "not found"));
                }
                if (*cur).key.len() > 0 {
                    return Ok((*cur).val);
                }
            }
        }
        return Err(TrieError::new(2, "not found"));
    }
}
