# Regenerating bindings

After updating the source, which includes [configuration files for the bindings and the upstream headers themselves](/.metadata) from <https://github.com/GPUOpen-Tools/common_src_amddxext>, both [the `.winmd`](/.windows/winmd/Amd.Ext.D3D.winmd) and [final Rust bindings](/src/bindings.rs) need to be regenerated. Follow these steps to regenerate the files locally or through automated CI setup steps.

## Regenerate via CI

Push a branch to our repository. The CI will automatically make new files available as artifacts to be downloaded and integrated.

## Regenerate locally

### Install dependencies

- Visual Studio 2022 (via `Visual Studio Installer`)
  - .NET SDK
  - .NET 6.0 Runtime
  - Windows 11 SDK (10.0.22000.0)
  - MSVC C++ x64/x86 build tools (Latest)
  - MSVC C++ ARM64 build tools (Latest)
- .NET 6.0 SDK: <https://dotnet.microsoft.com/en-us/download/dotnet/6.0>
  (Cannot be installed through `Visual Studio Installer`)

### Regenerate metadata

From the root of the repository:

```sh
dotnet build .metadata
```

### Regenerate rust code

From the root of the repository:

```sh
cargo run -p api_gen
```
