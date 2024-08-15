use std::fmt::Debug;

use nix::errno::Errno;

macro_rules! wrap_err {
    ( $from:ty , $wrapper:ident ) => {
        impl From<$from> for Error {
            fn from(value: $from) -> Self {
                Self::$wrapper(value)
            }
        }
    };
}

pub enum Error {
    Libusb(rusb::Error),
    Libc(Errno),
    StdIo(std::io::Error),
}
impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Libusb(err) => write!(f, "libusb error: {err:?}"),
            Error::Libc(err) => write!(f, "C errno: {err:?}"),
            Error::StdIo(err) => write!(f, "IO error: {err:?}"),
        }
    }
}
wrap_err!(rusb::Error, Libusb);
wrap_err!(Errno, Libc);
wrap_err!(std::io::Error, StdIo);
