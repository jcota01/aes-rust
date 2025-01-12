use crate::consts::rcon;

use super::{Word, Key};

pub struct Key256{
    original_key: [Word; 8],
    expanded_key: [Word; 4 * 15] // 15 keys
}

impl Key256{
    pub fn new(key_bytes: &[u8; 32]) -> Self{
        let mut key = Self{
            original_key: [Default::default(); 8],
            expanded_key: [Default::default(); 4 * 15]
        };

        for i in 0..8{
            let new_word = Word::new(&[key_bytes[i * 4], key_bytes[(i*4)+1], key_bytes[(i*4)+2], key_bytes[(i*4)+3]]);
            key.original_key[i] = new_word;
        }

        key.expand_key();

        key
    }
}

impl Key for Key256{
    fn expand_key(&mut self){
        let word_total = 4 * 15;

        for i in 0..8{
            self.expanded_key[i] = self.original_key[i];
        }

        for j in 8..word_total{
            let mut temp = self.expanded_key[j-1];

            if j % 8 == 0{
                temp = (temp.rot_word().sub_word()) ^ 
                    Word::new(&rcon(j/8));
            }
            else if j % 8 == 4 {
                temp = temp.sub_word();
            }

            self.expanded_key[j] = temp ^ self.expanded_key[j-8];
        }
    }

    fn expanded_to_string(&self) -> String{
        let mut ek_str = String::new();

        for i in 0..15{
            for j in 0..4{
                ek_str.push_str(&format!("{}", self.expanded_key[(i * 4) + j]));

                match j{
                    3 => if i != 14{ ek_str.push('\n'); },
                    _ => ek_str.push(' '),
                }
            }
        }

        ek_str
    }

    fn get_round_key(&self, round: usize) -> [Word; 4]{
        let words = round * 4;

        let mut key: [Word; 4] = [Default::default(); 4];
        for i in 0..4{
            key[i] = self.expanded_key[words + i];
        }

        key
    }

    fn get_rounds(&self) -> usize{
        14
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_key256_1(){
        let key = Key256::new(
            &[0x60, 0x3d, 0xeb, 0x10, 0x15, 0xca, 0x71, 0xbe, 
            0x2b, 0x73, 0xae, 0xf0, 0x85, 0x7d, 0x77, 0x81,
            0x1f, 0x35, 0x2c, 0x07, 0x3b, 0x61, 0x08, 0xd7, 
            0x2d, 0x98, 0x10, 0xa3, 0x09, 0x14, 0xdf, 0xf4]
        );

        assert_eq!(
            &key.expanded_to_string(),
            "0x603deb10 0x15ca71be 0x2b73aef0 0x857d7781\n\
            0x1f352c07 0x3b6108d7 0x2d9810a3 0x0914dff4\n\
            0x9ba35411 0x8e6925af 0xa51a8b5f 0x2067fcde\n\
            0xa8b09c1a 0x93d194cd 0xbe49846e 0xb75d5b9a\n\
            0xd59aecb8 0x5bf3c917 0xfee94248 0xde8ebe96\n\
            0xb5a9328a 0x2678a647 0x98312229 0x2f6c79b3\n\
            0x812c81ad 0xdadf48ba 0x24360af2 0xfab8b464\n\
            0x98c5bfc9 0xbebd198e 0x268c3ba7 0x09e04214\n\
            0x68007bac 0xb2df3316 0x96e939e4 0x6c518d80\n\
            0xc814e204 0x76a9fb8a 0x5025c02d 0x59c58239\n\
            0xde136967 0x6ccc5a71 0xfa256395 0x9674ee15\n\
            0x5886ca5d 0x2e2f31d7 0x7e0af1fa 0x27cf73c3\n\
            0x749c47ab 0x18501dda 0xe2757e4f 0x7401905a\n\
            0xcafaaae3 0xe4d59b34 0x9adf6ace 0xbd10190d\n\
            0xfe4890d1 0xe6188d0b 0x046df344 0x706c631e"
        )
    }
}