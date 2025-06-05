use std::{
    fs,
    io::{self, Seek as _, Write as _},
    path::{Path, PathBuf},
};

use crate::num_io::{NumReader, NumWriter};

pub fn receive(mut stream: impl NumWriter + NumReader, folder: &Path) -> io::Result<()> {
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

        let mut file = fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(full_path)?;

        let file_len = stream.read_num::<u64>()?;
        let mut buf = [0; 64 * 1024];
        let mut total_read = {
            let already_written = file.metadata()?.len();
            stream.write_num(&already_written)?;
            file.seek_relative(already_written as i64)?;
            already_written
        };
        // 1bp = 0.01%
        let mut file_bps: Option<u64> = None;
        let mut stderr = io::stderr().lock();
        while total_read < file_len {
            let read =
                stream.read(&mut buf[..((file_len - total_read) as usize).min(64 * 1024)])?;
            total_read += read as u64;
            file.write_all(&buf[..read])?;
            let current_bps = total_read * 10000 / file_len;
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
