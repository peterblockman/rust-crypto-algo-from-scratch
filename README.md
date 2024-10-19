# Rust Cryptography Algorithms from Scratch

Implementation of popular cryptographic algorithms from scratch in Rust. My goal is to provide a clear, educational resource for understanding cryptographic concepts and their implementation details. 

## Algorithms

These algorithms are planned to be implemented (this list may grow over time):

- Symmetric Encryption
  - AES (Advanced Encryption Standard)
  - DES (Data Encryption Standard)
  - Caesar Cipher
  - Affine Cipher
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

## Documentation
Run
```bash
RUSTDOCFLAGS="--html-in-header katex-header.html" cargo doc --no-deps --open
```

## Disclaimer

Please note that these implementations are for educational purposes only. They are not intended for use in production environments or security-critical applications. For real-world cryptographic needs, always use well-established and thoroughly audited cryptographic libraries. The documentations are written with the help of ChatGPT.
