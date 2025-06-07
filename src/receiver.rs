use std::{
    fs,
    io::{self, Write as _},
    path::Path,
};

use crate::fshar_io::{FSharReader, FSharWriter, FileChunks};

pub fn receive(
    mut stream: impl FSharWriter + FSharReader,
    folder: &Path,
    overwrite: bool,
) -> io::Result<()> {
    let num_files = stream.read_num::<u32>()?;

    for _ in 0..num_files {
        let file_path = stream.read_path()?;
        let full_path = folder.join(&file_path);

        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let mut chunks = FileChunks::<{ 64 * 1024 }>::receiver_file(
            fs::OpenOptions::new()
                .append(!overwrite)
                .write(overwrite)
                .truncate(overwrite)
                .create(true)
                .open(full_path)?,
            // Read the total size of the file..
            stream.read_num::<u64>()?,
        )?;

        // ..and write how much of it we already downloaded
        stream.write_num(&chunks.current_len())?;

        let mut file_bps: Option<u64> = None;
        let mut stderr = io::stderr().lock();

        while let Some(current_bps) = chunks.receive_next_chunk(&mut stream)? {
            if !matches!(file_bps, Some(bps) if bps == current_bps) {
                file_bps = Some(current_bps);
                write!(
                    stderr,
                    "\r{file_path:?}: {:.2}%",
                    current_bps as f32 / 100.0
                )?;
            }
        }

        println!("\r{file_path:?}: 100.00%");
    }
    println!("Done");

    Ok(())
}
