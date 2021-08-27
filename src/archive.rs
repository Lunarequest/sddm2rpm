use flate2::read::GzDecoder;
use std::fs::{remove_dir_all, File};
use std::path::Path;
use tar::Archive;

pub fn unpack(source: &String, dest: &String) -> Result<(), ()> {
    let source_path = Path::new(source);
    if source_path.exists() && source_path.is_file() {
        let tar_bin = File::open(source).unwrap();
        let tar = GzDecoder::new(tar_bin);
        let mut archive = Archive::new(tar);
        archive.unpack(dest).unwrap();
        return Ok(());
    } else {
        panic!("Path is not valid")
    }
}

pub fn cleanup(path: &String) {
    remove_dir_all(path).unwrap();
}
