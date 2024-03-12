use anyhow::{anyhow, Context};
use build::generate;
use databake::quote;
use std::env;
use std::path::Path;

pub fn main() -> anyhow::Result<()> {
    let baked = generate();
    let my_data_rs = quote! {
       pub(crate) static SETUPS: &[Setup] = #baked;
    };

    let out_dir = env::var_os("OUT_DIR").ok_or(anyhow!("env not found"))?;
    let dest_path = Path::new(&out_dir).join("generated.rs");
    std::fs::write(dest_path, my_data_rs.to_string()).context("writing file")?;

    Ok(())
}
