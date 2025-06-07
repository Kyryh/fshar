use std::{
    fs,
    io::{self, Seek as _, Write as _},
    path::{Path, PathBuf},
};

use crate::fshar_io::{FileChunks, NumReader, NumWriter};

pub fn receive(
    mut stream: impl NumWriter + NumReader,
    folder: &Path,
    overwrite: bool,
) -> io::Result<()> {
    let num_files = stream.read_num::<u32>()?;

    for _ in 0..num_files {
        let path_len = stream.read_num::<u32>()? as usize;
        let mut path_bytes = vec![0; path_len];

        stream.read_exact(&mut path_bytes)?;
        let rel_path = String::from_utf8_lossy(&path_bytes);
        let rel_path = PathBuf::from(rel_path.as_ref());
        let full_path = folder.join(&rel_path);

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
                write!(stderr, "\r{rel_path:?}: {:.2}%", current_bps as f32 / 100.0)?;
            }
        }

        println!("\r{rel_path:?}: 100.00%");
    }
    println!("Done");

    Ok(())
}
