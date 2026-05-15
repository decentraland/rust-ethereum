use std::{
    ffi::{c_char, CStr},
    ptr,
};

// ---------- Stateless API ----------

/// # Safety
///
/// The foreign language must provide valid pointers: `expected_signer_address` and `message`
/// as null-terminated C strings, and `signature` as a pointer to 65 bytes.
#[no_mangle]
pub unsafe extern "C" fn eth_verify_message(
    expected_signer_address: *const c_char,
    message: *const c_char,
    signature: *const u8,
) -> bool {
    if expected_signer_address.is_null() || message.is_null() || signature.is_null() {
        return false;
    }
    let Ok(address_str) = CStr::from_ptr(expected_signer_address).to_str() else {
        return false;
    };
    let Ok(string_message) = CStr::from_ptr(message).to_str() else {
        return false;
    };
    let sig_bytes: &[u8; 65] = &*(signature as *const [u8; 65]);
    crate::verify::verify_message(address_str, string_message, sig_bytes).unwrap_or(false)
}

/// Write `bytes` into `out` up to `out_capacity`. Returns the number of bytes written
/// on success or 0 on overflow / failure.
unsafe fn write_to_buffer(bytes: &[u8], out: *mut u8, out_capacity: usize) -> usize {
    if out.is_null() || bytes.len() > out_capacity {
        return 0;
    }
    std::ptr::copy_nonoverlapping(bytes.as_ptr(), out, bytes.len());
    bytes.len()
}

/// # Safety
///
/// `private_key` must point to 32 bytes. `out` must point to a writable buffer of
/// `out_capacity` bytes (>= 42 to hold "0x" + 40 hex chars). Returns the number of
/// bytes written, or 0 on failure (bad key, buffer too small, null out).
#[no_mangle]
pub unsafe extern "C" fn eth_address_from_private_key(
    private_key: *const u8,
    out: *mut u8,
    out_capacity: usize,
) -> usize {
    if private_key.is_null() {
        return 0;
    }
    let key_bytes: &[u8; 32] = &*(private_key as *const [u8; 32]);
    let Ok(address) = crate::sign::address_from_private_key(key_bytes) else {
        return 0;
    };
    write_to_buffer(address.as_bytes(), out, out_capacity)
}

/// # Safety
///
/// `private_key` must point to 32 bytes. `message` must be a null-terminated UTF-8 C
/// string. `out` must point to a writable buffer of `out_capacity` bytes (>= 132 to hold
/// "0x" + 130 hex chars). Returns the number of bytes written, or 0 on failure.
#[no_mangle]
pub unsafe extern "C" fn eth_sign_message(
    private_key: *const u8,
    message: *const c_char,
    out: *mut u8,
    out_capacity: usize,
) -> usize {
    if private_key.is_null() || message.is_null() {
        return 0;
    }
    let Ok(message_str) = CStr::from_ptr(message).to_str() else {
        return 0;
    };
    let key_bytes: &[u8; 32] = &*(private_key as *const [u8; 32]);
    let Ok(signature) = crate::sign::sign_message(key_bytes, message_str) else {
        return 0;
    };
    write_to_buffer(signature.as_bytes(), out, out_capacity)
}

// ---------- Stateful API ----------

/// # Safety
///
/// `data` must point to `len` readable bytes (a 32-byte private key).
#[no_mangle]
pub unsafe extern "C" fn sign_server_initialize(data: *const u8, len: usize) -> bool {
    let data = std::slice::from_raw_parts(data, len);
    crate::sign_server().setup(data).is_ok()
}

/// # Safety
///
/// `message` must be a null-terminated UTF-8 C string. `res_ptr` must point to a writable
/// buffer of at least 65 bytes. Caller must have invoked `sign_server_initialize`
/// successfully before this call.
#[no_mangle]
pub unsafe extern "C" fn sign_server_sign_message(message: *const c_char, res_ptr: *mut *const u8) {
    let string_message = CStr::from_ptr(message).to_str().unwrap();
    let signature = crate::sign_server().sign_message(string_message).unwrap();
    ptr::copy(signature.as_ptr(), res_ptr as *mut u8, signature.len());
}
