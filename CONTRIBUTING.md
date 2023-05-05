# Regenerating bindings

## Source

Bindings in [`AmdDxExt`](.metadata/AmdDxExt/) are taken from [`RadeonDeveloperToolSuite-2023-02-15-1051\samples\AmdDxExt`](https://gpuopen.com/introducing-radeon-developer-tool-suite/).
TODO: Take them from https://github.com/GPUOpen-Tools/common_src_amddxext instead _if that is the correct upstream_.

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
