use crate::consts::rcon;
use crate::types::Word;

use super::Key;

#[derive(Debug)]
pub struct Key128{
    original_key: [Word; 4],
    expanded_key: [Word; 4 * 11] // 11 keys
}

impl Key128{
    pub fn new(key_bytes: &[u8; 16]) -> Self{
        let mut key = Self{
            original_key: [Default::default(); 4],
            expanded_key: [Default::default(); 4 * 11]
        };

        for i in 0..4{
            let new_word = Word::new(&[key_bytes[i*4], key_bytes[(i*4)+1], key_bytes[(i*4)+2], key_bytes[(i*4)+3]]);
            key.original_key[i] = new_word;
        }

        key.expand_key();

        key
    }
}

impl Key for Key128{
    fn expand_key(&mut self){
        // total number of words
        let words_total = 4 * 11;

        // First key is cipher key
        for i in 0..4{
            self.expanded_key[i] = self.original_key[i];
        }


        // For the rest of the keys
        for j in 4..words_total{
            // temp = previous word
            let mut temp: Word = self.expanded_key[j-1];

            if j % 4 == 0{
                // subword(rotword(last word)) xor rcon[i / 4]
                temp = (temp.rot_word().sub_word()) ^ 
                    Word::new(&rcon(j/4));
            }


            // New word is word[i-1] XOR word[j-4]
            self.expanded_key[j] = temp ^ self.expanded_key[j-4];
        }
    }

    fn expanded_to_string(&self) -> String{
        let mut ek_str = String::new();

        for i in 0..11{
            for j in 0..4{
                ek_str.push_str(&format!("{}", self.expanded_key[(i * 4) + j]));

                match j{
                    3 => if i != 10{ ek_str.push('\n'); },
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
        10
    }
}

#[cfg(test)]
mod tests{
    use super::*;


    #[test]
    fn test_key128_1(){
        let key = Key128::new(
            &[0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 
                        0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf, 0x4f, 0x3c]
        );

        assert_eq!(
            &key.expanded_to_string(), 
            "0x2b7e1516 0x28aed2a6 0xabf71588 0x09cf4f3c\n\
            0xa0fafe17 0x88542cb1 0x23a33939 0x2a6c7605\n\
            0xf2c295f2 0x7a96b943 0x5935807a 0x7359f67f\n\
            0x3d80477d 0x4716fe3e 0x1e237e44 0x6d7a883b\n\
            0xef44a541 0xa8525b7f 0xb671253b 0xdb0bad00\n\
            0xd4d1c6f8 0x7c839d87 0xcaf2b8bc 0x11f915bc\n\
            0x6d88a37a 0x110b3efd 0xdbf98641 0xca0093fd\n\
            0x4e54f70e 0x5f5fc9f3 0x84a64fb2 0x4ea6dc4f\n\
            0xead27321 0xb58dbad2 0x312bf560 0x7f8d292f\n\
            0xac7766f3 0x19fadc21 0x28d12941 0x575c006e\n\
            0xd014f9a8 0xc9ee2589 0xe13f0cc8 0xb6630ca6"
        )
    }
}