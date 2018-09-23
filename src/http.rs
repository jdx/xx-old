extern crate reqwest;

use std::fs;
use std::io;
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
/// use std::fs::File;
/// use std::io::prelude::*;
///
/// fn main() -> Result<(), xx::http::DownloadError> {
///     xx::http::download("https://jsonplaceholder.typicode.com/todos/1", "tmp/todo.json")?;
///     let contents = xx::fs::read_file("tmp/todo.json").expect("error reading file");
///     assert_eq!(contents, "{\n  \"userId\": 1,\n  \"id\": 1,\n  \"title\": \"delectus aut autem\",\n  \"completed\": false\n}");
///     Ok(())
/// }
/// ```
pub fn download(url: &str, to: &str) -> Result<(), DownloadError> {
    println!("downloading {} to {}", url, to);
    let mut response = reqwest::get(url).map_err(DownloadError::HTTP)?;
    let parent_path = Path::new(to).parent().expect("invalid path");
    fs::create_dir_all(parent_path)
        .map_err(DownloadError::IO)?;
    let mut file = fs::File::create(to).map_err(DownloadError::IO)?;
    io::copy(&mut response, &mut file)
        .map_err(DownloadError::IO)?;
    Ok(())
}
