use bzip2::read::BzDecoder;
use flate2::read::GzDecoder;
use std::fs::{remove_dir_all, File};
use std::path::Path;
use tar::Archive;
use xz2::read::XzDecoder;
use zip::ZipArchive;
use zstd::stream::Decoder;

fn unpack_zip(path: &Path, dest: &Path) {
    let zip = File::open(path).unwrap();
    let mut zip_archive = ZipArchive::new(zip).unwrap();
    zip_archive.extract(dest).unwrap();
}

fn unpack_gz(path: &Path, dest: &Path) {
    let tar_gz = File::open(path).unwrap();
    let tar_from_gz = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar_from_gz);
    archive.unpack(dest).unwrap();
}

fn unpack_xz(path: &Path, dest: &Path) {
    let tar_xz = File::open(path).unwrap();
    let tar_from_xz = XzDecoder::new(tar_xz);
    let mut archive = Archive::new(tar_from_xz);
    archive.unpack(dest).unwrap();
}

fn unpack_bz2(path: &Path, dest: &Path) {
    let tar_xz = File::open(path).unwrap();
    let tar_from_xz = BzDecoder::new(tar_xz);
    let mut archive = Archive::new(tar_from_xz);
    archive.unpack(dest).unwrap();
}

fn unpack_zstd(path: &Path, dest: &Path) {
    let tar_xz = File::open(path).unwrap();
    let tar_from_zstd = Decoder::new(tar_xz).expect("is this properly encoded?");
    let mut archive = Archive::new(tar_from_zstd);
    archive.unpack(dest).unwrap();
}

pub fn unpack(source: &String, dest: &String) -> Result<(), ()> {
    let source_path = Path::new(source);
    let dest_path = Path::new(dest);
    if source_path.exists() && source_path.is_file() {
        if source.ends_with(".gz") {
            unpack_gz(source_path, dest_path);
        } else if source.ends_with(".xz") {
            unpack_xz(source_path, dest_path);
        } else if source.ends_with(".bz2") {
            unpack_bz2(source_path, dest_path);
        } else if source.ends_with(".zstd") {
            unpack_zstd(source_path, dest_path);
        } else if source.ends_with("zip") {
            unpack_zip(source_path, dest_path);
        } else {
            panic!("compression not implemented yet")
        }
        return Ok(());
    } else {
        panic!("Path is not valid")
    }
}

pub fn cleanup(path: &String) {
    remove_dir_all(path).unwrap();
}
