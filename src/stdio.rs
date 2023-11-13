use crate::{
    blocking::{Read, ReadAt, Seek, Write, WriteAt},
    Io, SeekFrom,
};

pub struct StdIo<T> {
    inner: T,
}

impl<T> StdIo<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }

    pub fn inner(&self) -> &T {
        &self.inner
    }

    pub fn inner_mut(&mut self) -> &mut T {
        &mut self.inner
    }

    pub fn to_inner(self) -> T {
        self.inner
    }
}

impl<T> Io for StdIo<T> {
    type Error = std::io::Error;
}

impl<T: std::io::Read> Read for StdIo<T> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        self.inner.read(buf)
    }
}

impl<T: std::os::unix::fs::FileExt> ReadAt for StdIo<T> {
    fn read_at(&mut self, buf: &mut [u8], offset: u64) -> Result<usize, Self::Error> {
        self.inner.read_at(buf, offset)
    }
}

impl<T: std::io::Write> Write for StdIo<T> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        self.inner.write(buf)
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        self.inner.flush()
    }
}

impl<T: std::os::unix::fs::FileExt + std::io::Write> WriteAt for StdIo<T> {
    fn write_at(&mut self, buf: &[u8], offset: u64) -> Result<usize, Self::Error> {
        self.inner.write_at(buf, offset)
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        self.inner.flush()
    }
}

impl From<SeekFrom> for std::io::SeekFrom {
    fn from(value: SeekFrom) -> Self {
        match value {
            SeekFrom::Start(n) => std::io::SeekFrom::Start(n),
            SeekFrom::End(n) => std::io::SeekFrom::End(n),
            SeekFrom::Current(n) => std::io::SeekFrom::Current(n),
        }
    }
}

impl<T: std::io::Seek> Seek for StdIo<T> {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64, Self::Error> {
        self.inner.seek(pos.into())
    }
}
