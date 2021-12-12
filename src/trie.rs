#![allow(unused_variables)]
use crate::error::TrieError;
use crate::nmap::Nmap;
use std::borrow::BorrowMut;
use std::fmt::Debug;
use std::ptr::NonNull;

pub trait Container<T>: Debug
where
    T: Default + Copy,
{
    fn new() -> Self;
    fn set(&mut self, k: u8, v: Option<NonNull<TrieNode<T>>>);
    fn get(&self, k: u8) -> Option<NonNull<TrieNode<T>>>;
}

pub struct TrieNode<T> {
    key: Option<Vec<u8>>,
    node_key: u8,
    val: Option<T>,
    prev: Option<NonNull<TrieNode<T>>>,
    next: Option<NonNull<TrieNode<T>>>,
    children: Option<NonNull<Nmap<T>>>,
}

impl<T> TrieNode<T>
where
    T: Default + Copy + std::fmt::Debug,
{
    pub fn root() -> TrieNode<T> {
        TrieNode {
            node_key: 0,
            key: None,
            val: None,
            prev: None,
            next: None,
            children: Some(Box::leak(Box::new(Nmap::<T>::new())).into()),
        }
    }
    pub fn new(k: u8) -> TrieNode<T> {
        TrieNode {
            node_key: k,
            key: None,
            val: None,
            prev: None,
            next: None,
            children: Some(Box::leak(Box::new(Nmap::<T>::new())).into()),
        }
    }
    pub fn leaf(k: u8, key: Vec<u8>, val: T) -> TrieNode<T> {
        TrieNode {
            node_key: k,
            key: Some(key),
            val: Some(val),
            prev: None,
            next: None,
            children: None,
        }
    }
}

pub struct Trie<T>
where
    T: Copy + Default,
{
    key_size: usize,
    root: Option<NonNull<TrieNode<T>>>,
    head: Option<NonNull<TrieNode<T>>>,
    tail: Option<NonNull<TrieNode<T>>>,
    size: usize,
}

impl<T> Trie<T>
where
    T: Copy + Clone + Default + std::fmt::Debug,
{
    pub fn new(key_size: usize) -> Trie<T> {
        Trie {
            key_size,
            root: Some(Box::leak(Box::new(TrieNode::root())).into()),
            head: None,
            tail: None,
            size: 0,
        }
    }

    pub fn set(&mut self, key: Vec<u8>, val: T) -> Result<(), TrieError> {
        if key.clone().len() != self.key_size {
            return Err(TrieError::new(1, "key size not match"));
        }

        let endk = key.clone()[self.key_size - 1];

        unsafe {
            let mut cur = self.root;
            if let Some(mut cur_node) = self.root {
                for k in key.clone() {
                    if let Some(mut children) = (*cur_node.as_ptr()).children {
                        if let None = (*children.as_ptr()).get(k) {
                            let mut node;
                            if k == endk {
                                node = Some(
                                    Box::leak(Box::new(TrieNode::leaf(k, key.clone(), val))).into(),
                                )
                            } else {
                                node = Some(Box::leak(Box::new(TrieNode::<T>::new(k))).into());
                            }
                            (*children.as_ptr()).set(k, node);
                            cur = node;
                            continue;
                        }
                        cur = (*children.as_ptr()).get(k);
                    }
                }
            }
        }
        Ok(())
    }

    pub fn get(&self, key: Vec<u8>) -> Result<T, TrieError> {
        if key.len() != self.key_size {
            return Err(TrieError::new(1, "key size not expect"));
        }
        unsafe {
            let mut cur = self.root;
            if let Some(cur_node) = self.root {
                for k in key {
                    if let Some(children) = (*cur_node.as_ptr()).children {
                        let node = (*children.as_ptr()).get(k);
                        if let Some(node_v) = node {
                            if let Some(val) = (*node_v.as_ptr()).val {
                                return Ok(val);
                            }
                        }
                        cur = node;
                        continue;
                    }
                    break;
                }
            }
        }
        return Err(TrieError::new(2, "not found"));
    }
}
