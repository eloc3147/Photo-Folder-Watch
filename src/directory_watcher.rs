use std::path::PathBuf;
use std::error::Error;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};


#[derive(Debug)]
pub struct DirectoryGroup {
    pub name: String,
    pub raw_dirs: Vec<PathBuf>,
    pub render_dirs: Vec<PathBuf>,
}

pub type ImageList = Vec<PathBuf>;

#[derive(Debug)]
pub struct Count {
    total: usize,
    edited: usize,
    deleted: usize,
}

pub type Counts = HashMap<String, Count>;


fn scan_directory(dir: &PathBuf, list: &mut ImageList) -> Result<(), Box<Error>> {
    for item in dir.read_dir()? {
        let item = item?.path();
        if item.is_file() {
            list.push(item);
        }
    }

    Ok(())
}

pub fn update_counts(counts: Arc<Mutex<Counts>>, group: DirectoryGroup) -> Result<(), Box<Error>> {
    // Count images
    let mut raw_images = ImageList::new();
    let mut render_images = ImageList::new();
    for dir in group.raw_dirs.iter() {
        scan_directory(&dir, &mut raw_images)?;
    }
    for dir in group.render_dirs.iter() {
        scan_directory(&dir, &mut render_images)?;
    }

    let count = Count {
        total: raw_images.len(),
        deleted: 0,
        edited: render_images.len(),
    };

    // Update counts
    counts.lock().unwrap().insert(group.name, count);

    Ok(())
}