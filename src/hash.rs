extern crate digest;
extern crate sha1;
extern crate sha2;

use self::sha1::{Digest, Sha1};
use self::sha2::{Sha256, Sha512};
use std::fs;
use std::io;
use std::convert::AsRef;

/// Compute the hash of file using SHA-1
///
/// # Examples
///
/// ```
/// use std::io;
///
/// fn main() -> io::Result<()> {
///     let digest = xx::hash::file_sha1("fixtures/foo")?;
///     assert_eq!(digest, "988881adc9fc3655077dc2d4d757d480b5ea0e11");
///     Ok(())
/// }
/// ```
pub fn file_sha1<F: AsRef<str>>(f: F) -> Result<String, io::Error> {
    let mut file = fs::File::open(f.as_ref())?;
    let hash = Sha1::digest_reader(&mut file)?;
    // println!("{:?} {:x}", f, hash);
    Ok(format!("{:x}", hash))
}

/// Compute the hash of file using SHA-256
///
/// # Examples
///
/// ```
/// use std::io;
///
/// fn main() -> io::Result<()> {
///     let digest = xx::hash::file_sha256("fixtures/foo")?;
///     assert_eq!(digest, "aec070645fe53ee3b3763059376134f058cc337247c978add178b6ccdfb0019f");
///     Ok(())
/// }
/// ```
pub fn file_sha256<F: AsRef<str>>(f: F) -> Result<String, io::Error> {
    let mut file = fs::File::open(f.as_ref())?;
    let hash = Sha256::digest_reader(&mut file)?;
    // println!("{:?} {:x}", f, hash);
    Ok(format!("{:x}", hash))
}

/// Compute the hash of file using SHA-512
///
/// # Examples
///
/// ```
/// use std::io;
///
/// fn main() -> io::Result<()> {
///     let digest = xx::hash::file_sha512("fixtures/foo")?;
///     assert_eq!(digest, "e79b8ad22b34a54be999f4eadde2ee895c208d4b3d83f1954b61255d2556a8b73773c0dc0210aa044ffcca6834839460959cbc9f73d3079262fc8bc935d46262");
///     Ok(())
/// }
/// ```
pub fn file_sha512<F: AsRef<str>>(f: F) -> Result<String, io::Error> {
    let mut file = fs::File::open(f.as_ref())?;
    let hash = Sha512::digest_reader(&mut file)?;
    // println!("{:?} {:x}", f, hash);
    Ok(format!("{:x}", hash))
}
