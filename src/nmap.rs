use crate::error::TrieError;
use crate::trie::Container;
use crate::trie::TrieNode;
use std::cell::RefCell;
use std::fmt::Debug;
use std::ops::Deref;
use std::rc::Rc;

#[derive(Debug)]
pub struct Nmap<T>
where
    T: Default + Copy,
{
    buckets: [Option<Rc<RefCell<TrieNode<T>>>>; 10],
}

impl<T> std::fmt::Display for Nmap<T>
where
    T: Copy + Default + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut msg = String::from("[");
        for i in 0..10 {
            let mut a = String::from("");
            if let Some(node) = self.buckets[i].clone() {
                a = format!("{}", node.as_ptr() as u8);
            } else {
                a = String::from("null")
            }
            msg = format!("{},{}", msg, a);
        }
        write!(f, "{}]", msg)?;
        Ok(())
    }
}

impl<T> Container<T> for Nmap<T>
where
    T: Default + Copy + Debug + PartialEq,
{
    fn new() -> Nmap<T> {
        Nmap {
            buckets: [None, None, None, None, None, None, None, None, None, None],
        }
    }
    fn get(&self, k: u8) -> Option<Rc<RefCell<TrieNode<T>>>> {
        self.buckets[k as usize].clone()
    }

    fn set(&mut self, k: u8, v: Option<Rc<RefCell<TrieNode<T>>>>) {
        self.buckets[k as usize] = v;
    }
}
