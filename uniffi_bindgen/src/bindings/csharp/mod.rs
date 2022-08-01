pub mod gen_csharp;

use self::gen_csharp::generate_csharp_bindings;
use crate::ComponentInterface;
use anyhow::{bail, Context, Result};
use camino::Utf8Path;
pub use gen_csharp::Config;
use std::{fs::File, io::Write, process::Command};

pub fn write_bindings(
    config: &Config,
    ci: &ComponentInterface,
    out_dir: &Utf8Path,
    _try_format_code: bool,
) -> Result<()> {
    let cs_file = out_dir.join(format!("{}.cs", ci.namespace()));
    let mut f = File::create(&cs_file)?;
    write!(f, "{}", generate_csharp_bindings(config, ci)?)?;

    Ok(())
}

pub fn run_script(out_dir: &Utf8Path, script_file: &Utf8Path) -> Result<()> {
    let cs_proj = out_dir.join("cs.csproj");
    let mut cs_proj = File::create(&cs_proj)?;
    write!(
        cs_proj,
        r#"
        <Project Sdk="Microsoft.NET.Sdk">
            <PropertyGroup>
                <OutputType>Exe</OutputType>
                <TargetFramework>net6.0</TargetFramework>
                <ImplicitUsings>enable</ImplicitUsings>
                <Nullable>enable</Nullable>
            </PropertyGroup>  
        </Project>
        "#
    )?;

    let main = out_dir.join("main.cs");
    std::fs::write(main, std::fs::read(script_file)?)?;

    let mut cmd = Command::new("dotnet");
    cmd.current_dir(out_dir);
    cmd.arg("run");
    let status = cmd
        .spawn()
        .context("Failed to spawn `dotnet` when running script")?
        .wait()
        .context("Failed to wait for `dotnet` when running script")?;

    if !status.success() {
        bail!("running `dotnet` failed")
    }

    Ok(())
}
