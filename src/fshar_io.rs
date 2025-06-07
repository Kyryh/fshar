use std::{
    fs::File,
    io::{self, Read, Seek, Write},
};

pub trait Number: Sized {
    fn read_num(reader: &mut impl Read) -> io::Result<Self>;
    fn write_num(&self, writer: &mut impl Write) -> io::Result<()>;
}

macro_rules! impl_num {
    ($num:ty) => {
        impl Number for $num {
            fn read_num(reader: &mut impl Read) -> io::Result<Self> {
                let mut buf = [0; size_of::<Self>()];

                reader.read_exact(&mut buf)?;
                Ok(Self::from_be_bytes(buf))
            }

            fn write_num(&self, writer: &mut impl Write) -> io::Result<()> {
                writer.write_all(&self.to_be_bytes())
            }
        }
    };
}

impl_num!(u8);
impl_num!(u16);
impl_num!(u32);
impl_num!(u64);

pub trait NumReader: Read + Sized {
    fn read_num<T>(&mut self) -> io::Result<T>
    where
        T: Number,
    {
        T::read_num(self)
    }
}

pub trait NumWriter: Write + Sized {
    fn write_num<T>(&mut self, num: &T) -> io::Result<()>
    where
        T: Number,
    {
        num.write_num(self)
    }
}

impl<R> NumReader for R where R: Read {}
impl<W> NumWriter for W where W: Write {}

pub struct FileChunks<const N: usize> {
    file: File,
    total_file_len: u64,
    current_file_len: u64,
    chunk_buf: [u8; N],
}

impl<const N: usize> FileChunks<N> {
    pub fn send_next_chunk(&mut self, stream: &mut impl Write) -> io::Result<Option<u64>> {
        match self.file.read(&mut self.chunk_buf) {
            Ok(0) => Ok(None),
            Ok(n) => {
                self.current_file_len += n as u64;
                stream.write_all(&self.chunk_buf[..n])?;
                Ok(Some(self.progress()))
            }
            Err(err) => Err(err),
        }
    }
    pub fn receive_next_chunk(&mut self, stream: &mut impl Read) -> io::Result<Option<u64>> {
        let remaining = (self.total_file_len - self.current_file_len) as usize;
        match stream.read(&mut self.chunk_buf[..remaining.min(N)]) {
            Ok(0) => Ok(None),
            Ok(n) => {
                self.current_file_len += n as u64;
                self.file.write_all(&self.chunk_buf[..n])?;
                Ok(Some(self.progress()))
            }
            Err(err) => Err(err),
        }
    }

    pub fn sender_file(file: File) -> io::Result<Self> {
        Ok(Self {
            current_file_len: 0,
            total_file_len: file.metadata()?.len(),
            file,
            chunk_buf: [0; N],
        })
    }

    pub fn receiver_file(file: File, total_file_len: u64) -> io::Result<Self> {
        let mut chunks = Self {
            current_file_len: file.metadata()?.len(),
            total_file_len,
            file,
            chunk_buf: [0; N],
        };
        chunks.seek_to(chunks.current_file_len)?;
        Ok(chunks)
    }

    /// Progress of file sending/receiving in bps (1bp = 0.01%)
    #[inline]
    pub fn progress(&self) -> u64 {
        self.current_file_len * 10000 / self.total_file_len
    }

    #[inline]
    pub fn total_len(&self) -> u64 {
        self.total_file_len
    }

    #[inline]
    pub fn current_len(&self) -> u64 {
        self.current_file_len
    }

    #[inline]
    pub fn seek_to(&mut self, position: u64) -> io::Result<u64> {
        self.current_file_len = position;
        self.file.seek(io::SeekFrom::Start(position))
    }
}
