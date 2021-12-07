#![allow(unused_variables)]
use crate::error::TrieError;
use crate::nmap::Nmap;
use std::borrow::BorrowMut;
use std::fmt::Debug;

pub trait Container<T>: Debug
where
    T: Default + Copy,
{
    fn new() -> Self;
    fn set(&mut self, k: u8, v: TrieNode<T>);
    fn get(&self, k: u8) -> Result<*const TrieNode<T>, TrieError>;
}

#[derive(Debug)]
pub struct TrieNode<T>
where
    T: Copy + Default + Clone,
{
    key: Vec<u8>,
    node_key: u8,
    pub val: T,
    prev: *const TrieNode<T>,
    next: *const TrieNode<T>,
    children: *const crate::nmap::Nmap<T>,
}

impl<T> TrieNode<T>
where
    T: Default + Copy + std::fmt::Debug,
{
    pub fn default(k: u8) -> TrieNode<T> {
        TrieNode {
            node_key: k,
            val: T::default(),
            key: Vec::new(),
            prev: std::ptr::null(),
            next: std::ptr::null(),
            children: &Nmap::new() as *const Nmap<T>,
        }
    }
    pub fn new(k: u8, key: Vec<u8>, val: T) -> TrieNode<T> {
        TrieNode {
            node_key: k,
            val: val,
            key: key,
            prev: std::ptr::null(),
            next: std::ptr::null(),
            children: &Nmap::new() as *const Nmap<T>,
        }
    }
}

pub struct Trie<T>
where
    T: Copy + Default,
{
    key_size: usize,
    root: TrieNode<T>,
    head: *const TrieNode<T>,
    tail: *const TrieNode<T>,
    size: usize,
}

impl<T> Trie<T>
where
    T: Copy + Clone + Default + std::fmt::Debug,
{
    pub fn new(key_size: usize) -> Trie<T> {
        Trie {
            key_size,
            root: TrieNode::default(0),
            head: std::ptr::null(),
            tail: std::ptr::null(),
            size: 0,
        }
    }

    pub fn set(&mut self, key: Vec<u8>, val: T) -> Result<(), TrieError> {
        if key.len() != self.key_size {
            return Err(TrieError::new(1, "key size not match"));
        }

        let mut cur = &mut self.root as *const TrieNode<T>;
        for k in key {
            unsafe {
                if let Err(err) = (*cur).children.as_ref().unwrap().get(k) {
                    let node = TrieNode::new(k, Vec::new(), T::default());
                    let children =&mut (*(*cur).children);
                    children.set(k, node);
                }
                cur = (*cur).children.as_ref().unwrap().get(k)?;
            }
        }
        Ok(())
    }

    pub fn get(&self, key: Vec<u8>) -> Result<T, TrieError> {
        if key.len() != self.key_size {
            return Err(TrieError::new(1, "key size not expect"));
        }
        let mut cur = &self.root as *const TrieNode<T>;
        for k in key {
            unsafe {
                cur = (*cur).children.as_ref().unwrap().get(k)?;
                if (*cur).key.len() > 0 {
                    return Ok((*cur).val);
                }
            }
        }
        return Err(TrieError::new(2, "not found"));
    }
}
