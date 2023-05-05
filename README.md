# Rust bindings for AMD's DirectX12 RGP markers

[![Actions Status](https://github.com/Traverse-Research/amd-ext-d3d-rs/workflows/Continuous%20integration/badge.svg)](https://github.com/Traverse-Research/amd-ext-d3d-rs/actions)
[![Latest version](https://img.shields.io/crates/v/amd-ext-d3d-rs.svg)](https://crates.io/crates/amd-ext-d3d-rs)
[![Documentation](https://docs.rs/amd-ext-d3d-rs/badge.svg)](https://docs.rs/amd-ext-d3d-rs)
[![Lines of code](https://tokei.rs/b1/github/Traverse-Research/amd-ext-d3d-rs)](https://github.com/Traverse-Research/amd-ext-d3d-rs)
![MIT](https://img.shields.io/badge/license-MIT-blue.svg)
[![Contributor Covenant](https://img.shields.io/badge/contributor%20covenant-v1.4%20adopted-ff69b4.svg)](../master/CODE_OF_CONDUCT.md)

[![Banner](banner.png)](https://traverseresearch.nl)

Call AMD's command list markers directly in Rust for event instrumentation in [Radeon GPU Profiler], as well as other functionality provided on `IAmdExtD3DDevice1`.

## Usage

Add this to your Cargo.toml:

```toml
[dependencies]
amd-ext-d3d = "0.0.0"
```

While [upstream documentation] details how to embed these calls into existing PIX3 markers (behind a C `#derive`), you get to call them explicitly in Rust:

```rust,no_run
# use std::ffi::CString;
# use windows::Win32::Graphics::Direct3D12::{ID3D12Device, ID3D12GraphicsCommandList};
# use windows::core::CanInto;
use amd_ext_d3d::AmdExtD3DDevice;

# let device: ID3D12Device = todo!("Open DirectX12 graphics device");
# let cmd_list: ID3D12GraphicsCommandList = todo!("device.CreateCommandList(...)");

// Load the extension
let amd_device = unsafe { AmdExtD3DDevice::new(device.can_clone_into()) }.unwrap();

let name = "My super cool GPU event!";
unsafe { amd_device.push_marker(&cmd_list, &CString::new(name).unwrap()) };
// Submit work to the GPU...
unsafe { amd_device.pop_marker(&cmd_list) };
```

[Radeon GPU Profiler]: https://gpuopen.com/rgp/
[upstream documentation]: https://radeon-gpuprofiler.readthedocs.io/en/latest/#directx12-user-markers

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for details on regenerating the metadata and Rust code from upstream headers.
