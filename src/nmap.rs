use crate::error::TrieError;
use crate::trie::Container;
use crate::TrieNode;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Nmap<T>
where
    T: Default + Copy,
{
    buckets: [*const TrieNode<T>; 10],
}

impl<T> std::fmt::Display for Nmap<T>
where
    T: Copy + Default + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut msg = String::from("[");
        for i in 0..10 {
            let mut a = String::from("");
            if self.buckets[i].is_null() {
                a = String::from("null")
            } else {
                a = format!("{}", self.buckets[i] as u8);
            }
            msg = format!("{},{}", msg, a);
        }
        write!(f, "{}]", msg);
        Ok(())
    }
}

impl<T> Container<T> for Nmap<T>
where
    T: Default + Copy + Debug,
{
    fn new() -> Nmap<T> {
        Nmap {
            buckets: [
                std::ptr::null(),
                std::ptr::null(),
                std::ptr::null(),
                std::ptr::null(),
                std::ptr::null(),
                std::ptr::null(),
                std::ptr::null(),
                std::ptr::null(),
                std::ptr::null(),
                std::ptr::null(),
            ],
        }
    }
    fn get(&self, k: u8) -> Result<*const TrieNode<T>, TrieError> {
        let v = self.buckets[k as usize];
        if v.is_null() {
            return Err(TrieError::new(1, "not found"));
        }
        return Ok(v);
    }

    fn set(&mut self, k: u8, v: TrieNode<T>) {
        self.buckets[k as usize] = &v as *const TrieNode<T>;
        ()
    }
}
