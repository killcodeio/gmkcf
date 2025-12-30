// Future home for Key Generation / Management logic
// that doesn't fit strictly into symmetric/asymmetric files.
pub struct KeyPair {
    pub public: Vec<u8>,
    pub private: Vec<u8>,
}
