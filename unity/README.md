# Rust Ethereum (Unity package)

Rust-backed Ethereum signing and verification for Decentraland.

This package contains the managed C# bindings under `Runtime/` and platform-specific
native libraries under `Runtime/Plugins/` (added at release time by CI; not present on
`main`).

## Install

In your project's `Packages/manifest.json`:

```json
"com.decentraland.rust-ethereum": "https://github.com/decentraland/rust-ethereum.git?path=unity#v0.2.0"
```

Pin the `#vX.Y.Z` tag — Unity will resolve to the matching commit, which carries the
native binaries with platform-specific `.meta` settings.

## Public API

```csharp
using DCL.RustEthereum;

// Stateless API (multi-account safe)
string  addr = RustEth.AddressFromPrivateKey(privateKey32Bytes);
string  sig  = RustEth.SignMessage(privateKey32Bytes, "hello");
bool    ok   = RustEth.Verify(addr, "hello", sig);

// Stateful API (single active signer per process)
RustEthSignServer.Initialize(privateKey32Bytes);
byte[]  raw  = RustEthSignServer.Sign("hello");
```
