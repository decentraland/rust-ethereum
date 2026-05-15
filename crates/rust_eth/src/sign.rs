use alloy_primitives::B256;
use alloy_signer::SignerSync;
use alloy_signer_local::PrivateKeySigner;

/// Derive the lowercase 0x-prefixed Ethereum address for a 32-byte private key.
pub fn address_from_private_key(private_key: &[u8; 32]) -> Result<String, std::fmt::Error> {
    let key = B256::from_slice(private_key);
    let signer = PrivateKeySigner::from_bytes(&key).map_err(|_| std::fmt::Error)?;
    Ok(format!("{:?}", signer.address()))
}

/// EIP-191 personal_sign of `message` under the 32-byte private key. Returns a
/// 0x-prefixed 65-byte hex signature (r||s||v), the same shape the verify path consumes.
pub fn sign_message(private_key: &[u8; 32], message: &str) -> Result<String, std::fmt::Error> {
    let key = B256::from_slice(private_key);
    let signer = PrivateKeySigner::from_bytes(&key).map_err(|_| std::fmt::Error)?;
    let signature = signer
        .sign_message_sync(message.as_bytes())
        .map_err(|_| std::fmt::Error)?;
    Ok(format!("0x{}", hex::encode(signature.as_bytes())))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::verify::verify_message;

    #[test]
    fn sign_then_verify_roundtrip() {
        let key = [0x42u8; 32];
        let address = address_from_private_key(&key).unwrap();
        let sig_hex = sign_message(&key, "Test message").unwrap();

        let sig_bytes_vec = hex::decode(sig_hex.trim_start_matches("0x")).unwrap();
        let sig_bytes: [u8; 65] = sig_bytes_vec.try_into().unwrap();

        assert!(verify_message(&address, "Test message", &sig_bytes).unwrap());
    }
}
