#[derive(Debug)]
pub enum ProcessError {
    ProcessNotFound,
    IoError{
        inner: std::io::Error
    },
    ConvertionError,

    OsError{
        #[cfg(target_os = "linux")]
        inner: nix::errno::Errno,
        #[cfg(target_os = "windows")]
        inner: windows::core::Error,
    },
}

impl From<std::io::Error> for ProcessError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError{ inner: value }
    }
}

impl From<std::num::ParseIntError> for ProcessError {
    fn from(_: std::num::ParseIntError) -> Self {
        Self::ConvertionError
    }
}

impl From<std::num::TryFromIntError> for ProcessError {
    fn from(_: std::num::TryFromIntError) -> Self {
        Self::ConvertionError
    }
}

// Linux only
#[cfg(target_os = "linux")]
impl From<nix::errno::Errno> for ProcessError {
    fn from(inner: nix::errno::Errno) -> Self {
        Self::OsError{
            inner
        }// TODO add code value
    }
}

// Windows only
#[cfg(target_os = "windows")]
impl From<windows::core::Error> for ProcessError {
    fn from(inner: windows::core::Error) -> Self {
        Self::OsError{
            inner
        }// TODO add code value
    }
}
