# Affine Cipher

## Definition
### Formal definition

Let $x,y,a,b\in\mathbb{Z}_{26}$

Encryption: $e_k(x)=y\equiv ax+b \pmod{26}$

Decryption: $d_k(y)=x\equiv a^{-1}(y-b) \pmod{26}$

with the key: $k=(a,b)$, which has the restriction: $\gcd(a,26)=1$.

(Source: Definition 1.4.4, Understanding Cryptography: A Textbook for Students and Practitioners by by Christof Paar and Jan Pelzl)

### Explanation
An **affine cipher** is a type of substitution cipher defined over a set of $n$ characters (for the English alphabet, $n=26$). The encryption and decryption processes are based on linear functions modulo $n$.

- **Encryption function**:
  $$y \equiv \alpha x + \beta \mod n$$
  
- **Decryption function**:
  $$x \equiv \gamma y + \delta \mod n$$
  
Where:
- $x$ is the numerical value of the plaintext character (the index of the character in the alphabet).
- $y$ is the numerical value of the ciphertext character (the index of the character in the alphabet).
- $\alpha$ and $\beta$ are the encryption key coefficients.
- $\gamma$ and $\delta$ are the decryption key coefficients.
- $\gamma$ is the multiplicative inverse of $\alpha$ modulo $n$, meaning $\alpha \gamma \equiv 1 \mod n$.

### Proof

The encryption function is:

$$y \equiv \alpha x + \beta \mod n$$

**Subtract $\beta$ from both sides**:

$$y - \beta \equiv \alpha x \mod n$$

**Multiply both sides by modular inverse of $\alpha$**:

$$\alpha^{-1} (y - \beta) \equiv \alpha^{-1} \alpha x \equiv x \mod n$$

**Rearange and distribute: **

$$x \equiv \alpha^{-1} (y - \beta) \mod n \\
x \equiv \alpha^{-1}y - \alpha^{-1}\beta \mod n$$

** We have the decryption function: **

$$x \equiv \gamma y + \delta \mod n$$

Where:
- $\gamma = \alpha^{-1}$
- $\delta = -\gamma \beta$

### Example 

Encryption key $(\alpha, \beta) = (3, 7)$:

#### Step 1: Compute $\gamma$

Find $\gamma$ such that:

$$3 \gamma \equiv 1 \mod 26$$

Testing values, we find $\gamma = 9$ because $3 \times 9 = 27 \equiv 1 \mod 26$.

#### Step 2: Compute $\delta$

Using the formula $\delta \equiv -\gamma \beta \mod 26$:

$$\delta \equiv -9 \times 7 \mod 26$$

Compute $-9 \times 7 = -63$.

Now find $-63 \mod 26$:

- Since $26 \times 2 = 52$, and $52 - 63 = -11$, so $-63 \equiv -11 \mod 26$.
- Adding $26$ to get a positive value: $-11 + 26 = 15$.

So:

$$\delta \equiv 15 \mod 26$$

#### Step 3: Write the Decryption Function

The decryption function is:

$$x \equiv 9 y + 15 \mod 26$$

This matches the decryption formula provided in the text.

### General Formula

In general, for an affine cipher with encryption key $(\alpha, \beta)$ and decryption key $(\gamma, \delta)$:

- **Decryption key coefficients**:

  $$\begin{align*}
  \gamma &\equiv \alpha^{-1} \mod n \quad \text{(Multiplicative inverse of } \alpha) \\
  \delta &\equiv -\gamma \beta \mod n \\
  \end{align*}$$

- **Decryption function**:

  $$x \equiv \gamma y + \delta \mod n$$

### Summary

- **To find $\delta$**:

  $$\delta = (-\gamma \beta) \mod n$$

- **Given**:

  - $\alpha$: Encryption multiplier.
  - $\beta$: Encryption shift.
  - $\gamma$: Multiplicative inverse of $\alpha$ modulo $n$.

- **Steps**:

  1. Compute $\gamma$ such that $\alpha \gamma \equiv 1 \mod n$.
  2. Compute $\delta$ using $\delta \equiv -\gamma \beta \mod n$.
  
### Bibliography
Paar, Christof, and Jan Pelzl. Understanding Cryptography: A Textbook for Students and Practitioners. Springer, 2010.

Arizona State University. "Affine Cipher." Accessed October 22, 2024. https://math.asu.edu/sites/default/files/affine.pdf