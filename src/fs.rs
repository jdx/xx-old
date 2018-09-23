use std::fs;
use std::io;

use std::io::prelude::*;

/// Read a file to a string
///
/// # Examples
///
/// ```
/// use std::io;
///
/// fn main() -> Result<(), io::Error> {
///     let contents = xx::fs::read_file("fixtures/foo")?;
///     assert_eq!(contents, "foobar\n");
///     Ok(())
/// }
/// ```
pub fn read_file(file: &str) -> Result<String, io::Error> {
    let mut f = fs::File::open(file)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    Ok(contents)
}
