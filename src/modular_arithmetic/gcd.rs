/// Recursive Euclidean Algorithm
///
/// The key point of this algorithm:
/// - The divisor from the previous division becomes the next dividend.
/// - The remainder becomes the next divisor.
/// - This process continues until the remainder is 0, at which point the
///   GCD is found.
///
/// Example of how the division steps work:
///
/// - a / b = q1, r1
/// - b / r1 = q2, r2
/// - r1 / r2 = q3, r3
/// - r2 / r3 = q4, r4
/// - Continue until `rN = 0` and the corresponding dividend is the GCD.
///
/// Recursive relationship: $\gcd(a, b) = \gcd(b, a \bmod b)$
///
/// # See also
/// - [Euclidean algorithm explanation](https://scienceland.info/en/algebra8/euclid-algorithm)
pub fn gcd(a: i32, b: i32) -> i32 {
    // when we reach b == 0 then gcd(a, 0) = a
    if b == 0 {
        return a;
    }
    // passing b as dividend and a % b (the remainder) as divisor
    gcd(b, a % b).abs()
}

/// Iterative gcd
pub fn gcdi(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        // b is carried over as dividend
        let carry = b;
        // calculate the remainder
        b = a % b;
        // set a to the carry (b becomes the next dividend)
        a = carry;
    }
    a.abs()
}

///  Recursive Extended Euclidean Algorithm
///
/// #### Overview
///
/// The Extended Euclidean Algorithm not only computes the greatest common divisor (GCD) of two integers $ a $ and $ b $ but also finds integers $ x $ and $ y $ satisfying:
///
/// \[
/// \gcd(a, b) = a \times x + b \times y
/// \]
///
/// #### The Algorithm Steps
///
/// 1. **Base Case**: If $ b = 0 $, the GCD is $ a $, and the coefficients are $ x = 1 $ and $ y = 0 $:
///
///    \[
///    \gcd(a, 0) = a = a \times 1 + 0 \times 0
///    \]
///
/// 2. **Recursive Case**: If $ b \neq 0 $, we recursively call the function with $ (b, a \bmod b) $:
///
///    \[
///    \gcd(a, b) = \gcd(b, a \bmod b)
///    \]
///
/// #### Deriving the Update Formulas
///
/// Let's derive the update formulas for $ x $ and $ y $ based on the recursive call.
///
/// ##### Step 1: Recursive Call
///
/// We start with the recursive call:
///
/// \[
/// \gcd(b, a \bmod b) = b \times x + (a \bmod b) \times y
/// \]
///
/// This gives us:
///
/// 1. $ \gcd(b, a \bmod b) = d $
/// 2. Coefficients $ x $ and $ y $ such that:
///
///    \[
///    d = b \times x + (a \bmod b) \times y
///    \]
///
/// ##### Step 2: Express $ a \bmod b $ in Terms of $ a $ and $ b $
///
/// Recall that:
///
/// \[
/// a \bmod b = a - \left\lfloor \frac{a}{b} \right\rfloor \times b
/// \]
///
/// Let $ q = \left\lfloor \frac{a}{b} \right\rfloor $, so:
///
/// \[
/// a \bmod b = a - q \times b
/// \]
///
/// ##### Step 3: Substitute Back into the GCD Equation
///
/// Substitute $ a \bmod b $ into the equation:
///
/// \[
/// d = b \times x + (a - q \times b) \times y
/// \]
///
/// Simplify:
///
/// \[
/// d = b \times x + a \times y - q \times b \times y
/// \]
///
/// Group like terms:
///
/// \[
/// d = a \times y + b \times (x - q \times y)
/// \]
///
/// ##### Step 4: Identify New Coefficients $ x $ and $ y $
///
/// From the rearranged equation, we can identify:
///
/// - Coefficient of $ a $: $ y $
/// - Coefficient of $ b $: $ x - q \times y $
///
/// Thus, we can set:
///
/// \[
/// x = y \newline
/// y = x - q \times y
/// \]
///
/// This ensures:
///
/// \[
/// d = a \times x + b \times y
/// \]
pub fn egcd(a: i32, b: i32) -> (i32, i32, i32) {
    // when we reach b == 0, then gcd = a, xp = 1, yp = 0
    if b == 0 {
        return (a, 1, 0);
    }

    let (gcd, x, y) = egcd(b, a % b);

    // going backward to find previous x_p and y_p
    let x_p = y;

    let y_p = x - a / b * y;

    return (gcd, x_p, y_p);
}

/// Iterative Extended Euclidean algorithm
///
/// Finds integer coefficients x and y such that: ax + by = gcd(a, b)
/// Suppose the previous remainder is $r_p = ax_p + by_p$ and the current remainder is $r = ax + by$. We have:
///
/// $$
/// \begin{align}
/// &r_p = q + new r  \newline
/// &new \ r = r_p - qr = (ax_p + by_p) - q(ax + by) = a(x_p - x) + b(y_p - y) \newline
/// &new \ x = x_p - x,\ new \ y = y_p - y \newline
/// \end{align}
/// $$
///
/// # See also
/// - [Extended Euclidean Algorithm](http://anh.cs.luc.edu/331/notes/xgcd.pdf)
pub fn egcdi(_a: i32, _b: i32) -> (i32, i32, i32) {
    unimplemented!()
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(216, 111), 3);
    assert_eq!(gcd(216, -111), 3);
    assert_eq!(gcd(-216, -111), 3);

    assert_eq!(gcdi(15, 14), 1);
    assert_eq!(gcdi(216, 111), 3);
    assert_eq!(gcdi(216, -111), 3);
    assert_eq!(gcdi(-216, -111), 3);
}

#[test]
fn test_egcd() {
    let (gcd, x_p, y_p) = egcd(35, 15);

    assert_eq!(gcd, 5);
    assert_eq!(x_p, 1);
    assert_eq!(y_p, -2);

    let (gcd, x_p, y_p) = egcd(222, 97);
    assert_eq!(gcd, 1);
    assert_eq!(x_p, -45);
    assert_eq!(y_p, 103);
}
