#![allow(unused_variables)]
use crate::error::TrieError;
use crate::nmap::Nmap;
use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

pub trait Container<T>: Debug
where
    T: Default + Copy + PartialEq,
{
    fn new() -> Self;
    fn set(&mut self, k: u8, v: Option<Rc<RefCell<TrieNode<T>>>>);
    fn get(&self, k: u8) -> Option<Rc<RefCell<TrieNode<T>>>>;
}

#[derive(Debug)]
pub struct TrieNode<T>
where
    T: Copy + Default + Clone,
{
    pub key: Vec<u8>,
    pub node_key: u8,
    pub val: T,
    pub prev: Option<Rc<RefCell<TrieNode<T>>>>,
    pub next: Option<Rc<RefCell<TrieNode<T>>>>,
    pub children: Nmap<T>,
}

impl<T> TrieNode<T>
where
    T: Default + Copy + std::fmt::Debug + PartialEq,
{
    pub fn default(k: u8) -> TrieNode<T> {
        TrieNode {
            node_key: k,
            val: T::default(),
            key: Vec::new(),
            prev: None,
            next: None,
            children: Nmap::new(),
        }
    }
    pub fn new(k: u8, key: Vec<u8>, val: T) -> TrieNode<T> {
        TrieNode {
            node_key: k,
            val: val,
            key: key,
            prev: None,
            next: None,
            children: Nmap::new(),
        }
    }
}

pub struct Trie<T>
where
    T: Copy + Default,
{
    key_size: usize,
    root: Option<Rc<RefCell<TrieNode<T>>>>,
    head: Option<Rc<RefCell<TrieNode<T>>>>,
    tail: Option<Rc<RefCell<TrieNode<T>>>>,
    size: usize,
}

impl<T> Trie<T>
where
    T: Copy + Clone + Default + std::fmt::Debug + PartialEq,
{
    pub fn new(key_size: usize) -> Trie<T> {
        Trie {
            key_size,
            root: Some(Rc::new(RefCell::new(TrieNode::default(0)))),
            head: None,
            tail: None,
            size: 0,
        }
    }

    pub fn set(&mut self, key: Vec<u8>, val: T) -> Result<(), TrieError> {
        if key.len() != self.key_size {
            return Err(TrieError::new(1, "key size not match"));
        }

        if let Some(mut x) = self.root.clone() {
            for k in key {
                let x_clone = x.clone();
                let mut cur = x_clone.as_ref().borrow_mut();
                match cur.children.get(k) {
                    None => {
                        let node = Some(Rc::new(RefCell::new(TrieNode::new(
                            k,
                            Vec::new(),
                            T::default(),
                        ))));
                        cur.children.set(k, node);
                    }
                    Some(node) => x = node,
                };
            }
        }

        Ok(())
    }

    pub fn get(&self, key: Vec<u8>) -> Result<T, TrieError> {
        if key.len() != self.key_size {
            return Err(TrieError::new(1, "key size not expect"));
        }

        if let Some(mut x) = self.root.clone() {
            for k in key {
                let x_clone = x.clone();
                let cur = x_clone.as_ref().borrow();
                match cur.children.get(k) {
                    None => {
                        return Err(TrieError::new(404, "not found"));
                    }
                    Some(node) => {
                        let v = node.as_ref().borrow().val;
                        if v != T::default() {
                            return Ok(v);
                        }
                        x = node;
                    }
                };
            }
        }
        return Err(TrieError::new(2, "not found"));
    }
}
