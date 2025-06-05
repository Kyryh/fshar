use std::io::{self, Read, Write};

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
