use std::{
    fs,
    io::{self, Read as _, Write as _},
    net::TcpStream,
    path::{Path, PathBuf},
    time::{Duration, Instant},
};

use crate::num_io::NumWriter as _;

pub fn send(mut stream: TcpStream, folder: &Path) -> io::Result<()> {
    let files = collect_files(folder, folder)?;

    let num_files = files.len() as u32;
    stream.write_num(&num_files)?;

    for (rel_path, abs_path) in files {
        let rel_path_str = rel_path.to_string_lossy();
        let path_bytes = rel_path_str.as_bytes();
        stream.write_num(&(path_bytes.len() as u32))?;
        stream.write_all(path_bytes)?;

        let mut file = fs::File::open(&abs_path)?;
        let file_len = fs::metadata(&abs_path)?.len();
        stream.write_num(&file_len)?;
        let mut buf = [0; 64 * 1024];
        let mut elapsed = Instant::now();
        let mut total_written = 0;
        while let Ok(n) = file.read(&mut buf) {
            if n == 0 {
                break;
            }
            total_written += n;
            stream.write_all(&buf[..n])?;
            if elapsed.elapsed() > Duration::from_secs(1) {
                print!(
                    "\r{rel_path:?}: {:.2}%",
                    total_written as f32 / file_len as f32 * 100.0
                );
                io::stdout().flush()?;
                elapsed = Instant::now();
            }
        }
        println!("\r{rel_path:?}: 100.00%");
    }
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
