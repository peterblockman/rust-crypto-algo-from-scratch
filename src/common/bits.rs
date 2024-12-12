use core::fmt;
use std::borrow::Cow;

use super::hex::Hex;

pub struct Bits<'a>(Cow<'a, [u8]>);

impl<'a> Bits<'a> {
    pub fn new<T: ?Sized + AsRef<[u8]>>(data: &'a T) -> Self {
        Self(Cow::Borrowed(data.as_ref()))
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }

    pub fn as_vec_bytes(&self) -> Vec<u8> {
        self.0
            .chunks(8)
            .map(|chunk| {
                let byte = chunk
                    .iter()
                    .enumerate()
                    .map(|(j, &bit)| (bit as u8) << j)
                    .sum::<u8>();
                byte
            })
            .collect::<Vec<u8>>()
    }

    pub fn update_bit(&mut self, index: usize, bit: u8) {
        self.0.to_mut()[index] = bit;
    }

    pub fn shift_left(&mut self, n: usize) {
        self.0.to_mut().rotate_left(n);

        for i in self.0.len() - n..self.0.len() {
            self.0.to_mut()[i] = 0;
        }
    }

    pub fn shift_right(&mut self, n: usize) {
        self.0.to_mut().rotate_right(n);
        for i in 0..n {
            self.0.to_mut()[i] = 0;
        }
    }
}

impl From<Hex<'_>> for Bits<'_> {
    fn from(hex: Hex<'_>) -> Self {
        let bits = hex.to_bits_msb();
        Self(Cow::Owned(bits))
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
    let mut bits = Bits::new(&[0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1]);
    let slice = bits.as_slice();
    assert_eq!(slice, &[0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1]);
    assert_eq!(format!("{}", bits), "0101010101010101");
    assert_eq!(format!("{:?}", bits), "01010101 01010101");

    bits.update_bit(0, 1);
    assert_eq!(format!("{}", bits), "1101010101010101");

    bits.shift_left(1);
    assert_eq!(format!("{}", bits), "1010101010101010");

    bits.shift_right(1);
    assert_eq!(format!("{}", bits), "0101010101010101");

    let hex = Hex::new("hello");
    let bits = Bits::from(hex);
    assert_eq!(
        format!("{}", bits),
        "0110100001100101011011000110110001101111"
    );
}
