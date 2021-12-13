#[derive(Debug)]
#[derive(Clone)]
pub enum ErrorKind {
    Notfound,
    KeySizeNotMatch,
    Unexpect,
    Common(u16, String),
}

#[derive(Debug)]
pub struct TrieError {
    kind: ErrorKind,
}

impl std::fmt::Display for TrieError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //format!("trie error, code:{},msg:{}", self.code, self.msg)
        match self.kind.clone() {
            ErrorKind::Notfound => {
                write!(f, "trie error, {:?}", self.kind)
            }
            ErrorKind::Common(code, msg) => {
                write!(f, "trie error, code:{:?}, msg:{}", code, msg)
            }
            _ => {
                write!(f, "trie error, {:?}", self.kind)
            }
        }
    }
}

impl TrieError {
    pub fn new(kind: ErrorKind) -> TrieError {
        TrieError { kind: kind }
    }
    pub fn from(kind: ErrorKind) -> TrieError {
        TrieError { kind }
    }
}

impl std::error::Error for TrieError {}
