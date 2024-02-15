use crate::{
    traits::{Converter, Ensure, Filter},
    utils::{buffer_alloc, read_uint32_le_from_file},
};
use std::{fs, path::Path};

pub struct AudioFile<'a> {
    pub path: &'a str,
    pub file_name: &'a str,
    pub suffix: &'a str,
}

impl<'a> AudioFile<'a> {
    pub fn new(full_path: &'a str) -> Self {
        let full_path = Path::new(full_path);
        let path = full_path
            .parent()
            .expect("Error when parsing path!")
            .to_str()
            .unwrap();
        let file_name = full_path
            .file_name()
            .expect("Error when parsing filename!")
            .to_str()
            .unwrap();
        let suffix = full_path
            .extension()
            .expect("Error when parsing suffix!")
            .to_str()
            .unwrap();
        AudioFile {
            path,
            file_name,
            suffix,
        }
    }
}

impl<'a> Ensure for AudioFile<'a> {
    fn ensure_directory_exists(dir: &str) -> () {
        let path = Path::new(dir);
        if !path.exists() {
            match fs::create_dir(&path) {
                Ok(_) => println!("Created directory: {}", dir),
                Err(e) => println!("Error creating directory: {}", e),
            }
        }
    }
}

impl<'a> Filter for AudioFile<'a> {
    fn filter_by_suffix(dir: &str, suffix: &str) {
        // walk through the directory
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Some(entry_suffix) = entry.path().extension().and_then(|s| s.to_str()) {
                        if entry_suffix != suffix {
                            if let Err(e) = fs::remove_file(entry.path()) {
                                eprintln!("Error removing file: {}", e);
                            }
                        }
                    }
                }
            }
        } else {
            eprintln!("Error reading directory {}", dir);
        }
    }
}

impl<'a> Converter for AudioFile<'a> {
    fn ncm2mp3(ncm_dir: &str, mp3_out_dir: &str, songcover_out_dir: &str) -> () {
        Self::ensure_directory_exists(ncm_dir);
        Self::ensure_directory_exists(mp3_out_dir);
        Self::ensure_directory_exists(songcover_out_dir);
        Self::filter_by_suffix(ncm_dir, "ncm");
        if let Ok(entries) = fs::read_dir(ncm_dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path_buf = entry.path();
                    let mut global_offset: u64 = 10;
                    let key_length =
                        read_uint32_le_from_file(path_buf.to_str().unwrap(), global_offset).unwrap();
                    global_offset += 4;
                    let key_data = buffer_alloc(key_length as usize, 0);
                    
                   
                }
            }
        }
    }
}
