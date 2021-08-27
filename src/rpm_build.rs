use rpm;
use std::env;
use std::path::Path;
use std::str::FromStr;
use walkdir;

pub fn buildrpm(source: &String, name: String) {
    let current_dir = env::current_dir().unwrap();
    let wd = Path::new(source);
    assert!(env::set_current_dir(wd).is_ok());
    // rpm-rs handles setting up the compressor lets use it
    let mut pkg =
        rpm::RPMBuilder::new(name.as_str(), "1.0.0", "GPL", "noarch", "autogenrated rpms")
            .compression(rpm::Compressor::from_str("gzip").unwrap());
    for entry in walkdir::WalkDir::new(".")
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let file = entry.path();
        if Path::new(&file.as_os_str()).is_file() {
            let dest = format!(
                "{}",
                file.to_string_lossy()
                    .replace("./", "/usr/share/sddm/themes/")
            );
            let options = rpm::RPMFileOptions::new(dest);
            pkg = pkg.with_file(file, options).expect("Error");
        }
    }
    assert!(env::set_current_dir(current_dir).is_ok());
    let package = pkg.build().unwrap();
    let mut f = std::fs::File::create(format!("{}.rpm", name)).unwrap();
    package.write(&mut f).unwrap();
}
