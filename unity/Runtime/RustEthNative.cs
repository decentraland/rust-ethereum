using System;
using System.Runtime.InteropServices;

namespace DCL.RustEthereum;

public static class RustEthNative
{
    private const string LIBRARY_NAME = "rust_eth";

    // -------- Stateless API --------

    /// <param name="expectedSignerAddress">The eth address of the signer</param>
    /// <param name="message">The original message that was signed</param>
    /// <param name="signature">The 65-byte signature to verify</param>
    /// <returns>True if the recovered signer matches the expected address</returns>
    [DllImport(LIBRARY_NAME, CallingConvention = CallingConvention.Cdecl, EntryPoint = "eth_verify_message")]
    internal static extern unsafe bool EthVerifyMessage(string expectedSignerAddress, string message, byte* signature);

    /// <param name="privateKey">Pointer to a 32-byte private key</param>
    /// <param name="outBuffer">Caller-allocated buffer that receives the lowercase 0x-prefixed address (42 bytes)</param>
    /// <param name="outCapacity">Capacity of <paramref name="outBuffer"/> in bytes</param>
    /// <returns>Number of bytes written, or 0 on failure</returns>
    [DllImport(LIBRARY_NAME, CallingConvention = CallingConvention.Cdecl, EntryPoint = "eth_address_from_private_key")]
    internal static extern unsafe nuint EthAddressFromPrivateKey(byte* privateKey, byte* outBuffer, nuint outCapacity);

    /// <param name="privateKey">Pointer to a 32-byte private key</param>
    /// <param name="message">Null-terminated UTF-8 message bytes</param>
    /// <param name="outBuffer">Caller-allocated buffer that receives the 0x-prefixed 130-char hex signature (132 bytes)</param>
    /// <param name="outCapacity">Capacity of <paramref name="outBuffer"/> in bytes</param>
    /// <returns>Number of bytes written, or 0 on failure</returns>
    [DllImport(LIBRARY_NAME, CallingConvention = CallingConvention.Cdecl, EntryPoint = "eth_sign_message")]
    internal static extern unsafe nuint EthSignMessage(byte* privateKey, string message, byte* outBuffer, nuint outCapacity);

    // -------- Stateful API --------

    /// <param name="privateKey">Pointer to a 32-byte private key</param>
    /// <param name="len">Length of the buffer pointed to by <paramref name="privateKey"/> (must be 32)</param>
    [DllImport(LIBRARY_NAME, CallingConvention = CallingConvention.Cdecl, EntryPoint = "sign_server_initialize")]
    internal static extern unsafe bool SignServerInitialize(byte* privateKey, nuint len);

    /// <param name="message">Null-terminated UTF-8 message to sign</param>
    /// <param name="signatureOutput">Pointer to a writable 65-byte buffer that receives r||s||v</param>
    [DllImport(LIBRARY_NAME, CallingConvention = CallingConvention.Cdecl, EntryPoint = "sign_server_sign_message")]
    internal static extern unsafe void SignServerSignMessage(string message, byte* signatureOutput);
}
