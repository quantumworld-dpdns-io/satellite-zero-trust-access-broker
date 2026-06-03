use pqcrypto_kyber::kyber1024::*;

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
}
