use crate::error::TrieError;
use crate::nmap::Nmap;
use std::borrow::BorrowMut;
use std::marker::PhantomData;
use std::ptr::NonNull;
use std::{fmt, mem};
pub trait Container<T> {
    fn new() -> Self;
    fn set(&mut self, k: u8, v: NonNull<TrieNode<T>>);
    fn get(&self, k: u8) -> Option<NonNull<TrieNode<T>>>;
    fn del(&mut self, k: u8) -> bool;
    fn prev(&self, k: u8) -> Option<NonNull<TrieNode<T>>>;
    fn next(&self, k: u8) -> Option<NonNull<TrieNode<T>>>;
    fn is_head(&self, k: u8) -> bool;
    fn head(&self) -> Option<NonNull<TrieNode<T>>>;
    fn is_tail(&self, k: u8) -> bool;
    fn tail(&self) -> Option<NonNull<TrieNode<T>>>;
    fn keys(&self) -> Vec<u8>;
    fn pad() -> u8;
}

pub struct TrieNode<T> {
    pub key: Option<Vec<u8>>,
    pub node_key: u8,
    pub val: Option<T>,
    pub prev: Option<NonNull<TrieNode<T>>>,
    pub next: Option<NonNull<TrieNode<T>>>,
    pub children: Option<NonNull<Nmap<T>>>,
}

// impl<T> Drop for TrieNode<T> {
//     fn drop(&mut self) {
//         self.children = None;
//         self.prev = None;
//         self.next = None;
//         self.val = None;
//         self.key = None;
//     }
// }

#[derive(Debug, Clone)]
pub struct Trie<T> {
    key_size: usize,
    root: Option<NonNull<TrieNode<T>>>,
    head: Option<NonNull<TrieNode<T>>>,
    tail: Option<NonNull<TrieNode<T>>>,
    size: usize,
    marker: PhantomData<TrieNode<T>>,
}

#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct Iter<'a, T: 'a> {
    key_size: usize,
    root: Option<NonNull<TrieNode<T>>>,
    head: Option<NonNull<TrieNode<T>>>,
    tail: Option<NonNull<TrieNode<T>>>,
    size: usize,
    marker: PhantomData<&'a TrieNode<T>>,
}

impl<T: fmt::Debug> fmt::Debug for Iter<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Iter")
            .field(&*mem::ManuallyDrop::new(Trie {
                key_size: self.key_size,
                root: self.root,
                head: self.head,
                tail: self.tail,
                size: self.size,
                marker: PhantomData,
            }))
            .field(&self.size)
            .finish()
    }
}

impl<T> Clone for Iter<'_, T> {
    fn clone(&self) -> Self {
        Iter { ..*self }
    }
}

pub struct IterMut<'a, T: 'a> {
    key_size: usize,
    root: Option<NonNull<TrieNode<T>>>,
    head: Option<NonNull<TrieNode<T>>>,
    tail: Option<NonNull<TrieNode<T>>>,
    size: usize,
    marker: PhantomData<&'a mut TrieNode<T>>,
}

impl<T: fmt::Debug> fmt::Debug for IterMut<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("IterMut")
            .field(&*mem::ManuallyDrop::new(Trie {
                key_size: self.key_size,
                root: self.root,
                head: self.head,
                tail: self.tail,
                size: self.size,
                marker: PhantomData,
            }))
            .field(&self.size)
            .finish()
    }
}

#[derive(Clone)]
pub struct IntoIter<T> {
    list: Trie<T>,
}

impl<T: fmt::Debug> fmt::Debug for IntoIter<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("IntoIter").field(&self.list).finish()
    }
}

impl<T> TrieNode<T>
where
    T: Clone,
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

    pub fn into_val(self: Box<Self>) -> Option<T> {
        self.val
    }
}

impl<T> Trie<T>
where
    T: Clone,
{
    pub fn new(key_size: usize) -> Trie<T> {
        Trie {
            key_size,
            root: Some(Box::leak(Box::new(TrieNode::root())).into()),
            head: None,
            tail: None,
            size: 0,
            marker: PhantomData,
        }
    }

    pub fn set(&mut self, key: Vec<u8>, val: T) -> Result<(), TrieError> {
        if key.clone().len() != self.key_size {
            return Err(TrieError::new(1, "key size not match"));
        }

        let endk = key.clone()[self.key_size - 1];

        unsafe {
            let mut cur = self.root;
            for k in key.clone() {
                if let Some(cur_ptr) = cur {
                    if let Some(children_ptr) = (*cur_ptr.as_ptr()).children {
                        let children = (*children_ptr.as_ptr()).borrow_mut();

                        let node = children.get(k);
                        if let None = node {
                            let node_ptr;
                            if k == endk {
                                node_ptr = Box::leak(Box::new(TrieNode::leaf(
                                    k,
                                    key.clone(),
                                    val.clone(),
                                )))
                                .into();
                            } else {
                                node_ptr = Box::leak(Box::new(TrieNode::<T>::new(k))).into();
                            }
                            children.set(k, node_ptr);
                            if children.is_tail(k) {
                                if let Some(next) = (*cur_ptr.as_ptr()).next {
                                    if let Some(next_children) = (*next.as_ptr()).children {
                                        if let Some(next_head) = (*next_children.as_ptr()).head() {
                                            (*next_head.as_ptr()).prev = Some(node_ptr);
                                            (*node_ptr.as_ptr()).next = Some(next_head);
                                        }
                                    }
                                }
                            }
                            if children.is_head(k) {
                                if let Some(prev) = (*cur_ptr.as_ptr()).prev {
                                    if let Some(prev_children) = (*prev.as_ptr()).children {
                                        if let Some(prev_tail) = (*prev_children.as_ptr()).tail() {
                                            (*prev_tail.as_ptr()).next = Some(node_ptr);
                                            (*node_ptr.as_ptr()).prev = Some(prev);
                                        }
                                    }
                                }
                            }
                            cur = Some(node_ptr);
                            continue;
                        } else {
                            cur = node;
                            continue;
                        }
                    }
                    return Err(TrieError::new(2, "trie stucture fail"));
                }
                return Err(TrieError::new(2, "trie stucture fail"));
            }
            if self.head.is_none() {
                self.head = cur;
                self.tail = cur;
            } else {
                if let Some(head) = self.head {
                    if !(*head.as_ptr()).prev.is_none() {
                        if (*head.as_ptr()).prev == cur {
                            self.head = cur;
                        }
                    }
                }

                if let Some(tail) = self.tail {
                    if !(*tail.as_ptr()).next.is_none() {
                        if Some((*tail.as_ptr()).next) == Some(cur) {
                            self.tail = cur;
                        }
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
            for k in key {
                if let Some(cur_ptr) = cur {
                    if let Some(children) = (*cur_ptr.as_ptr()).children {
                        let node = (*children.as_ptr()).get(k);
                        if let Some(node_v) = node {
                            if let Some(val) = (*node_v.as_ptr()).val.clone() {
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

    pub fn del(&mut self, key: Vec<u8>) -> bool {
        if self.key_size != key.len() {
            return false;
        }
        let mut stack = std::collections::LinkedList::<NonNull<TrieNode<T>>>::new();

        unsafe {
            let mut cur = self.root;
            for k in key {
                if let Some(cur_ptr) = cur {
                    if let Some(children) = (*cur_ptr.as_ptr()).children {
                        let node = (*children.as_ptr()).get(k);
                        if let Some(node_v) = node {
                            if !(*node_v.as_ptr()).val.is_none() {
                                if let Some(n) = stack.pop_back() {
                                    if let Some(children) = (*n.as_ptr()).children {
                                        (*children.as_ptr()).del(k);
                                    }
                                    stack.push_back(n);
                                }
                            } else {
                                stack.push_back(node_v);
                            }
                        }
                        cur = node;
                        continue;
                    }
                    return false;
                }
                return false;
            }
            loop {
                if let Some(n) = stack.pop_back() {
                    if let Some(children) = (*n.as_ptr()).children {
                        if (*children.as_ptr()).head().is_none() {
                            if let Some(temp) = stack.pop_back() {
                                if let Some(children) = (*temp.as_ptr()).children {
                                    (*children.as_ptr()).del((*n.as_ptr()).node_key);
                                }
                                stack.push_back(temp);
                            }
                        } else {
                            break;
                        }
                    }
                }
            }
        }
        true
    }
}
