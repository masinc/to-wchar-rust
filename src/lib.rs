//! This module contains the following traits.
//! * [`ToWchar`] - Convert wchar(utf-16) array to wchar(utf-16) array from a string.
//! * [`FromWchar`] - Converts wchar(utf-16) array to a string.
//!
//! # Examples
//!
//! ```
//! use to_wchar::prelude::*;
//!
//! let hello_s = "HELLO";
//! assert_eq!(hello_s.to_wchar(), vec![0x0048, 0x0045, 0x004C, 0x004C, 0x004F, 0x0000]);
//!
//! let hello_v: Vec<u16> = vec![0x0048, 0x0045, 0x004C, 0x004C, 0x004F, 0x0000];
//! assert_eq!(String::from_wchar(&hello_v), Ok("HELLO".into()));
//! ```
//!
//! [`ToWchar`]: trait.ToWchar.html
//! [`FromWchar`]: trait.FromWchar.html

pub mod prelude;

use std::ffi::{OsStr, OsString};

/// To wchar(utf-16) trait to a wchar(utf-16) `Vec`.
pub trait ToWchar {
    fn to_wchar(&self) -> Vec<u16>;
}

#[cfg(windows)]
impl ToWchar for str {
    /// Convert a string into a wchar(utf-16) `Vec`.
    #[inline]
    fn to_wchar(&self) -> Vec<u16> {
        use std::iter::once;
        use std::os::windows::ffi::OsStrExt;
        OsStr::new(&self).encode_wide().chain(once(0)).collect()
    }
}

/// From wchar(utf-16) trait to a `String`
pub trait FromWchar {
    /// Convert a wchar(utf-16) to a `String`.
    fn from_wchar(wchar: &[u16]) -> Result<String, OsString>;
}

#[cfg(windows)]
impl FromWchar for String {
    /// Convert a wchar(utf-16) array to a `String`.
    #[inline]
    fn from_wchar(wchar: &[u16]) -> Result<String, OsString> {
        use std::os::windows::ffi::OsStringExt;
        OsString::from_wide(wchar)
            .into_string()
            .map(|x| x.trim_end_matches('\0').into())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_to_wchar() {
        use super::ToWchar;
        assert_eq!(
            "HELLO".to_wchar(),
            vec![0x0048, 0x0045, 0x004C, 0x004C, 0x004F, 0x0000]
        );
    }

    #[test]
    fn test_from_wchar() {
        use super::FromWchar;
        let a: Vec<u16> = vec![0x0048, 0x0045, 0x004C, 0x004C, 0x004F, 0x0000];
        assert_eq!(String::from_wchar(&a).unwrap(), "HELLO")
    }
}
