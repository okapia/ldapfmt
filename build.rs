use clap::crate_name;
use clap_complete::{generate_to, shells::*};
use std::io::Error;

include!("src/cli.rs");

fn main() -> Result<(), Error> {
    let mut cmd = build_cli();
    let outdir = env!("CARGO_MANIFEST_DIR");
    for shell in [Shell::Bash, Shell::Elvish, Shell::Fish, Shell::Zsh] {
        let path = generate_to(shell, &mut cmd, crate_name!(), outdir)?;
        println!("cargo:warning=completion file generated: {:?}", path);
    }
    Ok(())
}
