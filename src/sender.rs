use std::{
    fs,
    io::{self, Write as _},
    path::{Path, PathBuf},
};

use crate::fshar_io::{FSharReader, FSharWriter, FileChunks};

pub fn send(mut stream: impl FSharWriter + FSharReader, folder: &Path) -> io::Result<()> {
    fs::create_dir_all(folder)?;
    let files = collect_files(folder, folder)?;

    let num_files = files.len() as u32;
    stream.write_num(&num_files)?;

    for (file_path, full_path) in files {
        // Send path info of the file
        stream.write_path(&file_path)?;

        let mut chunks = FileChunks::<{ 64 * 1024 }>::sender_file(fs::File::open(&full_path)?)?;

        // Send the total size of the file..
        stream.write_num(&chunks.total_len())?;
        // ..and receive how much of it the
        // receiver has already downlaoded
        chunks.seek_to(stream.read_num::<u64>()?)?;

        let mut file_bps: Option<u64> = None;
        let mut stderr = io::stderr().lock();

        while let Some(current_bps) = chunks.send_next_chunk(&mut stream)? {
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

fn collect_files(base: &Path, current: &Path) -> io::Result<Vec<(PathBuf, PathBuf)>> {
    let mut files = Vec::new();
    for entry in fs::read_dir(current)? {
        let path = entry?.path();
        if path.is_dir() {
            files.extend(collect_files(base, &path)?);
        } else {
            // Relative path from base
            let rel_path = path.strip_prefix(base).unwrap();
            files.push((rel_path.to_owned(), path));
        }
    }
    Ok(files)
}
