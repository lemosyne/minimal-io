use crate::Io;
use thiserror::Error;

const DEFAULT_BLOCK_SIZE: usize = 4096;

#[derive(Debug, Error)]
pub enum ReadExactError<E> {
    #[error("failed to fill whole buffer")]
    UnexpectedEof,

    #[error(transparent)]
    Other(#[from] E),
}

pub trait Read: Io {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error>;

    fn read_exact(&mut self, mut buf: &mut [u8]) -> Result<(), ReadExactError<Self::Error>> {
        while !buf.is_empty() {
            match self.read(buf) {
                Ok(0) => break,
                Ok(n) => buf = &mut buf[n..],
                Err(e) => return Err(ReadExactError::Other(e)),
            }
        }

        if !buf.is_empty() {
            Err(ReadExactError::UnexpectedEof)
        } else {
            Ok(())
        }
    }

    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize, Self::Error> {
        let mut count = 0;
        let mut block = [0; DEFAULT_BLOCK_SIZE];

        loop {
            match self.read(&mut block) {
                Ok(0) => break,
                Ok(n) => {
                    count += n;
                    buf.extend(&block[..n]);
                }
                Err(e) => return Err(e),
            }
        }

        Ok(count)
    }
}

pub trait ReadAt: Io {
    fn read_at(&mut self, buf: &mut [u8], offset: u64) -> Result<usize, Self::Error>;

    fn read_exact_at(
        &mut self,
        mut buf: &mut [u8],
        mut offset: u64,
    ) -> Result<(), ReadExactError<Self::Error>> {
        while !buf.is_empty() {
            match self.read_at(buf, offset) {
                Ok(0) => break,
                Ok(n) => {
                    let tmp = buf;
                    buf = &mut tmp[n..];
                    offset += n as u64;
                }
                Err(e) => return Err(e.into()),
            }
        }

        if !buf.is_empty() {
            Err(ReadExactError::UnexpectedEof)
        } else {
            Ok(())
        }
    }

    fn read_to_end_at(&mut self, buf: &mut Vec<u8>, mut offset: u64) -> Result<usize, Self::Error> {
        let mut count = 0;
        let mut block = [0; DEFAULT_BLOCK_SIZE];

        loop {
            match self.read_at(&mut block, offset) {
                Ok(0) => break,
                Ok(n) => {
                    count += n;
                    offset += n as u64;
                    buf.extend(&block[..n]);
                }
                Err(e) => return Err(e),
            }
        }

        Ok(count)
    }
}

#[derive(Debug, Error)]
pub enum WriteAllError<E> {
    #[error("failed to write whole buffer")]
    WriteZero,

    #[error(transparent)]
    Other(#[from] E),
}

pub trait Write: Io {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error>;

    fn flush(&mut self) -> Result<(), Self::Error>;

    fn write_all(&mut self, mut buf: &[u8]) -> Result<(), WriteAllError<Self::Error>> {
        while !buf.is_empty() {
            match self.write(buf) {
                Ok(0) => return Err(WriteAllError::WriteZero),
                Ok(n) => buf = &buf[n..],
                Err(e) => return Err(e.into()),
            }
        }
        Ok(())
    }
}

pub trait WriteAt: Io {
    fn write_at(&mut self, buf: &[u8], offset: u64) -> Result<usize, Self::Error>;

    fn flush(&mut self) -> Result<(), Self::Error>;

    fn write_all_at(
        &mut self,
        mut buf: &[u8],
        mut offset: u64,
    ) -> Result<(), WriteAllError<Self::Error>> {
        while !buf.is_empty() {
            match self.write_at(buf, offset) {
                Ok(0) => return Err(WriteAllError::WriteZero),
                Ok(n) => {
                    buf = &buf[n..];
                    offset += n as u64
                }
                Err(e) => return Err(WriteAllError::Other(e)),
            }
        }
        Ok(())
    }
}
