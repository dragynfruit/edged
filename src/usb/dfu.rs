//!
//! Device firmware update
//!

pub enum Error {
    WrongTarget,
    FileVerifyFailed,
    WriteFailed,
    EraseFailed,
    EraseCheckFailed,
    ProgramFailed,
    ProgramVerifyFailed,
    InvalidAddress,
    //...
}
