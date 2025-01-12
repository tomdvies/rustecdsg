# rustecdsg
A from-scratch implementation of Elliptic Curve Digital Signature Algorithm (ECDSA) in Rust. This project provides a pure Rust implementation of elliptic curve cryptography primitives and digital signatures.

## Features

- Pure Rust implementation of ECDSA
- Generic implementation supporting various integer sizes - see the `GenericUInt` trait
- Finite field arithmetic operations
- Elliptic curve point operations
- Key generation and management
- Signature creation and verification

## Components

- `FpElem`: Finite field element implementation
- `ECPoint`: Elliptic curve point operations
- `KeyChain`: Key management and signature operations

## Usage

The library provides cryptographic primitives for:
- Key pair generation
- Message signing
- Signature verification

Example code will be added soon.

## Security Note
<div style="background-color: #ffebee; padding: 16px; border-radius: 4px; border-left: 4px solid #ff0000;">
<h3 style="color: #ff0000; margin-top: 0;">⚠️ NOT FOR PRODUCTION USE!</h3>
<p style="color: #ff0000; font-size: 1.2em; font-weight: bold;">
This is an educational implementation to understand ECDSA. For production use, please use well-audited cryptographic libraries.
</p>
</div>
