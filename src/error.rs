#[derive(Debug)]
pub struct TrieError {
    code: u16,
    msg: String,
}

impl std::fmt::Display for TrieError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //format!("trie error, code:{},msg:{}", self.code, self.msg)
        write!(f, "trie error, code:{},msg:{}", self.code, self.msg)
    }
}

impl TrieError {
    pub fn new(code: u16, msg: &str) -> TrieError {
        TrieError {
            code,
            msg: String::from(msg),
        }
    }
}

impl std::error::Error for TrieError {}
