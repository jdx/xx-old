extern crate reqwest;

use std::fs;
use std::io;
use std::convert::AsRef;
use std::path::Path;

#[derive(Debug)]
pub enum DownloadError {
    IO(io::Error),
    HTTP(reqwest::Error),
}

/// Download a file
///
/// # Examples
///
/// ```
/// use std::fs;
/// use std::io::prelude::*;
///
/// fn main() {
///     xx::http::download("https://jsonplaceholder.typicode.com/todos/1", "tmp/todo.json")
///       .expect("error downloading");
///     let contents = fs::read_to_string("tmp/todo.json").expect("error reading file");
///     assert_eq!(contents, "{\n  \"userId\": 1,\n  \"id\": 1,\n  \"title\": \"delectus aut autem\",\n  \"completed\": false\n}");
/// }
/// ```
pub fn download<A: AsRef<str>, B: AsRef<Path>>(_url: A, _to: B) -> Result<(), DownloadError> {
    let url = _url.as_ref();
    let to = _to.as_ref();
    println!("downloading {:?} to {:?}", url, to);
    let mut response = reqwest::get(url).map_err(DownloadError::HTTP)?;
    let parent_path = Path::new(to).parent().expect("invalid path");
    fs::create_dir_all(parent_path).map_err(DownloadError::IO)?;
    let mut file = fs::File::create(to).map_err(DownloadError::IO)?;
    io::copy(&mut response, &mut file).map_err(DownloadError::IO)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_download_path() {
        let p = Path::new("tmp/todo2.json");
        download(String::from("https://jsonplaceholder.typicode.com/todos/1"), p).unwrap();
        let contents = fs::read_to_string("tmp/todo2.json").expect("error reading file");
        assert_eq!(contents, "{\n  \"userId\": 1,\n  \"id\": 1,\n  \"title\": \"delectus aut autem\",\n  \"completed\": false\n}");
    }
}
