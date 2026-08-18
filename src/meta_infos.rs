pub use std::os::windows::fs::MetadataExt;
pub use crate::formater::format_time_stamp;

pub fn meta_infos() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("\n");
        eprintln!(r"Usage: {} [path]", &args[0]);
        std::process::exit(1);
    }

    let ref file_path = args[1];
    match std::fs::metadata(file_path) {
        Ok(metadata) => {
            if metadata.is_dir() { println!("File Type: Directory"); }
            else if metadata.is_file() { println!("File Type: File"); }
            else if metadata.is_symlink() { println!("File Type: Symbol link"); }
            else { println!("File Type: Unknown"); }
            println!("File Size: {} KB", metadata.file_size() / 1024);
            println!("Created at: {}", format_time_stamp(metadata.created()?));
            println!("Is Read Only: {:?}", metadata.permissions().readonly());
            println!("Last Modified: {}", format_time_stamp(metadata.modified()?));
            println!("Last accessed: {}", format_time_stamp(metadata.accessed()?));
        }
        Err(e) => {
            anyhow::bail!("{}", e);
        }
    }

    Ok(())
}
