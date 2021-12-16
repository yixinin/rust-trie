use crate::{trie::Container, trie::TrieNode};
use std::fmt::Debug;
use std::ptr::NonNull;

#[derive(Debug)]
pub struct ByteMap<T> {
    buckets: [Option<NonNull<TrieNode<T>>>; 256],
}

impl<T> std::fmt::Display for ByteMap<T> {
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

impl<T> Container<T> for ByteMap<T> {
    fn new() -> ByteMap<T> {
        ByteMap {
            buckets: [
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None,
            ],
        }
    }
    fn get(&self, k: u8) -> Option<NonNull<TrieNode<T>>> {
        self.buckets[k as usize]
    }

    fn set(&mut self, k: u8, v: NonNull<TrieNode<T>>) {
        unsafe {
            if let Some(prev) = self.prev(k) {
                (*prev.as_ptr()).next = Some(v);
                (*v.as_ptr()).prev = Some(prev);
            }
            if let Some(next) = self.next(k) {
                (*next.as_ptr()).prev = Some(v);
                (*v.as_ptr()).next = Some(next);
            }
            self.buckets[k as usize] = Some(v);
        }
        ()
    }
    fn del(&mut self, k: u8) -> bool {
        unsafe {
            if let Some(node) = self.buckets[k as usize] {
                let prev = (*node.as_ptr()).prev;
                let next = (*node.as_ptr()).next;
                if let Some(p) = prev {
                    (*p.as_ptr()).next = next
                }
                if let Some(n) = (*node.as_ptr()).next {
                    (*n.as_ptr()).prev = prev
                }
                self.buckets[k as usize] = None;
                return true;
            }
        }
        false
    }
    fn prev(&self, k: u8) -> Option<NonNull<TrieNode<T>>> {
        if k == 0 {
            return None;
        }
        let k = k as usize - 1;
        for i in k..256 {
            if !self.buckets[255 - i].is_none() {
                return self.buckets[255 - i];
            }
        }
        None
    }

    fn next(&self, k: u8) -> Option<NonNull<TrieNode<T>>> {
        if k == 255 {
            return None;
        }
        let k = (k + 1) as usize;
        for i in k..256 {
            if !self.buckets[i].is_none() {
                return self.buckets[i];
            }
        }
        None
    }
    fn is_head(&self, k: u8) -> bool {
        let k = k as usize;
        for i in 0..256 {
            if !self.buckets[i].is_none() {
                return i == k;
            }
        }
        false
    }
    fn head(&self) -> Option<NonNull<TrieNode<T>>> {
        for i in 0..256 {
            if !self.buckets[i].is_none() {
                return self.buckets[i];
            }
        }
        None
    }

    fn is_tail(&self, k: u8) -> bool {
        let k = k as usize;
        for i in 0..256 {
            if !self.buckets[255 - i].is_none() {
                return i == k;
            }
        }
        false
    }

    fn tail(&self) -> Option<NonNull<TrieNode<T>>> {
        for i in 0..256 {
            if !self.buckets[255 - i].is_none() {
                return self.buckets[i];
            }
        }
        None
    }
    fn keys(&self) -> Vec<u8> {
        let mut keys = Vec::with_capacity(10);
        for i in 0..256 {
            if let Some(_) = self.buckets[i] {
                keys.push(i as u8)
            }
        }
        return keys;
    }

    fn pad() -> u8 {
        return 0;
    }
}
