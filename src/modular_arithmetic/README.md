# Recursive Extended Euclidean Algorithm

## Overview

The Extended Euclidean Algorithm not only computes the greatest common divisor (GCD) of two integers $a$ and $b$ but also finds integers $x$ and $y$ satisfying:

```math
gcd(a, b) = a \times x + b \times y
```

## The Algorithm Steps

1. **Base Case**: 

    If $b = 0$, the GCD is $a$, and the coefficients are $x = 1$ and $y = 0$:

    ```math
    \gcd(a, 0) = a = a \times 1 + 0 \times 0
    ```

2. **Recursive Case**:
    
    If $b \neq 0$, we recursively call the function with $(b, a \bmod b)$:

```math
\gcd(a, b) = \gcd(b, a \bmod b)
```

## Deriving the Update Formulas

Let's derive the update formulas for $x$ and $y$ based on the recursive call.

### Step 1: Recursive Call

We start with the recursive call:

```math
\gcd(b, a \bmod b) = b \times x + (a \bmod b) \times y
```

This gives us:

1. $\gcd(b, a \bmod b) = d$
2. Coefficients $x$ and $y$ such that:

```math
d = b \times x + (a \bmod b) \times y
```

### Step 2: Express $a \bmod b$ in Terms of $a$ and $b$

Recall that:

```math
a \bmod b = a - \left\lfloor \frac{a}{b} \right\rfloor \times b
```

Let $q = \left\lfloor \frac{a}{b} \right\rfloor$, so:

```math
a \bmod b = a - q \times b
```

### Step 3: Substitute Back into the GCD Equation

Substitute $a \bmod b$ into the equation:

```math
d = b \times x + (a - q \times b) \times y
```

Simplify:

```math
d = b \times x + a \times y - q \times b \times y
```

Group like terms:

```math
d = a \times y + b \times (x - q \times y)
```

### Step 4: Identify New Coefficients $x$ and $y$

From the rearranged equation, we can identify:

- Coefficient of $a$: $y$
- Coefficient of $b$: $x - q \times y$

Thus, we can set:

```math
x = y \newline
y = x - q \times y
```

This ensures:

```math
d = a \times x + b \times y
```

# Iterative Extended Euclidean Algorithm

If $b = 0$, the GCD is $a$, and the coefficients are $x = 1$ and $y = 0$:

```math
\gcd(a, 0) = a = a \times 1 + 0 \times 0
```

Therefore, we initialize:
```rust
    // first remainder
    let x0 = 1;
    let y0 = 0;

    // second remainder
    let x1 = 0;
    let y1 = 1;

    // set initial values
    let mut x_p = x0;
    let mut x = x1;
    let mut y_p = y0;
    let mut y = y1;

```

# Modular Inverse

The modular inverse of $a$ modulo $m$ is the integer $x$ such that:

```math
a \times x \equiv 1 \pmod{m}
```

