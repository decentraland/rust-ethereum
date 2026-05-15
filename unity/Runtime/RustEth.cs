using System;
using System.Text;

namespace DCL.RustEthereum
{
    public static class RustEth
    {
        private const int ADDRESS_BUFFER_SIZE = 64;   // "0x" + 40 hex chars, padded
        private const int SIGNATURE_BUFFER_SIZE = 144; // "0x" + 130 hex chars, padded

        public static bool Verify(string expectedSignerAddress, string message, string signature)
        {
            string hex = signature.StartsWith("0x", StringComparison.OrdinalIgnoreCase)
                ? signature.Substring(2)
                : signature;

            byte[] sigBytes = HexToBytes(hex);

            unsafe
            {
                fixed (byte* ptr = sigBytes)
                {
                    return RustEthNative.EthVerifyMessage(expectedSignerAddress, message, ptr);
                }
            }
        }

        /// <summary>Derive the lowercase 0x-prefixed Ethereum address for a 32-byte private key.</summary>
        public static string AddressFromPrivateKey(byte[] privateKey)
        {
            if (privateKey is null || privateKey.Length != 32)
                throw new ArgumentException("private key must be 32 bytes", nameof(privateKey));

            unsafe
            {
                byte* outBuffer = stackalloc byte[ADDRESS_BUFFER_SIZE];
                fixed (byte* keyPtr = privateKey)
                {
                    nuint written = RustEthNative.EthAddressFromPrivateKey(keyPtr, outBuffer, (nuint)ADDRESS_BUFFER_SIZE);
                    if (written == 0)
                        throw new InvalidOperationException("rust-eth: address derivation failed");
                    return Encoding.UTF8.GetString(outBuffer, (int)written);
                }
            }
        }

        /// <summary>EIP-191 personal_sign of <paramref name="message"/> under the 32-byte private key.</summary>
        /// <returns>0x-prefixed 130-char hex signature (r||s||v).</returns>
        public static string SignMessage(byte[] privateKey, string message)
        {
            if (privateKey is null || privateKey.Length != 32)
                throw new ArgumentException("private key must be 32 bytes", nameof(privateKey));

            unsafe
            {
                byte* outBuffer = stackalloc byte[SIGNATURE_BUFFER_SIZE];
                fixed (byte* keyPtr = privateKey)
                {
                    nuint written = RustEthNative.EthSignMessage(keyPtr, message, outBuffer, (nuint)SIGNATURE_BUFFER_SIZE);
                    if (written == 0)
                        throw new InvalidOperationException("rust-eth: signing failed");
                    return Encoding.UTF8.GetString(outBuffer, (int)written);
                }
            }
        }

        private static byte[] HexToBytes(string hex)
        {
            var bytes = new byte[hex.Length / 2];
            for (var i = 0; i < bytes.Length; i++)
                bytes[i] = Convert.ToByte(hex.Substring(i * 2, 2), 16);
            return bytes;
        }
    }
}
