pub mod treer;
pub mod meta_infos;
pub mod formater;

pub use meta_infos::*;
pub use treer::*;
pub use std::path::PathBuf;
pub use std::env;

fn main() -> anyhow::Result<()> {
    let path: PathBuf = env::current_dir()?;
    tree("".to_string(), path)?;
    Ok(())
}