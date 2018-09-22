extern crate base64;

pub use self::base64::DecodeError;
use std::fmt;
use std::string;

struct Digest(Vec<u8>);

#[derive(Debug)]
pub enum Base64Error {
    Decode(base64::DecodeError),
    Utf8(string::FromUtf8Error),
}

impl fmt::LowerHex for Digest {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        for byte in & self.0 {
            fmt::LowerHex::fmt(byte, f)?;
        }
        Ok(())
    }
}

/// Decode base64 string to hex string
///
/// # Examples
///
/// ```
/// fn main() -> Result<(), xx::base64::DecodeError> {
///     let digest = xx::base64::decode_hex("aGVsbG8gd29ybGQ=")?;
///     assert_eq!(digest, "68656c6c6f20776f726c64");
///     Ok(())
/// }
/// ```
pub fn decode_hex(input: &str) -> Result<String, DecodeError> {
    let output = Digest(base64::decode(input)?);
    Ok(format!("{:x}", output))
}

/// Decode base64 string to string
///
/// # Examples
///
/// ```
/// fn main() -> Result<(), xx::base64::Base64Error> {
///     let digest = xx::base64::decode("aGVsbG8gd29ybGQ=")?;
///     assert_eq!(digest, "hello world");
///     Ok(())
/// }
/// ```
pub fn decode(input: &str) -> Result<String, Base64Error> {
    let bytes = base64::decode(input).map_err(Base64Error::Decode)?;
    let output = String::from_utf8(bytes).map_err(Base64Error::Utf8)?;
    Ok(output)
}
