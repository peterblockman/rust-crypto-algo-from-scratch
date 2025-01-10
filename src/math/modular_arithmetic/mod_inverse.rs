use super::gcd::egcd;

pub fn mod_inverse(a: i32, m: i32) -> Result<i32, String> {
    let (gcd, x, _y) = egcd(a, m);

    // modular inverse exists only if gcd(a, m) = 1
    if gcd > 1 {
        return Err("Modular inverse does not exist".to_string());
    }

    // ensure the result is a positive integer within [0, m - 1]
    Ok((x % m + m) % m)
}

#[test]
fn test_mod_inverse() {
    assert_eq!(mod_inverse(3, 26).unwrap(), 9);
    assert_eq!(
        mod_inverse(2, 26).unwrap_err(),
        "Modular inverse does not exist".to_string()
    );
}
