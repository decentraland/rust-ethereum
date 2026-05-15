# rust-ethereum

Rust-backed Ethereum signing and verification for Decentraland.

A single `cdylib` crate (`rust_eth`) exposing two FFI surfaces:

- **Stateless** (`eth_*`): caller supplies the private key on every call — safe for
  multi-account scenarios.
- **Stateful** (`sign_server_*`): a process-wide singleton holds the active signer —
  matches the legacy sign-server API for callers that prefer it.

Managed bindings are shipped as:

- A **.NET class library** (`dotnet/DCL.RustEthereum.csproj`) — see [Consuming from .NET](#consuming-from-net).
- A **Unity UPM package** (`unity/`) — see [Consuming from Unity](#consuming-from-unity).

## Repository layout

```
crates/rust_eth/           # the Rust crate (lib + cdylib)
dotnet/                    # .NET 10 class library (no committed binaries)
unity/                     # UPM package source + static .meta files
.github/workflows/         # ci.yml + release.yml
```

`dotnet/*.cs` and `unity/Runtime/*.cs` are byte-identical and CI enforces it (`ci.yml`
parity job). When changing one, change the other.

## Building locally

```bash
# Rust crate + tests
cargo test --release

# .NET class library (produces dotnet/bin/Release/net10.0/DCL.RustEthereum.dll)
dotnet build dotnet/DCL.RustEthereum.csproj -c Release
```

The native binary lands at `target/release/rust_eth.{dll,so,dylib}` (with `lib` prefix on
Linux/macOS). Drop it next to `DCL.RustEthereum.dll` for a local smoke test.

## Releasing

Tag-driven. Push `vX.Y.Z` (or run the `Release` workflow manually with that version) and CI:

1. Builds `rust_eth` for `win-x64`, `linux-x64`, `osx-x64`, `osx-arm64`.
2. Produces a macOS universal `librust_eth.dylib` (`llvm-lipo`) for the Unity package.
3. Assembles a `rust-eth-natives-vX.Y.Z.zip` containing both a flat `native/{windows,linux,macos}/` layout and a NuGet-style `runtimes/<rid>/native/` layout.
4. Drops the per-platform Unity binaries into `unity/Runtime/Plugins/{Windows,Linux,macOS}/`, bumps `unity/package.json` to `X.Y.Z`, commits, and force-points the tag at the release commit. `main` stays free of native binaries — they exist only on the tag.
5. Publishes a GitHub Release with the .zip, individual native libs, and checksums.

Before tagging: bump `crates/rust_eth/Cargo.toml` `version` to `X.Y.Z`.

## Consuming from .NET

`.NET` consumers fetch native binaries from the GitHub Release for the pinned version. No
NuGet feed (yet).

```xml
<!-- consumer.csproj -->
<ItemGroup>
  <ProjectReference Include="path/to/DCL.RustEthereum.csproj" />
  <None Include="native/$(NativeFolder)/$(NativeLib)">
    <CopyToOutputDirectory>PreserveNewest</CopyToOutputDirectory>
    <TargetPath>$(NativeLib)</TargetPath>
  </None>
</ItemGroup>
```

Recommended fetch flow (one of):

- **Download script committed to the consumer**: a small `scripts/fetch-rust-eth.ps1` /
  `.sh` that pulls `rust-eth-natives-vX.Y.Z.zip` from the GitHub Release and unpacks it
  into the consumer's `native/` (or `Libraries/`) folder. Run during dev setup and in CI
  before build.
- **`<Target>` in the consumer csproj** running `BeforeTargets="Build"` that does the
  same, then copies the right platform binary into `$(OutputPath)`.

P/Invoke looks up the library by name `rust_eth`. The runtime resolves to
`rust_eth.dll` / `librust_eth.so` / `librust_eth.dylib` on the respective OSes.

## Consuming from Unity

Add to the consumer's `Packages/manifest.json`:

```json
{
  "dependencies": {
    "com.decentraland.rust-ethereum": "https://github.com/decentraland/rust-ethereum.git?path=unity#v0.2.0"
  }
}
```

The tag resolves to a release commit that includes platform binaries with their `.meta`
files preconfigured (Win64 + Editor x86_64, Linux Any, macOS OSXUniversal + Editor).
Reference the assembly in your `.asmdef`:

```json
{
  "references": ["DCL.RustEthereum"]
}
```

## Public C# API

```csharp
using DCL.RustEthereum;

// Stateless API (multi-account safe)
string addr = RustEth.AddressFromPrivateKey(privateKey32Bytes);
string sig  = RustEth.SignMessage(privateKey32Bytes, "hello");
bool   ok   = RustEth.Verify(addr, "hello", sig);

// Stateful API (single active signer per process)
RustEthSignServer.Initialize(privateKey32Bytes);
byte[] raw  = RustEthSignServer.Sign("hello");
```

## License

Apache 2.0. See [`LICENSE`](LICENSE).
