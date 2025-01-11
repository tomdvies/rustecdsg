use rustecdsg::FpElem;
extern crate primitive_types;
use primitive_types::U512;
use rustecdsg::{ECPoint, KeyChain, Signature};

#[test]
fn signature_verify() {
    let z = U512::from("0xbc62d4b80d9e36da29c16c5d4d9f11731f36052c72401a76c23c0fb5a9b74423");
    let r = U512::from("0x37206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c6");
    let s = U512::from("0x8ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaec");
    let px = U512::from("0x04519fac3d910ca7e7138f7013706f619fa8f033e6ec6e09370ea38cee6a7574");
    let py = U512::from("0x82b51eab8c27c66e26c858a079bcdf4f1ada34cec420cafc7eac1a42216fb6c4");

    let gx = U512::from("0x79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798");
    let gy = U512::from("0x483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8");
    let n = U512::from("0xfffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141");

    let p = U512::from("0xfffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f");

    // secp256k1 generating point
    let g: ECPoint<FpElem<U512>> = ECPoint::new(gx, gy, U512::from(0), U512::from(7), p);

    // Create the public key point P
    let pubkeypoint = ECPoint::new(px, py, U512::from(0), U512::from(7), p);

    // Create FpElem instances for z, r, s with modulus n
    let z = FpElem::new(z, n);
    let r = FpElem::new(r, n);
    let s = FpElem::new(s, n);

    // Create signature
    let sig = Signature::new(r, s);

    // Create keychain with public key
    let keychain = KeyChain::new_pub(pubkeypoint, (g, n));

    let is_valid = keychain.verify_sig(&z, &sig);
    println!("Signature verification result: {}", is_valid);
    // Verify the signature
    assert!(is_valid);
}


#[test]
fn sign_hash() {
    // secp256k1 parameters
    let gx = U512::from("0x79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798");
    let gy = U512::from("0x483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8");
    let n = U512::from("0xfffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141");
    let p = U512::from("0xfffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f");

    // Create generator point
    let g: ECPoint<FpElem<U512>> = ECPoint::new(gx, gy, U512::from(0), U512::from(7), p);
    
    // Create private key and KeyChain
    let priv_key = FpElem::new(U512::from(123456789), n); // Example private key
    let keychain = KeyChain::new_priv(priv_key, (g, n));
    
    // Message hash to sign
    let z = FpElem::new(U512::from("0xbc62d4b80d9e36da29c16c5d4d9f11731f36052c72401a76c23c0fb5a9b74423"), n);
    
    // Sign the hash
    let sig = keychain.sign(&z).unwrap();
    
    // Verify the signature with the public key
    let is_valid = keychain.verify_sig(&z, &sig);
    println!("Generated signature verification result: {}", is_valid);
    println!("Signature r: {}", sig.targetx);
    println!("Signature s: {}", sig.sig);
    println!("Hash: {}", z);
    
    assert!(is_valid);
}
