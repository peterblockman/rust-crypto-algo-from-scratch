use std::{
    fmt::{self, Debug},
    num::ParseIntError,
    str::Utf8Error,
};

use thiserror::Error;

#[derive(Debug, Clone)]
pub struct Hex<'a>(&'a [u8]);

impl<'a> Hex<'a> {
    pub fn new<T: ?Sized + Debug + AsRef<[u8]>>(data: &'a T) -> Result<Self, HexError> {
        let bytes = data.as_ref();

        // Check if it's a hex string
        if bytes.iter().all(|&c| c.is_ascii_hexdigit()) && bytes.len() % 2 == 0 {
            let parsed = Self::hex_to_bytes(bytes)?;
            // use Box::leak to extend the lifetime
            Ok(Self(Box::leak(parsed.into_boxed_slice())))
        } else {
            Ok(Self(bytes))
        }
    }
}

impl fmt::Display for Hex<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in &*self.0 {
            // format as uppercase hex
            write!(f, "{:X}", byte)?;
        }
        Ok(())
    }
}

impl Hex<'_> {
    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }

    /// convert hex to bits in msb-first order
    pub fn to_bits_msb(&self) -> Vec<u8> {
        let mut bits = Vec::with_capacity(self.0.len() * 8);

        for byte in self.0 {
            for i in (0..8).rev() {
                bits.push((byte >> i) & 1);
            }
        }

        bits
    }

    /// convert hex to bits in lsb-first order
    /// The bytes sequence is reversed first
    /// then each byte is converted to bits (bits are in msb-first order)
    pub fn to_bits_lsb(&self) -> Vec<u8> {
        let mut bits = Vec::with_capacity(self.0.len() * 8);

        for byte in self.0.iter().rev() {
            for i in (0..8).rev() {
                bits.push((byte >> i) & 1);
            }
        }

        bits
    }

    // ref: https://stackoverflow.com/a/52992629/10104154
    // convert hex string to bytes (ASCII values)
    pub fn hex_to_bytes(hex: &[u8]) -> Result<Vec<u8>, HexError> {
        (0..hex.len())
            .step_by(2)
            .try_fold(Vec::new(), |mut acc, i| {
                let pair = &hex[i..i + 2];
                let hex_str = std::str::from_utf8(pair)?;
                let byte = u8::from_str_radix(hex_str, 16)?;
                acc.push(byte);
                Ok(acc)
            })
    }
}

impl From<Hex<'_>> for u64 {
    fn from(hex: Hex<'_>) -> Self {
        hex.0.iter().fold(0, |acc, &byte| (acc << 8) | byte as u64)
    }
}

#[derive(Debug, Error, PartialEq)]
pub enum HexError {
    #[error("Invalid hex string")]
    InvalidHexString,

    #[error("Parse int error: {0}")]
    ParseIntError(#[from] ParseIntError),

    #[error("Utf8 error: {0}")]
    Utf8Error(#[from] Utf8Error),
}

#[test]
fn test_hex() {
    let text = "hello";
    let hex_string = "68656C6C6F";
    let hex = Hex::new(&text).unwrap();
    assert_eq!(format!("{}", hex), hex_string);

    let bits = hex.to_bits_msb();
    assert_eq!(
        bits,
        vec![
            0, 1, 1, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 0, 1,
            1, 0, 0, 0, 1, 1, 0, 1, 1, 1, 1
        ]
    );

    let hex = Hex::new(&[0x68, 0x65, 0x6C, 0xFD, 0xFA]).unwrap();
    assert_eq!(format!("{}", hex), "68656CFDFA");
}

#[test]
fn test_hex_to_u64() {
    let hex = Hex::new("68656CFDFA").unwrap();
    assert_eq!(u64::from(hex), 448378240506);
}

// TODO: test errors
