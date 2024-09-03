# ECDSA Signature Verification using secp256k1 in Rust

## Overview

This project implements the Elliptic Curve Digital Signature Algorithm (ECDSA) to verify signatures, ensuring that a message was indeed signed by the claimed signer. The implementation utilizes the Bitcoin `secp256k1` elliptic curve, renowned for its efficiency and security in cryptocurrency applications.



## Elliptic Curve Cryptography (ECC) and ECDSA

Elliptic Curve Cryptography (ECC) is a public key cryptography approach based on the algebraic structure of elliptic curves over finite fields. ECC offers the same level of security as traditional systems like RSA but with smaller key sizes, leading to faster computations and reduced storage requirements.

The Elliptic Curve Digital Signature Algorithm (ECDSA) is a variant of the Digital Signature Algorithm (DSA) that leverages ECC. ECDSA is widely used for securing digital communications, including in cryptocurrencies like Bitcoin, due to its robustness and efficiency.

## secp256k1 Curve Parameters

The `secp256k1` curve is a Koblitz curve defined over the finite field $\( \mathbb{F}_p \)$. Below are its domain parameters:

- The curve E is defined as $y^2 = x^3 + ax + b$ over Fp, where:
- **a**:
  ```
  a = 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000
  ```
- **b**:
  ```
  b = 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000007
  ```

- The base point **G** (generator point) is:
- Compressed form:
  ```
  G = 02 79BE667E F9DCBBAC 55A06295 CE870B07 029BFCDB 2DCE28D9 59F2815B 16F81798
  ```
- Uncompressed form:
  ```
  G = 04 79BE667E F9DCBBAC 55A06295 CE870B07 029BFCDB 2DCE28D9 59F2815B 16F81798
      483ADA77 26A3C465 5DA4FBFC 0E1108A8 FD17B448 A6855419 9C47D08F FB10D4B8
  ```

- **n** (order of G):
  ```
  n = FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFE BAAEDCE6 AF48A03B BFD25E8C D0364141
  ```

### Finite Field $\( \mathbb{F}_p \)$
```
p = {0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F}
   = 2^{256} - 2^{32} - 2^{9} - 2^{8} - 2^{7} - 2^{6} - 2^{4} - 1
```

### Elliptic Curve Equation

The curve \( E \) is defined by the equation:

$\[
E: y^2 = x^3 + ax + b \quad \text{over} \quad \mathbb{F}_p
\]$

where:

- $\( a = 0 \)$
- \$( b = 7 \)$

Represented in hexadecimal:

```plaintext
a = 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000
b = 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000007
```



## Key Points of secp256k1

- `secp256k1` is a Koblitz curve, which is efficient for elliptic curve cryptography.
- The `p` value defines the prime field over which the elliptic curve is defined.
- The base point `G` is a well-known starting point for key generation.
- The order `n` of `G` ensures that operations on the elliptic curve eventually repeat after `n` points.

## How It Works

1. **Private and Public Keys**: 
 - The signer has a private key, and the public key is derived from the base point `G` and the private key.
 
2. **Message Signing**: 
 - The message is hashed, and a signature is generated using the private key.

3. **Signature Verification**: 
 - The verifier uses the signer's public key, the message, and the signature to check if the signature is valid and the signer is who they claim to be.

## Project Structure

The implementation is written in Rust using the `secp256k1` curve to handle elliptic curve operations. Below is an outline of the key functionality:

- **Key generation**: Generates a private key and derives the corresponding public key.
- **Signing**: Signs a message using the private key.
- **Verification**: Verifies a signature using the public key and original message.

## Usage

To run the project:

1. Install Rust from [here](https://github.com/mvk25/elliptic_curves.git).
2. Clone the repository.
3. Run the project:
 ```bash
 cd ecdsa
 cargo run
 ```