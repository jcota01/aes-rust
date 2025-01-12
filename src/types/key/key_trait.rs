use super::aes128::Key128;
use super::aes192::Key192;
use super::aes256::Key256;
use super::Word;

pub trait Key{
    fn expand_key(&mut self);

    fn expanded_to_string(&self) -> String;

    fn get_round_key(&self, round: usize) -> [Word; 4];

    fn get_rounds(&self) -> usize;
}

pub struct AesKey
{
    key_struct: Box<dyn Key>
}

impl AesKey
{
    pub fn new128(key_bytes: &[u8; 16]) -> AesKey{
        AesKey{
            key_struct: Box::new(Key128::new(key_bytes))
        }
    }

    pub fn new192(key_bytes: &[u8; 24]) -> AesKey{
        AesKey{
            key_struct: Box::new(Key192::new(key_bytes))
        }
    }

    pub fn new256(key_bytes: &[u8; 32]) -> AesKey{
        AesKey{
            key_struct: Box::new(Key256::new(key_bytes))
        }
    }
    
    pub fn get_round_key(&self, round: usize) -> [Word; 4]{
        self.key_struct.get_round_key(round)
    }

    pub fn get_rounds(&self) -> usize{
        self.key_struct.get_rounds()
    }
}