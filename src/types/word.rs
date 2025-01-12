use std::{fmt, ops::BitXor};
use crate::consts::sbox;


#[derive(Copy, Clone, Debug)]
pub struct Word{
    bytes: [u8; 4],
}

impl Word{
    pub fn new(bytes: &[u8;4]) -> Word{
        Word { bytes: *bytes }
    }

    pub fn rot_word(&self) -> Self{
        Word { 
            bytes: [
                self.bytes[1], self.bytes[2],
                self.bytes[3], self.bytes[0]
            ],
        }
    }

    pub fn sub_word(&self) -> Self{
        Word { bytes: sbox(&self.bytes) }
    }

    pub fn into_bytes(&self) -> [u8; 4]{
        self.bytes
    }
}

impl BitXor for Word{
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output{
        let mut new_bytes: [u8; 4] = [0; 4];

        // Iter over bytes of self and xor with bytes of rhs 
        for (i, byte) in self.bytes.iter().enumerate(){
            new_bytes[i] = byte ^ rhs.bytes[i];
        }
        
        Self{
            bytes: new_bytes
        }
    }
}

impl Default for Word{
    fn default() -> Self{
        Self{
            bytes: [0; 4]
        }
    }
}

impl PartialEq for Word{
    fn eq(&self, other: &Self) -> bool{
        for i in 0..4{
            if self.bytes[i] != other.bytes[i]{
                return false;
            }
        }

        true
    }
}

impl fmt::Display for Word{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        write!(f, "0x{:02x}{:02x}{:02x}{:02x}", self.bytes[0], self.bytes[1],
                                self.bytes[2], self.bytes[3])
    }
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_rot1(){
        let word = Word::new(&[0, 1, 2, 3]);
        let rot_word = word.rot_word();

        assert_eq!(rot_word.into_bytes(), [1, 2, 3, 0]);
    }

    #[test]
    fn test_rot2(){
        let word = Word::new(&[4, 25, 0, 18]);
        let rot_word = word.rot_word();
        
        assert_eq!(rot_word.into_bytes(), [25, 0, 18, 4]);
    }

    #[test]
    fn test_display(){
        let word = Word::new(&[0, 1, 2, 3]);

        let word_str = format!("{}", word);

        assert_eq!(word_str, String::from("0x00010203"))
    }
}