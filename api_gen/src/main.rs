use std::io::prelude::*;

use anyhow::{Context, Result};
use windows_metadata::reader::File;

fn main() -> Result<()> {
    let start = std::time::Instant::now();
    let output_path = std::path::PathBuf::from("src");

    let winmd_files = [
        File::new(".windows/winmd/Amd.Ext.D3D.winmd")?,
        File::new(".windows/winmd/Windows.Win32.winmd")?,
    ];

    let _ = std::fs::remove_dir_all(output_path.join("Amd"));

    for namespace in ["Amd", "Amd.Ext", "Amd.Ext.D3D"] {
        let path = output_path.join(namespace.replace('.', "/"));
        std::fs::create_dir_all(&path)?;
        let tokens = windows_bindgen::component(namespace, &winmd_files);
        let output = std::fs::File::create(path.join("mod.rs"))?;
        fmt_tokens(&tokens, output).context("While formatting tokens")?;
    }

    println!("Took {:.1?}", start.elapsed());

    Ok(())
}

fn fmt_tokens(tokens: &str, output: std::fs::File) -> Result<()> {
    let mut child = std::process::Command::new("rustfmt")
        .stdin(std::process::Stdio::piped())
        .stdout(output)
        .spawn()
        .context("Failed to spawn `rustfmt`")?;

    let mut stdin = child.stdin.take().context("Failed to open stdin")?;
    stdin.write_all(tokens.as_bytes())?;
    drop(stdin);

    let output = child.wait_with_output()?;
    let stderr = std::str::from_utf8(&output.stderr)?;
    anyhow::ensure!(output.status.success(), "rustfmt failed: {}", stderr);

    if !stderr.is_empty() {
        eprintln!("rustfmt stderr: {}", stderr)
    }

    Ok(())
}
