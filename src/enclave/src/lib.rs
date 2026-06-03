use pqcrypto_kyber::kyber1024::*;
use pqcrypto_dilithium::dilithium5::*;
use pqcrypto_traits::sign::{DetachedSignature as _, PublicKey as SignPublicKey, SecretKey as SignSecretKey};

/// Simulates the generation of a post-quantum keypair for a satellite connection.
pub fn generate_pqc_keypair() -> (PublicKey, SecretKey) {
    keypair()
}

/// Simulates the encapsulation of a shared secret using a given public key.
pub fn encapsulate_secret(pk: &PublicKey) -> (SharedSecret, Ciphertext) {
    encapsulate(pk)
}

/// Simulates the decapsulation of a ciphertext to retrieve the shared secret.
pub fn decapsulate_secret(ct: &Ciphertext, sk: &SecretKey) -> SharedSecret {
    decapsulate(ct, sk)
}

/// Generates a Post-Quantum Signature keypair using Dilithium5.
pub fn generate_signature_keypair() -> (pqcrypto_dilithium::dilithium5::PublicKey, pqcrypto_dilithium::dilithium5::SecretKey) {
    pqcrypto_dilithium::dilithium5::keypair()
}

/// Signs a message using Dilithium5.
pub fn sign_message(msg: &[u8], sk: &pqcrypto_dilithium::dilithium5::SecretKey) -> pqcrypto_dilithium::dilithium5::DetachedSignature {
    detached_sign(msg, sk)
}

/// Verifies a Dilithium5 signature.
pub fn verify_signature(sig: &pqcrypto_dilithium::dilithium5::DetachedSignature, msg: &[u8], pk: &pqcrypto_dilithium::dilithium5::PublicKey) -> bool {
    verify_detached_signature(sig, msg, pk).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pqcrypto_traits::kem::SharedSecret as _;

    #[test]
    fn test_kyber_kem() {
        let (pk, sk) = generate_pqc_keypair();
        let (shared_secret_sender, ciphertext) = encapsulate_secret(&pk);
        let shared_secret_receiver = decapsulate_secret(&ciphertext, &sk);

        assert_eq!(shared_secret_sender.as_bytes(), shared_secret_receiver.as_bytes());
    }

    #[test]
    fn test_dilithium_signatures() {
        let (pk, sk) = generate_signature_keypair();
        let message = b"authorize_satellite_command_0x4A";
        let signature = sign_message(message, &sk);
        
        assert!(verify_signature(&signature, message, &pk));
        
        let bad_message = b"authorize_satellite_command_0x4B";
        assert!(!verify_signature(&signature, bad_message, &pk));
    }
}
