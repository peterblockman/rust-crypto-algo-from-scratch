use core::fmt;
use std::borrow::Cow;

use thiserror::Error;

use super::hex::Hex;

pub struct Bits<'a>(Cow<'a, [u8]>);

impl<'a> Bits<'a> {
    pub fn new<T: ?Sized + AsRef<[u8]>>(data: &'a T) -> Result<Self, BitsError> {
        let bits_ref = data.as_ref();
        bits_ref.iter().try_for_each(|bit| {
            if *bit != 0 && *bit != 1 {
                return Err(BitsError::InvalidBit(*bit));
            }
            Ok(())
        })?;

        Ok(Self(Cow::Borrowed(bits_ref)))
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }

    /// Construct bytes from their 8 bits.
    /// the bytes can be in msb-first order or lsb-first order (TODO test this)
    /// The individual bits chunks are in msb-first order
    pub fn as_vec_bytes(&self) -> Vec<u8> {
        self.0
            .chunks(8)
            .map(|bits| {
                let mut byte = 0;
                for bit in bits.iter() {
                    // Shift val left by 1 to make room for the next bit
                    byte <<= 1;
                    // Add the current bit (0 or 1) into val
                    byte |= bit & 1;
                }
                byte
            })
            .collect::<Vec<u8>>()
    }

    /// toggle reverse the bytes between lsb and msb
    pub fn reverse_mut(&mut self) {
        let mut rev = Vec::with_capacity(self.0.len());
        for byte in self.0.chunks(8).rev() {
            rev.extend(byte);
        }
        self.0 = Cow::Owned(rev);
    }

    pub fn reverse(&self) -> Vec<u8> {
        let mut rev = self.0.to_vec();
        rev.reverse();
        rev
    }

    pub fn get_bit(&self, index: usize) -> u8 {
        self.0[index]
    }

    pub fn update_bit(&mut self, index: usize, bit: u8) {
        self.0.to_mut()[index] = bit;
    }

    pub fn insert(&mut self, index: usize, bit: u8) {
        self.0.to_mut().insert(index, bit);
    }

    pub fn push(&mut self, bit: u8) {
        self.0.to_mut().push(bit);
    }

    pub fn shift_left(&mut self, n: usize) {
        for _ in 0..n {
            self.0.to_mut().push(0);
        }
    }

    pub fn shift_right(&mut self, n: usize) {
        self.0.to_mut().rotate_right(n);
        for i in 0..n {
            self.0.to_mut()[i] = 0;
        }
    }

    pub fn xor(&self, other: &Self) -> Result<Vec<u8>, BitsError> {
        if self.0.len() != other.0.len() {
            return Err(BitsError::LengthMismatch);
        }

        Ok(self
            .0
            .iter()
            .zip(other.0.iter())
            .try_fold(Vec::new(), |mut acc, (a, b)| {
                acc.push(*a ^ *b);
                Ok(acc)
            })?)
    }
}

#[derive(Debug, Error, PartialEq)]
pub enum BitsError {
    #[error("Invalid bit: {0}")]
    InvalidBit(u8),

    #[error("Length mismatch between bit sequences")]
    LengthMismatch,
}

impl From<Hex<'_>> for Bits<'_> {
    fn from(hex: Hex<'_>) -> Self {
        let bits = hex.to_bits_msb();
        Self(Cow::Owned(bits))
    }
}

impl<'a> From<Vec<u8>> for Bits<'a> {
    fn from(data: Vec<u8>) -> Self {
        Self(Cow::Owned(data))
    }
}

impl<'a> fmt::Display for Bits<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for bit in self.0.iter() {
            write!(f, "{}", bit)?;
        }

        Ok(())
    }
}

impl<'a> fmt::Debug for Bits<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, bit) in self.0.iter().enumerate() {
            write!(f, "{}", bit)?;
            if (i + 1) % 8 == 0 && i < self.0.len() - 1 {
                write!(f, " ")?;
            }
        }
        Ok(())
    }
}

#[test]
fn test_bits() {
    // "01101000 01101001" is "hi" in ascii
    let mut bits = Bits::new(&[0, 1, 1, 0, 1, 0, 0, 0, 0, 1, 1, 0, 1, 0, 0, 1]).unwrap();
    let slice = bits.as_slice();
    assert_eq!(slice, &[0, 1, 1, 0, 1, 0, 0, 0, 0, 1, 1, 0, 1, 0, 0, 1]);
    assert_eq!(format!("{}", bits), "0110100001101001");
    assert_eq!(format!("{:?}", bits), "01101000 01101001");

    bits.update_bit(0, 1);
    assert_eq!(format!("{}", bits), "1110100001101001");

    bits.shift_left(1);
    assert_eq!(format!("{}", bits), "11101000011010010");

    bits.shift_right(2);
    assert_eq!(format!("{}", bits), "00111010000110100");

    let hex = Hex::new("hello").unwrap();
    let bits = Bits::from(hex);
    assert_eq!(
        format!("{}", bits),
        "0110100001100101011011000110110001101111"
    );

    let hex = Hex::new("38EB86FF73").unwrap();
    let bits = Bits::from(hex);
    assert_eq!(
        format!("{}", bits),
        "0011100011101011100001101111111101110011"
    );

    let mut bits = Bits::new(&[1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1]).unwrap();

    bits.reverse_mut();

    assert_eq!(format!("{}", bits), "0000111111110000");
}

#[test]
fn test_bits_error() {
    let bits = Bits::new(&[1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 2]).unwrap_err();
    assert_eq!(bits, BitsError::InvalidBit(2));

    // test length mismatch
    let bits = Bits::new(&[1, 1, 1, 1, 0, 0, 0, 0]).unwrap();
    let bits2 = Bits::new(&[1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1]).unwrap();
    let err = bits.xor(&bits2).unwrap_err();
    assert_eq!(err, BitsError::LengthMismatch);
}
