use crate::{trie::Container, trie::TrieNode};
use std::fmt::Debug;
use std::ptr::NonNull;

#[derive(Debug)]
pub struct Nmap<T> {
    buckets: [Option<NonNull<TrieNode<T>>>; 10],
}

impl<T> std::fmt::Display for Nmap<T>
where
    T: Copy + Default + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut msg = String::from("[");
        for i in 0..10 {
            let mut a: String = String::from("");
            if let Some(node) = self.buckets[i] {
                a = format!("{}", node.as_ptr() as u8);
            } else {
                a = String::from("null")
            }
            msg = format!("{},{},", msg, a);
        }
        write!(f, "{}]", msg)?;
        Ok(())
    }
}

impl<T> Container<T> for Nmap<T>
where
    T: Default + Copy + Debug,
{
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
}
