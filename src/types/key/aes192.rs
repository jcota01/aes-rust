use crate::consts::rcon;
use super::{Word, Key};

pub struct Key192{
    original_key: [Word; 6],
    expanded_key: [Word; 4 * 13] // 13 keys
}

impl Key192{
    pub fn new(key_bytes: &[u8; 24]) -> Self{
        let mut key = Self{
            original_key: [Default::default(); 6],
            expanded_key: [Default::default(); 4 * 13]
        };

        for i in 0..6{
            let new_word = Word::new(&[key_bytes[i*4], key_bytes[(i*4)+1], key_bytes[(i*4)+2], key_bytes[(i*4)+3]]);
            key.original_key[i] = new_word;
        }

        key.expand_key();

        key
    }
}

impl Key for Key192{
    fn expand_key(&mut self){
        let words_total = 4 * 13;

        // first key is cipher key
        for i in 0..6{
            self.expanded_key[i] = self.original_key[i];
        }

        // for the rest of the keys
        for j in 6..words_total{
            // temp = previous word
            let mut temp: Word = self.expanded_key[j-1];

            if j % 6 == 0{
                // subword(rotword(lastWord)) XOR rcon[i/6]
                temp = (temp.rot_word().sub_word()) ^
                    Word::new(&rcon(j/6));
            }

            // new word is temp XOR word[j-6]
            self.expanded_key[j] = temp ^ self.expanded_key[j-6];
        }
    }

    fn expanded_to_string(&self) -> String{
        let mut ek_str = String::new();

        for i in 0..13{
            for j in 0..4{
                ek_str.push_str(&format!("{}", self.expanded_key[(i * 4) + j]));

                match j{
                    3 => if i != 12{ ek_str.push('\n'); },
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
        12
    }
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_key192_1(){
        let key = Key192::new(
            &[0x8e, 0x73, 0xb0, 0xf7, 0xda, 0x0e, 0x64, 0x52, 0xc8, 0x10, 0xf3, 0x2b, 
            0x80, 0x90, 0x79, 0xe5, 0x62, 0xf8, 0xea, 0xd2, 0x52, 0x2c, 0x6b, 0x7b]
        );


        assert_eq!(
            &key.expanded_to_string(), 
            "0x8e73b0f7 0xda0e6452 0xc810f32b 0x809079e5\n\
            0x62f8ead2 0x522c6b7b 0xfe0c91f7 0x2402f5a5\n\
            0xec12068e 0x6c827f6b 0x0e7a95b9 0x5c56fec2\n\
            0x4db7b4bd 0x69b54118 0x85a74796 0xe92538fd\n\
            0xe75fad44 0xbb095386 0x485af057 0x21efb14f\n\
            0xa448f6d9 0x4d6dce24 0xaa326360 0x113b30e6\n\
            0xa25e7ed5 0x83b1cf9a 0x27f93943 0x6a94f767\n\
            0xc0a69407 0xd19da4e1 0xec1786eb 0x6fa64971\n\
            0x485f7032 0x22cb8755 0xe26d1352 0x33f0b7b3\n\
            0x40beeb28 0x2f18a259 0x6747d26b 0x458c553e\n\
            0xa7e1466c 0x9411f1df 0x821f750a 0xad07d753\n\
            0xca400538 0x8fcc5006 0x282d166a 0xbc3ce7b5\n\
            0xe98ba06f 0x448c773c 0x8ecc7204 0x01002202"
        );
    }
}