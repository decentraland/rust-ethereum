//! Regression: `eth_verify_message` must not abort the host process when the
//! `message` argument contains non-UTF-8 bytes.
//!
//! Historically the implementation called `CStr::to_str().unwrap()`, which
//! panics on invalid UTF-8. Across an `extern "C"` boundary that panic becomes
//! `panic-cannot-unwind` → `SIGABRT`, killing the host. The fix is to return
//! `false` on decode failure instead.
//!
//! Lives in its own integration-test binary so a regression's `SIGABRT` only
//! fails this one test rather than poisoning sibling tests in the same process.

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
fn non_utf8_message_does_not_abort_process() {
    let signer = test_signer();
    let address = format!("{:?}", signer.address());
    let address_c = CString::new(address).unwrap();
    let sig = signer.sign_message_sync(b"Test message").unwrap().as_bytes();

    // 0xFF is never a valid UTF-8 start byte; `CString` accepts it because it
    // only rejects interior NULs.
    let bad_msg = CString::new(vec![0xFFu8, 0xFE, 0xFD]).unwrap();

    let result = unsafe {
        rust_eth::cabi::eth_verify_message(address_c.as_ptr(), bad_msg.as_ptr(), sig.as_ptr())
    };

    assert!(!result, "non-UTF-8 message must not verify");
}
