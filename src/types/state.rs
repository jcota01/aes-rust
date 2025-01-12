use crate::consts::{mix_col, sbox};

use super::Word;



pub struct State{
    bytes: [[u8; 4]; 4],
}

impl State{
    pub fn new(bytes: &[u8; 16]) -> Self{
        let mut b_array: [[u8;4]; 4] = [[0; 4]; 4];

        for (i, byte) in bytes.iter().enumerate(){
            b_array[i / 4][i % 4] = *byte;
        }
        State { bytes: b_array }
    }

    pub fn sub_bytes(&mut self){
        let b_array = self.bytes.map(|b| sbox(&b));

        self.bytes = b_array;
    }

    pub fn shift_rows(&mut self){
        let mut b_array = [[0; 4];4];
        for i in  0..4{
            b_array[i] = [self.bytes[(i) % 4][0], self.bytes[(i + 1) % 4][1], 
                            self.bytes[(i + 2) % 4][2], self.bytes[(i + 3) % 4][3]];
        }

        self.bytes = b_array;
    }

    pub fn mix_cols(&mut self){
        for i in 0..4{
            self.bytes[i] = mix_col(&self.bytes[i]);
        }
    }

    pub fn add_round_key(&mut self, key: &[Word; 4]){
        for i in 0..4{
            let mut col = self.bytes[i];
            let word = key[i].into_bytes();

            for j in 0..4{
                col[j] ^= word[j];
            }

            self.bytes[i] = col;
        }
    }

    pub fn to_bytes(&self) -> [u8; 16]{
        let mut b_array: [u8; 16] = [0; 16];

        for i in 0..16{
            b_array[i] = self.bytes[i / 4][i % 4];
        }

        b_array
    }
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_shift_rows(){
        let mut input = State{
            bytes:
            [
                [0x0, 0x4, 0x8, 0xc],
                [0x1, 0x5, 0x9, 0xd],
                [0x2, 0x6, 0xa, 0xe],
                [0x3, 0x7, 0xb, 0xf]
            ]
        };

        input.shift_rows();


        let expected_bytes: [[u8; 4]; 4] = [
            [0x0, 0x5, 0xa, 0xf],
            [0x1, 0x6, 0xb, 0xc],
            [0x2, 0x7, 0x8, 0xd],
            [0x3, 0x4, 0x9, 0xe]
        ];
        
        assert_eq!(input.bytes, expected_bytes);
    }
}