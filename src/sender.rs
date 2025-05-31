use std::{
    fs,
    io::{self, Read as _, Seek as _, Write as _},
    path::{Path, PathBuf},
};

use crate::num_io::{NumReader, NumWriter};

pub fn send(mut stream: impl NumWriter + NumReader, folder: &Path) -> io::Result<()> {
    fs::create_dir_all(folder)?;
    let files = collect_files(folder, folder)?;

    let num_files = files.len() as u32;
    stream.write_num(&num_files)?;
    //println!("Sending {num_files} files to {}", stream.peer_addr()?);

    for (rel_path, abs_path) in files {
        let rel_path_str = rel_path.to_string_lossy();
        let path_bytes = rel_path_str.as_bytes();
        stream.write_num(&(path_bytes.len() as u32))?;
        stream.write_all(path_bytes)?;

        let mut file = fs::File::open(&abs_path)?;
        let file_len = fs::metadata(&abs_path)?.len();
        stream.write_num(&file_len)?;
        let mut buf = [0; 64 * 1024];
        let mut total_written = {
            let already_written = stream.read_num::<u64>()?;
            file.seek_relative(already_written as i64)?;
            already_written
        };
        // 1bp = 0.01%
        let mut file_bps: Option<u64> = None;
        let mut stderr = io::stderr().lock();
        while let Ok(n) = file.read(&mut buf) {
            if n == 0 {
                break;
            }
            total_written += n as u64;
            stream.write_all(&buf[..n])?;
            let current_bps = total_written * 10000 / file_len;
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

fn collect_files(base: &Path, current: &Path) -> std::io::Result<Vec<(PathBuf, PathBuf)>> {
    let mut files = Vec::new();
    for entry in fs::read_dir(current)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            files.extend(collect_files(base, &path)?);
        } else {
            // Relative path from base
            let rel_path = path.strip_prefix(base).unwrap().to_path_buf();
            files.push((rel_path, path));
        }
    }
    Ok(files)
}
