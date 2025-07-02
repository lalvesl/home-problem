use anyhow::{Context, Result};
use std::fs::File;
use std::io::{BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use zip::{write::SimpleFileOptions, ZipWriter};

pub fn create_zip_from_files(
    zip_path: impl AsRef<Path>,
    files: Vec<PathBuf>,
) -> Result<()> {
    let zip_file = File::create(&zip_path).with_context(|| {
        format!("Failed to create ZIP file at {:?}", zip_path.as_ref())
    })?;
    let mut zip = ZipWriter::new(BufWriter::new(zip_file));

    let base_path =
        common_prefix_path(&files).unwrap_or_else(|| PathBuf::from("/"));

    for path in files {
        if path.is_file() {
            let mut f = File::open(&path)
                .with_context(|| format!("Failed to open file {:?}", path))?;

            let rel_path = path.strip_prefix(&base_path).unwrap_or(&path);
            let rel_path_str = rel_path.to_string_lossy();

            zip.start_file(rel_path_str, SimpleFileOptions::default())?;

            let mut buffer = Vec::new();
            f.read_to_end(&mut buffer)?;
            zip.write_all(&buffer)?;
        }
    }

    zip.finish()?;
    Ok(())
}

fn common_prefix_path(paths: &[PathBuf]) -> Option<PathBuf> {
    if paths.is_empty() {
        return None;
    }

    let mut components = paths[0].components().collect::<Vec<_>>();

    for path in &paths[1..] {
        let mut new_components = vec![];
        for (a, b) in components.iter().zip(path.components()) {
            if a == &b {
                new_components.push(*a);
            } else {
                break;
            }
        }
        components = new_components;
    }

    Some(components.iter().collect())
}
