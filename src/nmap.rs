use crate::{trie::Container, trie::TrieNode};
use std::fmt::Debug;
use std::ptr::NonNull;

#[derive(Debug)]
pub struct Nmap<T> {
    buckets: [Option<NonNull<TrieNode<T>>>; 10],
}

impl<T> std::fmt::Display for Nmap<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut msg = String::from("[");
        for i in 0..10 {
            if let Some(node) = self.buckets[i] {
                msg = format!("{},{},", msg, node.as_ptr() as u8);
            } else {
                msg = format!("{},{},", msg, "None");
            }
        }
        write!(f, "{}]", msg)?;
        Ok(())
    }
}

impl<T> Container<T> for Nmap<T> {
    fn new() -> Nmap<T> {
        Nmap {
            buckets: [None, None, None, None, None, None, None, None, None, None],
        }
    }
    fn get(&self, k: u8) -> Option<NonNull<TrieNode<T>>> {
        self.buckets[k as usize]
    }

    fn set(&mut self, k: u8, v: Option<NonNull<TrieNode<T>>>) {
        self.buckets[k as usize] = v;
        ()
    }
    fn del(&mut self, k: u8) -> bool {
        if let Some(node) = self.buckets[k as usize] {
            self.buckets[k as usize] = None;
            return true;
        }
        false
    }
    fn prev(&self, k: u8) -> Option<NonNull<TrieNode<T>>> {
        let k = (k - 1) as usize;
        let mut i = k;
        while i < k {
            if !self.buckets[i].is_none() {
                return self.buckets[i];
            }
            i -= 1;
        }
        None
    }

    fn next(&self, k: u8) -> Option<NonNull<TrieNode<T>>> {
        let k = (k + 1) as usize;
        for i in k + 1..10 {
            if !self.buckets[i].is_none() {
                return self.buckets[i];
            }
        }
        None
    }
    fn is_head(&self, k: u8) -> bool {
        let k = k as usize;
        for i in 0..10 {
            if !self.buckets[i].is_none() {
                return i == k;
            }
        }
        false
    }
    fn head(&self) -> Option<NonNull<TrieNode<T>>> {
        self.next(255)
    }

    fn is_tail(&self, k: u8) -> bool {
        let k = k as usize;
        let mut i = 9;
        while i < k {
            if !self.buckets[i].is_none() {
                return i == k;
            }
            i -= 1;
        }
        false
    }

    fn tail(&self) -> Option<NonNull<TrieNode<T>>> {
        self.prev(10)
    }
    fn keys(&self) -> Vec<u8> {
        let mut keys = Vec::with_capacity(10);
        for i in 0..10 {
            if let Some(node) = self.buckets[i] {
                keys.push(i as u8)
            }
        }
        return keys;
    }

    fn pad() -> u8 {
        return '0' as u8;
    }
}
