//! Generates `src/bindings.rs` from AMD's D3D12 extension headers in [`HEADER_DIR`].
//!
//! Run from the repository root: `cargo run -p api_gen`.
//!
//! # Pipeline
//!
//! This follows the `windows-clang` -> RDL -> winmd -> `windows-bindgen` pipeline that
//! `d3d12-agility` (and windows-rs itself) use, as an *additive* scrape on top of the
//! `d3d12-agility` metadata, similar to how windows-rs scrapes the WDK on top of Win32:
//!
//! 1. The `d3d12-agility` metadata is compiled to a reference winmd from the committed RDL in the
//!    exact `d3d12-agility` checkout that Cargo resolved for this crate (found via
//!    `cargo metadata`), so the reference can never drift from the dependency.
//! 2. The AMD headers are parsed by libclang with the same Agility SDK and Windows SDK headers that
//!    `d3d12-agility` is generated from, into the same flat `Windows.Win32` root namespace. Every
//!    entity the reference winmd already defines (`ID3D12GraphicsCommandList`,
//!    `D3D12_GRAPHICS_PIPELINE_STATE_DESC`, `IUnknown`, ...) is skipped, so the committed RDL in
//!    [`RDL_DIR`] only holds the AMD declarations, and references to D3D12 resolve by bare name.
//! 3. `windows-bindgen` projects the AMD items in flat mode, so every reference to a D3D12 type
//!    is a bare name (`ID3D12GraphicsCommandList`, not a path).
//! 4. `windows-bindgen` 0.100 removed `--reference`, which used to make it skip types provided by
//!    another crate. Without it, the dependency closure of the AMD items (the D3D12 types above,
//!    their fields and base interfaces) is emitted as private copies, and `!` exclude filters do
//!    not help: they drop every item that depends on an excluded type. Instead, every top-level
//!    item that `d3d12-agility` defines is stripped from the output (see [`strip_items`]), and
//!    `use d3d12_agility::*;` is prepended so the bare names resolve to the `d3d12-agility` types.
//!    The bindings therefore share type identity with `d3d12-agility` and every crate built on it.
//!    Generation fails if anything other than an AMD item remains.

use std::{
    collections::BTreeSet,
    path::{Path, PathBuf},
    process::Command,
};
use windows_clang::*;

/// Pinned `Microsoft.Direct3D.D3D12` (Agility SDK) NuGet package version. Keep in sync with
/// `AGILITY_VERSION` in the `d3d12-agility` revision this crate depends on.
const AGILITY_VERSION: &str = "1.722.0-preview";

/// Pinned Windows SDK. Keep in sync with `SDK_VERSION` in the `d3d12-agility` revision this crate
/// depends on.
const SDK_VERSION: &str = "10.0.28000.2270";

/// AMD extension headers, taken from AMD's `AmdDxExt` sample (see `CONTRIBUTING.md`).
const HEADER_DIR: &str = "api_gen/include";

/// The headers whose declarations are emitted. `AmdPix3.h` only holds C++ inline helpers.
const HEADERS: &[&str] = &[
    "AmdDxExt/AmdExtD3D.h",
    "AmdDxExt/AmdExtD3DDeviceApi.h",
    "AmdDxExt/AmdExtD3DCommandListMarkerApi.h",
];

/// Committed per-header RDL snapshot of the AMD declarations, reviewable in pull requests.
const RDL_DIR: &str = "metadata";

/// Generated bindings.
const BINDINGS: &str = "src/bindings.rs";

/// Same flat root namespace as `d3d12-agility`, so the AMD declarations resolve D3D12 types by
/// bare name and `windows-bindgen` maps `IUnknown`, `GUID`, `HRESULT`, ... onto `windows-core`.
const ROOT: &str = "Windows.Win32";

/// Same prelude as `d3d12-agility`'s `api-gen`.
const PRELUDE: &str = "#define SECURITY_WIN32\n#define WIN32_NO_STATUS\n#include <winsock2.h>\n#include <windows.h>\n#undef WIN32_NO_STATUS\n#include <ntstatus.h>";

/// Emitted after [`PRELUDE`], before the AMD headers. `windows-clang` 0.100.0 maps an enum's fixed
/// underlying type without canonicalizing it, so `enum AmdExtD3DStructType : UINT` falls back to
/// `i32`. Spelling the typedef as its builtin type sidesteps that. `UINT` never appears in the
/// generated metadata (it is resolved to `u32` either way), so this changes nothing else.
/// Fixed upstream by https://github.com/microsoft/windows-rs/pull/5233; drop once released.
const ENUM_BASE_TYPE_WORKAROUND: &str = "\n#define UINT unsigned int";

fn main() {
    let time = std::time::Instant::now();

    let target_dir =
        PathBuf::from(std::env::var("CARGO_TARGET_DIR").unwrap_or("target".into())).join("api_gen");
    let reference_winmd = target_dir.join("D3D12Agility.winmd");
    let amd_winmd = target_dir.join("AmdExtD3D.winmd");

    std::fs::create_dir_all(&target_dir).unwrap();

    // 1. Compile the `d3d12-agility` reference metadata.
    let agility = d3d12_agility_dir();
    println!("Using d3d12-agility metadata from {}", agility.display());
    windows_rdl::reader()
        .input(agility.join("metadata"))
        // Hand-authored metadata attribute vocabulary that `d3d12-agility` seeds its scrape with.
        .input(agility.join("api-gen/src/metadata.rdl"))
        .output(&reference_winmd)
        .write()
        .unwrap_or_else(|e| panic!("failed to compile the d3d12-agility metadata: {e}"));

    // 2. Scrape the AMD headers on top of it.
    ensure_libclang();
    assert_libclang_version();

    let agility_include = nuget_package("microsoft.direct3d.d3d12", AGILITY_VERSION)
        .join("build")
        .join("native")
        .join("include");

    // The Agility SDK include directory goes before the Windows SDK, exactly like in
    // `d3d12-agility`, so `<d3d12.h>` resolves to the same header the D3D12 types came from.
    let mut include_dirs = vec![slashed(Path::new(HEADER_DIR)), slashed(&agility_include)];
    include_dirs.extend(sdk_include_dirs());
    let include_args: Vec<String> = include_dirs
        .into_iter()
        .flat_map(|dir| ["-isystem".to_string(), dir])
        .collect();

    let mut source = String::from(PRELUDE);
    source.push_str(ENUM_BASE_TYPE_WORKAROUND);
    for header in HEADERS {
        source.push_str(&format!("\n#include <{header}>"));
    }

    let mut clang = clang();
    clang
        .args(["-x", "c++", "-ferror-limit=0"])
        // The SAL capture shim `d3d12-agility` uses, so annotations are scraped identically.
        .args([
            "-include".to_string(),
            slashed(&agility.join("api-gen/src/sal.h")),
        ])
        .args(include_args)
        // `AmdExtD3DCreateInterface` is not exported by any import library: the driver DLL is
        // loaded at runtime and only its `PFNAmdExtD3DCreateInterface` signature is used.
        .drop_lib_less()
        // A directory scope that matches nothing enables the reachability sweep with `HEADERS`
        // as its only roots; see `d3d12-agility`'s `api-gen`.
        .scope("amd-ext-d3d-no-directory-scope")
        .scope_headers(HEADERS.iter().copied());
    clang.input_text(&source);

    let summary = clang.scrape(&ScrapePlan {
        root: ROOT.to_string(),
        rdl_dir: RDL_DIR.into(),
        out_dir: target_dir.join("scrape"),
        winmd: amd_winmd.clone(),
        archs: Arch::canonical_plus(&[], |name| Arch::known(name).unwrap()),
        // Everything `d3d12-agility` defines is excluded from the scrape.
        reference_winmds: vec![reference_winmd.clone()],
        resolution_winmds: Vec::new(),
        seed: None,
        parallel: false,
    });
    print!("{summary}");

    // 3. Project only the AMD items.
    let amd_items = rdl_items(Path::new(RDL_DIR));
    assert!(!amd_items.is_empty(), "no AMD declarations were scraped");
    let agility_items = rdl_items(&agility.join("metadata"));

    let mut args = vec![
        "--in".to_string(),
        slashed(&reference_winmd),
        "--in".to_string(),
        slashed(&amd_winmd),
        "--out".to_string(),
        BINDINGS.to_string(),
        "--flat".to_string(),
        "--filter".to_string(),
    ];
    args.extend(amd_items.iter().map(|item| format!("{ROOT}.{item}")));
    windows_bindgen::bindgen(&args);

    // 4. Replace the D3D12 copies with `d3d12-agility`.
    let bindings = std::fs::read_to_string(BINDINGS).unwrap();
    let bindings = strip_items(&bindings, &amd_items, &agility_items);
    std::fs::write(BINDINGS, bindings).unwrap();

    println!("Finished in {:.2}s", time.elapsed().as_secs_f32());
}

/// Returns the source directory of the `d3d12-agility` package that Cargo resolved for this
/// workspace. For a git dependency this is a full checkout, including its `metadata/` RDL.
fn d3d12_agility_dir() -> PathBuf {
    let output = Command::new(std::env::var("CARGO").unwrap_or("cargo".into()))
        .args(["metadata", "--format-version", "1"])
        .output()
        .expect("failed to run `cargo metadata`");
    assert!(
        output.status.success(),
        "`cargo metadata` failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let metadata: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    let package = metadata["packages"]
        .as_array()
        .unwrap()
        .iter()
        .find(|package| package["name"] == "d3d12-agility")
        .expect("`d3d12-agility` is not a dependency of this workspace");
    let manifest = package["manifest_path"].as_str().unwrap();
    Path::new(manifest).parent().unwrap().to_path_buf()
}

/// Removes every top-level item of the generated `bindings` that belongs to a `d3d12-agility`
/// item, and prepends `use d3d12_agility::*;` so the remaining bare-name references resolve to
/// the `d3d12-agility` types. Panics if an item belongs to neither crate.
///
/// Items are removed as whole lines, so the remaining code keeps the `rustfmt` formatting that
/// `windows-bindgen` applied.
fn strip_items(
    bindings: &str,
    amd_items: &BTreeSet<String>,
    agility_items: &BTreeSet<String>,
) -> String {
    use syn::spanned::Spanned;

    let file = syn::parse_file(bindings).expect("failed to parse the generated bindings");
    assert!(file.attrs.is_empty() && file.shebang.is_none());

    // 1-based, inclusive line ranges of the items to drop.
    let mut drop = vec![];
    for item in &file.items {
        let mut name =
            owner(item).unwrap_or_else(|| panic!("unexpected item in bindings: {item:?}"));
        // Enum variants are emitted as constants of the (integer alias) enum type and belong to
        // that enum.
        if let syn::Item::Const(item) = item
            && !amd_items.contains(&name)
            && !agility_items.contains(&name)
            && let syn::Type::Path(ty) = &*item.ty
            && let Some(ty) = ty.path.get_ident()
        {
            name = ty.to_string();
        }
        if agility_items.contains(&name) {
            let span = item.span();
            drop.push(span.start().line..=span.end().line);
        } else {
            assert!(
                amd_items.contains(&name),
                "`{name}` is neither an AMD nor a d3d12-agility item"
            );
        }
    }

    let mut out = String::from(
        "// Generated by `cargo run -p api_gen`, do not edit.\n\
         //\n\
         // D3D12 types are referenced by bare name and come from `d3d12-agility`, so that they\n\
         // share type identity with every other crate built on `d3d12-agility`.\n\
         use d3d12_agility::*;\n\n",
    );
    for (line_number, line) in bindings.lines().enumerate() {
        if !drop.iter().any(|range| range.contains(&(line_number + 1))) {
            out.push_str(line);
            out.push('\n');
        }
    }
    out
}

/// Returns the name of the metadata item that a generated top-level Rust item belongs to, e.g.
/// `ID3D12Object` for `ID3D12Object_Vtbl`, `impl ID3D12Object_Vtbl` and
/// `define_interface!(ID3D12Object, ...)`.
fn owner(item: &syn::Item) -> Option<String> {
    let ident = match item {
        syn::Item::Const(item) => item.ident.clone(),
        syn::Item::Fn(item) => item.sig.ident.clone(),
        syn::Item::Struct(item) => item.ident.clone(),
        syn::Item::Union(item) => item.ident.clone(),
        syn::Item::Type(item) => item.ident.clone(),
        syn::Item::Trait(item) => item.ident.clone(),
        syn::Item::Impl(item) => match &*item.self_ty {
            syn::Type::Path(path) => path.path.segments.last()?.ident.clone(),
            _ => return None,
        },
        // `define_interface!(Name, ...)`, `interface_hierarchy!(Name, ...)`, ...
        syn::Item::Macro(item) => match item.mac.tokens.clone().into_iter().next()? {
            proc_macro2::TokenTree::Ident(ident) => ident,
            _ => return None,
        },
        _ => return None,
    };
    let name = ident.to_string();
    let name = name.strip_prefix("r#").unwrap_or(&name);
    let name = name.strip_suffix("_Vtbl").unwrap_or(name);
    let name = name.strip_suffix("_Impl").unwrap_or(name);
    Some(name.to_string())
}

/// Returns the names of all items defined under [`ROOT`] in the `.rdl` files in `dir`.
fn rdl_items(dir: &Path) -> BTreeSet<String> {
    let mut items = BTreeSet::new();
    for entry in std::fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().is_some_and(|ext| ext == "rdl") {
            items.extend(windows_rdl::item_names(&path, ROOT).unwrap());
        }
    }
    items
}

fn slashed(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

/// `10.0.28000.2270` -> `10.0.28000.0`, the directory name the SDK packages use.
fn marketing_dir(version: &str) -> String {
    let mut parts = version.split('.');
    match (parts.next(), parts.next(), parts.next()) {
        (Some(major), Some(minor), Some(build)) => format!("{major}.{minor}.{build}.0"),
        _ => panic!("`{version}` is not a `major.minor.build[.revision]` version"),
    }
}

fn sdk_include_dirs() -> Vec<String> {
    let base: PathBuf = nuget_package("microsoft.windows.sdk.cpp", SDK_VERSION)
        .join("c")
        .join("Include")
        .join(marketing_dir(SDK_VERSION));
    ["ucrt", "um", "shared", "winrt", "cppwinrt"]
        .iter()
        .map(|seg| slashed(&base.join(seg)))
        .collect()
}
