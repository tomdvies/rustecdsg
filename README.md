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

### Example: Signature Generation and Verification

```rust
use rustecdsg::{ECPoint, FpElem, KeyChain, Signature};
use primitive_types::U512;

fn main() {
    // secp256k1 curve parameters
    let gx = U512::from("0x79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798");
    let gy = U512::from("0x483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8");
    let n = U512::from("0xfffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141");  // curve order
    let p = U512::from("0xfffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f");   // field modulus

    // Create the generator point G for secp256k1
    let g: ECPoint<FpElem<U512>> = ECPoint::new(
        gx,      // x coordinate
        gy,      // y coordinate
        U512::from(0),  // a parameter of curve
        U512::from(7),  // b parameter of curve
        p,       // prime field modulus
    );

    // 1. Key Generation
    // ----------------
    // Create a private key (normally this should be randomly generated)
    let private_key = FpElem::new(U512::from(123456789), n);
    
    // Create a KeyChain instance with the private key
    let keychain = KeyChain::new_priv(private_key, (g, n));

    // 2. Signing a Message
    // -------------------
    // The message hash to sign (normally this would be the SHA-256 hash of your message)
    let message_hash = FpElem::new(
        U512::from("0xbc62d4b80d9e36da29c16c5d4d9f11731f36052c72401a76c23c0fb5a9b74423"),
        n
    );

    // Sign the message hash
    let signature = keychain.sign(&message_hash).unwrap();

    // The signature consists of two values (r,s)
    println!("Signature r: {}", signature.targetx);  // x-coordinate of R
    println!("Signature s: {}", signature.sig);      // signature proof value

    // 3. Signature Verification
    // -----------------------
    // Verify the signature (this would normally be done by another party)
    let is_valid = keychain.verify_sig(&message_hash, &signature);
    
    println!("Signature valid: {}", is_valid);
    assert!(is_valid);

    // If you only have the public key, you can create a KeyChain for verification:
    let public_key = keychain.pub_key;
    let verifier = KeyChain::new_pub(public_key, (g, n));
    
    // Verify using only public information
    assert!(verifier.verify_sig(&message_hash, &signature));
}
```

## Security Note
<div style="background-color: #ffebee; padding: 16px; border-radius: 4px; border-left: 4px solid #ff0000;">
<h3 style="color: #ff0000; margin-top: 0;">⚠️ NOT FOR PRODUCTION USE!</h3>
<p style="color: #ff0000; font-size: 1.2em; font-weight: bold;">
This is an educational implementation to understand ECDSA. For production use, please use well-audited cryptographic libraries.
</p>
</div>
