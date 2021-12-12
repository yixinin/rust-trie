use crate::error::TrieError;
use crate::nmap::Nmap;
use std::marker::PhantomData;
use std::ptr::NonNull;
use std::{fmt, mem};
pub trait Container<T> {
    fn new() -> Self;
    fn set(&mut self, k: u8, v: Option<NonNull<TrieNode<T>>>);
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
    key: Option<Vec<u8>>,
    node_key: u8,
    val: Option<T>,
    prev: Option<NonNull<TrieNode<T>>>,
    next: Option<NonNull<TrieNode<T>>>,
    children: Option<NonNull<Nmap<T>>>,
}

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

impl<T> TrieNode<T> {
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
            if let Some(cur_node) = cur {
                for k in key.clone() {
                    if let Some(children) = (*cur_node.as_ptr()).children {
                        let opt_node = (*children.as_ptr()).get(k);
                        if let None = opt_node {
                            let node;
                            if k == endk {
                                node = Some(
                                    Box::leak(Box::new(TrieNode::leaf(
                                        k,
                                        key.clone(),
                                        val.clone(),
                                    )))
                                    .into(),
                                )
                            } else {
                                node = Some(Box::leak(Box::new(TrieNode::<T>::new(k))).into());
                            }
                            (*children.as_ptr()).set(k, node);
                            let child_tail = (*children.as_ptr()).tail();
                            cur = node;
                            continue;
                        }
                        cur = opt_node
                    }
                    break;
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
            if let Some(cur_node) = cur {
                for k in key {
                    if let Some(children) = (*cur_node.as_ptr()).children {
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

    pub fn del(&mut self, key: Vec<u8>) {}
}
