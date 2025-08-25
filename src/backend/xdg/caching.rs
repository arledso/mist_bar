use super::generic::get_xdg_dirs;
use std::{fs::{create_dir_all, File}, path::{Path, PathBuf}};
use anyhow::{Context, Result as Result};
use reqwest::blocking::get;
use std::io::Write;

pub fn try_cache_spotify_image(url: &str) -> Result<()> {
    let xdg_dirs = get_xdg_dirs();

    let image_dir = xdg_dirs
        .place_cache_file(&format!("spotify_images/{url}.jpg"))
        .with_context(|| "failed to place the cache file, a spotify image")?;
    
    if let Some(parent) = image_dir.parent() { create_dir_all(parent)?; }
    let mut cache_file = File::create(image_dir)?;

    let image = get(url)?.bytes()?;
    cache_file.write(&image)?;

    println!("downloaded and cached spotify image with url: {url}");
    Ok(())
}

pub fn find_image_from_cache(url: &str) -> Option<PathBuf> {
    let xdg_dirs = get_xdg_dirs();

    let relative_path_string = format!("spotify_images/{url}.jpg");
    let image_path = Path::new(&relative_path_string);
    xdg_dirs.find_cache_file(image_path)
}
