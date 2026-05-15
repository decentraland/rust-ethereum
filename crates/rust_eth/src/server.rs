use std::sync::Mutex;

use alloy_primitives::B256;
use alloy_signer::{k256::ecdsa::SigningKey, SignerSync};
use alloy_signer_local::{LocalSigner, PrivateKeySigner};

#[derive(Default)]
pub struct SignServer {
    signer: Mutex<Option<LocalSigner<SigningKey>>>,
}

impl SignServer {
    /// Install a 32-byte private key as the active signer. Subsequent calls overwrite the
    /// previous key. Returns Err if the bytes are not a valid secp256k1 key or the lock is
    /// poisoned.
    pub fn setup(&self, private_key: &[u8]) -> Result<(), std::fmt::Error> {
        let fixed_bytes = B256::from_slice(private_key);
        let signer = PrivateKeySigner::from_bytes(&fixed_bytes).map_err(|_| std::fmt::Error)?;
        let mut guard = self.signer.lock().map_err(|_| std::fmt::Error)?;
        *guard = Some(signer);
        Ok(())
    }

    /// EIP-191 personal_sign of `message` under the currently installed private key.
    /// Returns the raw 65-byte signature (r||s||v).
    pub fn sign_message(&self, message: &str) -> Result<[u8; 65], std::fmt::Error> {
        let guard = self.signer.lock().map_err(|_| std::fmt::Error)?;
        let signer = guard.as_ref().ok_or(std::fmt::Error)?;
        let signature = signer
            .sign_message_sync(message.as_bytes())
            .map_err(|_| std::fmt::Error)?;
        Ok(signature.as_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sign() {
        let key_str = "64fdd126fe0e2de2ccbea065d710e9939d083ec96bb9933b750013f30ee81004";
        let message = "Test message";
        let required_signature = "578d0780163581456421895b03b79e038ab898d013450b3b58e20432fd89a0b54b86ae68348972fb8ca7544a88327e8628d32ba3e3a703b0e988f348ba26c3da1b";

        let server = SignServer::default();
        let vec_key = hex::decode(key_str).unwrap();
        server.setup(vec_key.as_slice()).unwrap();

        let signature = server.sign_message(message).unwrap();
        assert_eq!(hex::encode(signature), required_signature);
    }
}
