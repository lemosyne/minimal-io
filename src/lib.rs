pub mod blocking;
pub mod stdio;

#[cfg(feature = "stat")]
pub mod stat;

pub trait Io {
    type Error: std::error::Error + core::fmt::Debug;
}

pub enum SeekFrom {
    Start(u64),
    End(i64),
    Current(i64),
}

pub trait Seek: Io {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64, Self::Error>;

    fn rewind(&mut self) -> Result<(), Self::Error> {
        self.seek(SeekFrom::Start(0))?;
        Ok(())
    }

    fn stream_position(&mut self) -> Result<u64, Self::Error> {
        self.seek(SeekFrom::Current(0))
    }
}
