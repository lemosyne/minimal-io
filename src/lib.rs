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
