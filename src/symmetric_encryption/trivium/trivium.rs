use crate::common::bits::{Bits, BitsError};
use thiserror::Error;

#[derive(Debug)]
pub struct Trivium<'a> {
    state: Bits<'a>,
    key_stream: Bits<'a>,
    count: u16,
}

impl<'a> Trivium<'a> {
    // Register A indices
    const BIT66: usize = 65;
    const BIT69: usize = 68;
    const BIT91: usize = 90;
    const BIT92: usize = 91;
    const BIT93: usize = 92;
    const BIT94: usize = 93;

    // Register B indices
    const BIT162: usize = 161;
    const BIT171: usize = 170;
    const BIT175: usize = 174;
    const BIT176: usize = 175;
    const BIT177: usize = 176;
    const BIT178: usize = 177;

    // Register C indices
    const BIT243: usize = 242;
    const BIT264: usize = 263;
    const BIT286: usize = 285;
    const BIT287: usize = 286;
    const BIT288: usize = 287;

    pub fn new(key: &[u8], iv: &[u8]) -> Result<Self, TriviumError> {
        // Validate key length
        if key.len() != 80 {
            return Err(TriviumError::InvalidKeyLength(key.len()));
        }

        // Validate IV length
        if iv.len() != 80 {
            return Err(TriviumError::InvalidIVLength(iv.len()));
        }
        let mut ra = vec![0; 93]; // bit 1 -> 93 (93 bits)
        let mut rb = vec![0; 84]; // bit 94 -> 177 (84 bits)
        let mut rc = vec![0; 111]; // bit 178 -> 288 (111 bits)

        // load the key to the first 80 bits (bit 1 ->  80)
        for (i, byte) in key.iter().enumerate() {
            ra[i] = *byte;
        }

        // load the iv into the next 80 bits (bit 94 -> 173)
        for (i, byte) in iv.iter().enumerate() {
            rb[i] = *byte;
        }

        // set the last 3 bits of rc to 1
        let rc_len = rc.len();
        rc[rc_len - 1] = 1;
        rc[rc_len - 2] = 1;
        rc[rc_len - 3] = 1;

        let state = [ra, rb, rc].concat();

        Ok(Self {
            state: state.into(),
            key_stream: Bits::new(&[])?,
            count: 0,
        })
    }

    pub fn update_state(&mut self, count: u16) {
        // intermediate bits
        let mut t1 = self.state.get_bit(Self::BIT66) ^ self.state.get_bit(Self::BIT93);
        let mut t2 = self.state.get_bit(Self::BIT162) ^ self.state.get_bit(Self::BIT177);
        let mut t3 = self.state.get_bit(Self::BIT243) ^ self.state.get_bit(Self::BIT288);
        // output bits
        if count > 1151 {
            let z = t1 ^ t2 ^ t3;
            // because the key stream is in lsb, we need to insert the bit at the beginning
            self.key_stream.insert(0, z);
        }

        t1 = t1
            ^ (self.state.get_bit(Self::BIT91) & self.state.get_bit(Self::BIT92))
            ^ self.state.get_bit(Self::BIT171);
        t2 = t2
            ^ (self.state.get_bit(Self::BIT175) & self.state.get_bit(Self::BIT176))
            ^ self.state.get_bit(Self::BIT264);
        t3 = t3
            ^ (self.state.get_bit(Self::BIT286) & self.state.get_bit(Self::BIT287))
            ^ self.state.get_bit(Self::BIT69);

        // right shift 1 of the state
        self.state.shift_right(1);

        // feed the new values to the state
        self.state.update_bit(0, t3);
        self.state.update_bit(Self::BIT94, t1);
        self.state.update_bit(Self::BIT178, t2);

        self.count += 1;
    }

    pub fn warm_up(&mut self) {
        for _ in 0..4 * 288 {
            self.update_state(self.count);
        }
    }

    pub fn build_key_stream(&mut self, plain_text_length: usize) {
        for _ in 0..plain_text_length {
            self.update_state(self.count);
        }
    }

    pub fn encrypt(&mut self, plain_text: &[u8]) -> Result<Vec<u8>, TriviumError> {
        if self.count < 1152 {
            return Err(TriviumError::NotWarmedUp);
        }
        let plain_text_bits = Bits::new(plain_text)?;

        Ok(plain_text_bits.xor(&self.key_stream)?)
    }

    pub fn decrypt(&mut self, cipher_text: &[u8]) -> Result<Vec<u8>, TriviumError> {
        if self.count < 1152 {
            return Err(TriviumError::NotWarmedUp);
        }
        let cipher_text_bits = Bits::new(cipher_text)?;

        Ok(cipher_text_bits.xor(&self.key_stream)?)
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum TriviumError {
    #[error("Bits error: {0}")]
    BitsError(#[from] BitsError),

    #[error("Invalid key length: expected 10 bytes, got {0} bytes")]
    InvalidKeyLength(usize),

    #[error("Invalid IV length: expected 10 bytes, got {0} bytes")]
    InvalidIVLength(usize),

    #[error("Trivium is not warmed up")]
    NotWarmedUp,
}

#[test]
fn test_trivium() {
    use crate::common::hex::Hex;
    // test vector from: https://github.com/cantora/avr-crypto-lib/blob/master/testvectors/trivium-80.80.test-vectors
    let plain_text = "hello";
    let plain_text_hex = Hex::new(plain_text).unwrap();
    let key_hex = Hex::new("80000000000000000000").unwrap();
    let iv_hex = Hex::new("00000000000000000000").unwrap();
    let key_bits = key_hex.to_bits_lsb();
    let iv_bits = iv_hex.to_bits_lsb();

    assert_eq!(key_bits.len(), 80);
    assert_eq!(iv_bits.len(), 80);

    let mut trivium = Trivium::new(&key_bits, &iv_bits).unwrap();
    trivium.warm_up();
    assert_eq!(trivium.count, 1152);

    let plain_text_bits = plain_text_hex.to_bits_lsb();
    trivium.build_key_stream(plain_text_bits.len());

    // test vector key stream: 38EB86FF73
    // if we want to get the test vector value, we need to reverse the key stream using reverse_mut
    // because the key stream is in lsb-first order
    assert_eq!(
        format!("{}", Hex::new(&trivium.key_stream.as_vec_bytes()).unwrap()),
        "73FF86EB38"
    );

    let cipher: Vec<u8> = trivium.encrypt(&plain_text_bits).unwrap();
    let cipher_bits = Bits::new(&cipher).unwrap();
    // test vector cipher text: 508EEA931C
    assert_eq!(
        format!("{}", Hex::new(&cipher_bits.as_vec_bytes()).unwrap()),
        "1C93EA8E50"
    );

    let plain_text_decrypted = trivium.decrypt(&cipher).unwrap();
    assert_eq!(plain_text_decrypted, plain_text_bits);
}

#[test]
fn test_trivium_errors() {
    use crate::common::hex::Hex;

    let key_hex = Hex::new("800").unwrap();
    let iv_hex = Hex::new("00000000000000000000").unwrap();
    let key_bits = key_hex.to_bits_lsb();
    let iv_bits = iv_hex.to_bits_lsb();

    let trivium = Trivium::new(&key_bits, &iv_bits).unwrap_err();
    assert_eq!(trivium, TriviumError::InvalidKeyLength(24));

    let key_hex = Hex::new("80000000000000000000").unwrap();
    let iv_hex = Hex::new("0000").unwrap();
    let key_bits = key_hex.to_bits_lsb();
    let iv_bits = iv_hex.to_bits_lsb();

    let trivium = Trivium::new(&key_bits, &iv_bits).unwrap_err();
    assert_eq!(trivium, TriviumError::InvalidIVLength(16));

    // write test for BitsError in encrypt and decrypt
    let key_hex = Hex::new("80000000000000000000").unwrap();
    let iv_hex = Hex::new("00000000000000000000").unwrap();
    let key_bits = key_hex.to_bits_lsb();
    let iv_bits = iv_hex.to_bits_lsb();

    let mut trivium = Trivium::new(&key_bits, &iv_bits).unwrap();
    let err = trivium.encrypt(&[0, 1, 2]).unwrap_err();
    assert_eq!(err, TriviumError::NotWarmedUp);

    let err = trivium.decrypt(&[0, 1, 2]).unwrap_err();
    assert_eq!(err, TriviumError::NotWarmedUp);

    trivium.warm_up();
    let err = trivium.encrypt(&[0, 1, 2]).unwrap_err();
    assert_eq!(err, TriviumError::BitsError(BitsError::InvalidBit(2)));

    let err = trivium.decrypt(&[0, 1, 2]).unwrap_err();
    assert_eq!(err, TriviumError::BitsError(BitsError::InvalidBit(2)));
}
