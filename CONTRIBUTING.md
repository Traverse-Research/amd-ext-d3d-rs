# Regenerating bindings

## Source

Bindings in [`AmdDxExt`](api_gen/include/AmdDxExt/) are taken from [`RadeonDeveloperToolSuite-2023-02-15-1051\samples\AmdDxExt`](https://gpuopen.com/introducing-radeon-developer-tool-suite/).
TODO: Take them from <https://github.com/GPUOpen-Tools/common_src_amddxext> instead _if that is the correct upstream_.

## Regenerate

From the root of the repository, on Windows:

```sh
cargo run -p api_gen
```

This needs no Visual Studio or .NET installation: the pinned libclang, Windows SDK and DirectX 12 Agility SDK NuGet packages are downloaded on first use. It:

1. compiles the metadata (RDL) of the exact [`d3d12-agility`](https://github.com/Traverse-Research/d3d12-agility-rs) revision this crate depends on into a reference `.winmd`;
2. scrapes the AMD headers with [`windows-clang`](https://crates.io/crates/windows-clang) against the same Agility SDK and Windows SDK headers, skipping everything `d3d12-agility` already defines, and writes the AMD declarations to [`metadata/`](metadata) (review its diff to see what changed in the API);
3. generates `src/bindings.rs` with [`windows-bindgen`](https://crates.io/crates/windows-bindgen), and replaces the D3D12 types it pulls in with `use d3d12_agility::*;` so they are the `d3d12-agility` types.

See the documentation at the top of [`api_gen/src/main.rs`](api_gen/src/main.rs) for details.

When bumping the `d3d12-agility` dependency, keep `AGILITY_VERSION` and `SDK_VERSION` in `api_gen/src/main.rs` in sync with those in its `api-gen`. CI verifies that the committed bindings match a fresh regeneration.
