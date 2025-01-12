use types::{AesKey, State};

mod types;
mod consts;

pub fn aes128(input: &[u8], key_bytes: &[u8; 16]) -> Vec<u8>{
    let key = AesKey::new128(&key_bytes);

    let mut input_state = [0u8; 16];
    input.iter().enumerate().for_each(|(i, x)| input_state[i] = *x);

    let mut state = State::new(&input_state);

    encrypt(&mut state, &key);

    state.to_bytes().to_vec()
}

pub fn aes192(input: &[u8], key_bytes: &[u8; 24]) -> Vec<u8>{
    let key = AesKey::new192(&key_bytes);

    let mut input_state = [0u8; 16];
    input.iter().enumerate().for_each(|(i, x)| input_state[i] = *x);

    let mut state = State::new(&input_state);

    encrypt(&mut state, &key);

    state.to_bytes().to_vec()
}

pub fn aes256(input: &[u8], key_bytes: &[u8; 32]) -> Vec<u8>{
    let key = AesKey::new256(&key_bytes);

    let mut input_state = [0u8; 16];
    input.iter().enumerate().for_each(|(i, x)| input_state[i] = *x);

    let mut state = State::new(&input_state);

    encrypt(&mut state, &key);

    state.to_bytes().to_vec()
}

fn encrypt(state: &mut State, key: &AesKey){
    // pre whiten
    state.add_round_key(&key.get_round_key(0));

    for i in 1..=key.get_rounds(){
        state.sub_bytes();
        state.shift_rows();

        if i != key.get_rounds(){
            state.mix_cols();
        }

        state.add_round_key(&key.get_round_key(i));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aes128_1(){
        let input: [u8; 16] = [
            0x32, 0x43, 0xf6, 0xa8, 0x88, 0x5a, 0x30, 0x8d, 
            0x31, 0x31, 0x98, 0xa2, 0xe0, 0x37, 0x07, 0x34
        ];

        let key_bytes: [u8; 16] = [
            0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6,
            0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf, 0x4f, 0x3c
        ];

        let out = aes128(&input, &key_bytes);

        assert_eq!(
            out,
            vec![
                0x39, 0x25, 0x84, 0x1d,
                0x02, 0xdc, 0x09, 0xfb,
                0xdc, 0x11, 0x85, 0x97,
                0x19, 0x6a, 0x0b, 0x32
            ]
        )
    }

    #[test]
    fn test_aes192_1(){
        let input: [u8; 16] = [
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff
        ];

        let key_bytes: [u8; 24] = [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 
            0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17
        ];

        let out = aes192(&input, &key_bytes);

        assert_eq!(
            out,
            vec![
                0xdd, 0xa9, 0x7c, 0xa4, 0x86, 0x4c, 0xdf, 0xe0, 0x6e, 0xaf, 0x70, 0xa0, 0xec, 0x0d, 0x71, 0x91
            ]
        );
    }

    #[test]
    fn test_aes256_1(){
        let input: [u8; 16] = [
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff
        ];

        let key_bytes: [u8; 32] = [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 
            0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
            0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f
        ];

        let out = aes256(&input, &key_bytes);

        assert_eq!(
            out,
            vec![
                0x8e, 0xa2, 0xb7, 0xca, 0x51, 0x67, 0x45, 0xbf, 0xea, 0xfc, 0x49, 0x90, 0x4b, 0x49, 0x60, 0x89
            ]
        );
    }

}
