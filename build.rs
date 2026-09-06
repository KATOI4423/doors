//! build.rs
//!
//! Define build process

use anyhow::Result;
use vergen::{
    Build, Cargo, Emitter, Rustc, Sysinfo
};
use vergen_gitcl::Gitcl;

fn main() -> Result<()> {
    generate_build_information()?;

    Ok(())
}

/// # Generate Build Information to `cargo:rustc-env`
fn generate_build_information() -> Result<()> {
    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=.git/refs/heads");

    let build = Build::all_build();
    let cargo = Cargo::all_cargo();
    let rustc = Rustc::all_rustc();
    let si = Sysinfo::all_sysinfo();
    let git = Gitcl::all_git();

    Emitter::default()
        .add_instructions(&build)?
        .add_instructions(&cargo)?
        .add_instructions(&rustc)?
        .add_instructions(&si)?
        .add_instructions(&git)?
        .emit()?;

    Ok(())
}
