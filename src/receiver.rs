use std::{
    fs,
    io::{self, Read as _, Write as _},
    net::TcpStream,
    path::{Path, PathBuf},
    time::{Duration, Instant},
};

use crate::num_io::NumReader;

pub fn receive(mut stream: TcpStream, folder: &Path) -> io::Result<()> {
    let num_files = stream.read_num::<u32>()?;
    println!("Receiving {} files...", num_files);

    for _ in 0..num_files {
        let path_len = stream.read_num::<u32>()? as usize;
        let mut path_bytes = Vec::with_capacity(path_len);
        unsafe { path_bytes.set_len(path_len) };

        stream.read_exact(&mut path_bytes)?;
        let rel_path = String::from_utf8_lossy(&path_bytes);
        let rel_path = PathBuf::from(rel_path.as_ref());
        let full_path = folder.join(&rel_path);

        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let mut file = fs::File::create(&full_path)?;

        let file_len = stream.read_num::<u64>()?;
        let mut buf = [0; 64 * 1024];
        let mut total_read = 0;
        let mut elapsed = Instant::now();
        while total_read < file_len {
            let read =
                stream.read(&mut buf[..((file_len - total_read) as usize).min(64 * 1024)])?;
            total_read += read as u64;
            file.write_all(&buf[..read])?;
            if elapsed.elapsed() > Duration::from_secs(1) {
                print!(
                    "\r{rel_path:?}: {:.2}%",
                    total_read as f32 / file_len as f32 * 100.0
                );
                io::stdout().flush()?;
                elapsed = Instant::now();
            }
        }

        println!("\r{rel_path:?}: 100.00%");
    }

    Ok(())
}
