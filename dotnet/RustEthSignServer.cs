using System;

namespace DCL.RustEthereum
{
    /// <summary>
    /// Thin managed wrapper over the stateful native sign-server API. The underlying native
    /// state is a process-wide singleton — <see cref="Initialize"/> installs the active private
    /// key and subsequent calls overwrite it. Use this when a single signer is needed for the
    /// lifetime of the process. For multi-account scenarios prefer the stateless
    /// <see cref="RustEth"/> API.
    /// </summary>
    public static class RustEthSignServer
    {
        public const int SIGNATURE_SIZE = 65;

        /// <summary>Install <paramref name="privateKey"/> (must be exactly 32 bytes) as the active signer.</summary>
        /// <returns>True on success, false if the bytes are not a valid secp256k1 key.</returns>
        public static bool Initialize(byte[] privateKey)
        {
            if (privateKey is null || privateKey.Length != 32)
                throw new ArgumentException("private key must be 32 bytes", nameof(privateKey));

            unsafe
            {
                fixed (byte* ptr = privateKey)
                {
                    return RustEthNative.SignServerInitialize(ptr, (nuint)privateKey.Length);
                }
            }
        }

        /// <summary>EIP-191 personal_sign of <paramref name="message"/> under the currently installed key.</summary>
        /// <returns>Raw 65-byte signature (r||s||v).</returns>
        /// <remarks>Caller must have invoked <see cref="Initialize"/> successfully before calling.</remarks>
        public static byte[] Sign(string message)
        {
            var output = new byte[SIGNATURE_SIZE];
            unsafe
            {
                fixed (byte* ptr = output)
                {
                    RustEthNative.SignServerSignMessage(message, ptr);
                }
            }
            return output;
        }
    }
}
