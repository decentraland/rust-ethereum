//! Regression: same `unwrap()`-on-`to_str()` defect as
//! `cabi_non_utf8_message`, this time via the `expected_signer_address`
//! argument. Must return `false` on invalid UTF-8 rather than aborting.

use std::ffi::CString;

use alloy_primitives::B256;
use alloy_signer::SignerSync;
use alloy_signer_local::PrivateKeySigner;

fn test_signer() -> PrivateKeySigner {
    let key_str = "64fdd126fe0e2de2ccbea065d710e9939d083ec96bb9933b750013f30ee81004";
    let fixed_bytes = B256::from_slice(&hex::decode(key_str).unwrap());
    PrivateKeySigner::from_bytes(&fixed_bytes).unwrap()
}

#[test]
fn non_utf8_address_does_not_abort_process() {
    let signer = test_signer();
    let sig = signer.sign_message_sync(b"Test message").unwrap().as_bytes();

    let msg_c = CString::new("Test message").unwrap();
    // 0xC3 0x28 is the canonical invalid-UTF-8 pair (valid leading byte,
    // invalid continuation).
    let bad_addr = CString::new(vec![0xC3u8, 0x28]).unwrap();

    let result = unsafe {
        rust_eth::cabi::eth_verify_message(bad_addr.as_ptr(), msg_c.as_ptr(), sig.as_ptr())
    };

    assert!(!result, "garbage address must not verify");
}
