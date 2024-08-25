# Elliptic Curve Cryptography (ECC) Journey

## Introduction

Over the past few months, I have embarked on a journey to understand elliptic curve cryptography (ECC). This README documents the mathematical principles I've learned, the elliptic curve I worked on, and the significance of these concepts in cryptographic applications. My goal is to explain the fundamental concepts in a practical context, focusing on the essential parts of ECC.

## The Elliptic Curve

The elliptic curve I explored is defined by the equation:

$E : Y^2 = X^3 + 497X + 1768 \mod 9739$

This equation defines an elliptic curve over the finite field $\mathbb{F}_{17}$, where 17 is a prime number. By iterating through all possible values of $x$ and solving for $y$, I calculated the valid points on this curve.

### Points on the Curve

The points on the curve $E$ are pairs $(x, y)$ that satisfy the equation. These points are crucial in ECC, as they represent the possible values used in cryptographic operations.

## Finite Field Arithmetic

To fully understand elliptic curves over finite fields, I studied finite field arithmetic, which involves operations within a finite set of numbers. In $\mathbb{F}_{17}$, all operations (addition, multiplication, etc.) are performed modulo 17.

### Key Operations

- **Addition:** For two elements $a$ and $b$ in $\mathbb{F}_{p}$, $(a + b) \mod p$.
- **Multiplication:** $(a \times b) \mod p$.
- **Negation:** The additive inverse $-a$ is such that $a + (-a) \equiv 0 \mod p$.
- **Square Root:** Solving $y^2 \equiv x \mod p$ for $y$.
- **Multiplicative Inverse:** For $a$ in $\mathbb{F}_{p}$, find $b$ such that $(a \times b) \equiv 1 \mod p$.

These operations are essential for implementing elliptic curve addition and multiplication.

## Elliptic Curve Point Operations

One of the most fascinating aspects of ECC is the point addition operation. Given two points $P$ and $Q$ on the curve, we can compute $R = P + Q$. This operation is defined geometrically and algebraically and forms the basis of elliptic curve cryptography.

### Point Addition and Doubling

- **Point Addition:** Given two distinct points $P(x_1, y_1)$ and $Q(x_2, y_2)$, the sum $R(x_3, y_3)$ is calculated using the slope between them.
- **Point Doubling:** If $P = Q$, the slope is calculated differently to account for the tangent at $P$.

These operations allow us to compute multiples of a point, which is the foundation of generating public and private keys in ECC.

## The Generator Point

In ECC, a generator point $G$ is used to produce other points on the curve through repeated addition. For a given private key $k$, the corresponding public key is $kG$. The security of ECC relies on the difficulty of solving the discrete logarithm problem, i.e., finding $k$ given $kG$ and $G$.

## Algebraic Structures and the Discrete Logarithm Problem

Understanding algebraic structures such as groups, rings, and fields has been essential in grasping how elliptic curves work. The discrete logarithm problem is particularly important in ECC because it underpins the security of the cryptographic system. The difficulty of reversing the point multiplication operation (finding $k$) ensures the security of keys generated using ECC.

## Importance of Learning Finite Field Math

Learning finite field math and the properties of elliptic curves has given me a solid foundation for understanding more advanced cryptographic systems. The study of orders, or the number of points on a curve, has also been crucial. Orders play a significant role in ensuring the security of the elliptic curve used.

## Conclusion

This project has been an enlightening exploration into the world of elliptic curve cryptography. I have gained a deeper understanding of finite field arithmetic, point operations, and the discrete logarithm problem. ECC is a powerful and practical tool in modern cryptography, and my work on this simple curve has been a foundational step in understanding how cryptographic keys are securely generated and managed.

I hope this documentation provides a clear and concise overview of the principles I have learned, and serves as a useful resource for anyone interested in elliptic curve cryptography.