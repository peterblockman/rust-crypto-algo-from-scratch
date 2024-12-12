use std::{
    borrow::Cow,
    fmt::{self, Debug},
    num::ParseIntError,
};

#[derive(Debug)]
pub struct Hex<'a>(Cow<'a, [u8]>);

impl<'a> Hex<'a> {
    pub fn new<T: ?Sized + Debug + AsRef<[u8]>>(data: &'a T) -> Self {
        let bytes = data.as_ref();

        // Check if it's a hex string
        if bytes.iter().all(|&c| c.is_ascii_hexdigit()) && bytes.len() % 2 == 0 {
            let parsed = Self::hex_to_bytes(bytes).expect("Invalid hex string");
            // use Box::leak to extend the lifetime
            return Self(Cow::Owned(parsed));
        }

        Self(Cow::Borrowed(bytes))
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

    pub fn into_owned(self) -> Hex<'static> {
        Hex(Cow::Owned(self.0.into_owned()))
    }

    /// convert each byte into bits according msb-first order
    pub fn to_bits_msb(&self) -> Vec<u8> {
        let mut bits = Vec::with_capacity(self.0.len() * 8);

        for byte in &*self.0 {
            for i in (0..8).rev() {
                bits.push((byte >> i) & 1);
            }
        }

        bits
    }

    pub fn to_bits_lsb(&self) -> Vec<u8> {
        let mut bits = Vec::with_capacity(self.0.len() * 8);

        for byte in &*self.0 {
            for i in 0..8 {
                bits.push((byte >> i) & 1);
            }
        }

        bits
    }

    // ref: https://stackoverflow.com/a/52992629/10104154
    // convert hex string to bytes (ASCII values)
    pub fn hex_to_bytes(hex: &[u8]) -> Result<Vec<u8>, ParseIntError> {
        (0..hex.len())
            .step_by(2)
            .map(|i| {
                let pair = &hex[i..i + 2];
                let hex_str = std::str::from_utf8(pair).unwrap();
                u8::from_str_radix(hex_str, 16)
            })
            .collect()
    }
}

#[test]
fn test_hex() {
    let text = "hello";
    let hex_string = "68656C6C6F";
    let hex = Hex::new(&text);
    assert!(matches!(hex.0, Cow::Borrowed(_)));
    assert_eq!(format!("{}", hex), hex_string);

    let bits = hex.to_bits_msb();
    assert_eq!(
        bits,
        vec![
            0, 1, 1, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 0, 1,
            1, 0, 0, 0, 1, 1, 0, 1, 1, 1, 1
        ]
    );

    let hex = hex.into_owned();
    assert!(matches!(hex.0, Cow::Owned(_)));

    let hex = Hex::new(&hex_string);
    assert!(matches!(hex.0, Cow::Owned(_)));
}
