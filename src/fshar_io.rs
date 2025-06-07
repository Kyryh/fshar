use std::{
    fs::File,
    io::{self, Read, Write},
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
    chunk_buf: [u8; N],
}

impl<const N: usize> FileChunks<N> {
    pub fn send_next_chunk(&mut self, stream: &mut impl Write) -> io::Result<Option<usize>> {
        match self.next_chunk()? {
            Some(chunk) => {
                stream.write_all(chunk)?;
                Ok(Some(chunk.len()))
            }
            None => Ok(None),
        }
    }
    pub fn next_chunk(&mut self) -> io::Result<Option<&[u8]>> {
        match self.file.read(&mut self.chunk_buf) {
            Ok(0) => Ok(None),
            Ok(n) => Ok(Some(&self.chunk_buf[..n])),
            Err(err) => Err(err),
        }
    }
}

impl<const N: usize> From<File> for FileChunks<N> {
    fn from(value: File) -> Self {
        Self {
            chunk_buf: [0; N],
            file: value,
        }
    }
}
