# Rust Cryptography Algorithms from Scratch

**Disclaimer**  
*Please note that these implementations are for educational purposes only. They are not intended for use in production environments or security-critical applications. For real-world cryptographic needs, always use well-established and thoroughly audited cryptographic libraries.*

---

Implementation of popular cryptographic algorithms from scratch in Rust. The goal is to provide a clear, educational resource for understanding cryptographic concepts and their implementation details. 

## Algorithms

These algorithms are planned to be implemented (this list may grow over time):

- Math
  - Modular Arithmetic
    - [GCD (Greatest Common Divisor)](src/math/modular_arithmetic/)
    - [Modular Inverse](src/math/modular_arithmetic/)
- Random Number Generators
  - [LCG (Linear Congruential Generator)](src/rng/)
- Symmetric Encryption
  - [Caesar Cipher](src/symmetric_encryption/caesar)
  - [Affine Cipher](src/symmetric_encryption/affine)
  - [Trivium](src/symmetric_encryption/trivium)
  - [DES (Data Encryption Standard)](src/symmetric_encryption/des)
  - AES (Advanced Encryption Standard)
- Asymmetric Encryption
  - RSA (Rivest–Shamir–Adleman)
  - ECC (Elliptic Curve Cryptography)
- Hash Functions
  - SHA-256 (Secure Hash Algorithm 2)
  - MD5 (Message Digest algorithm 5)
  - Digital Signatures
  - DSA (Digital Signature Algorithm)
- Key Exchange
  - Diffie-Hellman
