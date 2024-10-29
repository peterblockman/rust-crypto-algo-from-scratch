// Linear Congruential Generator
use std::num::Wrapping;

struct LCG {
    m: u64,
    a: u64,
    b: u64,
    x: Wrapping<u64>,
}

impl LCG {
    pub fn new(seed: u64, m: u64, a: u64, b: u64) -> Self {
        Self {
            m,
            a,
            b,
            x: Wrapping(seed),
        }
    }

    pub fn next(&mut self) -> u64 {
        self.x = (self.x * Wrapping(self.a) + Wrapping(self.b)) % Wrapping(self.m);
        self.x.0
    }
}

impl Iterator for LCG {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        Some(self.next())
    }
}

#[test]
fn test_lcg() {
    let mut lcg = LCG::new(312, 100000, 70495, 24245);

    let random_number = lcg.next();

    assert_eq!(random_number, 18685);

    let expected_numbers = vec![23320, 67645, 58520];

    for (random_number, expected_number) in lcg.take(3).zip(expected_numbers) {
        assert_eq!(random_number, expected_number);
    }

    let mut lcg = LCG::new(2u64.pow(10), 2u64.pow(32) - 1, 12345, 678910);

    let random_number = lcg.next();

    assert_eq!(random_number, 13320190);

    let random_number = lcg.next();

    assert_eq!(random_number, 1229667250);
}
