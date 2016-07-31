mod piface;

#[cfg(feature = "spi")]
pub use self::piface::PiFace;
