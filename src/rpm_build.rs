use rpm::{self, RPMBuilder};
use std::env;
use std::path::Path;
use std::process::exit;
use std::str::FromStr;

async fn buildpkg(name: &str, version: &str, license: &str) -> RPMBuilder {
    let os_release = sys_info::linux_os_release().map_err(|e| {
        eprintln!("{}", e);
    });
    // rpm-rs handles setting up the compressor lets use it
    let pkg = rpm::RPMBuilder::new(
        name,
        version,
        license,
        "noarch",
        "autogenrated sddm theme rpm",
    )
    .compression(rpm::Compressor::from_str("zstd").unwrap())
    .requires(rpm::Dependency::any("sddm"));

    let info = os_release.unwrap().id.unwrap();
    if info.contains("suse") {
        pkg.requires(rpm::Dependency::any("kf5-plasma"))
            .requires(rpm::Dependency::any("libqt5-qtquickcontrols"))
            .requires(rpm::Dependency::any("libqt5-qtvirtualkeyboard"))
    } else if info.contains("Fedora Linux") {
        pkg.requires(rpm::Dependency::any("libKF5Plasma5"))
            .requires(rpm::Dependency::any("qt5-qtquickcontrols"))
            .requires(rpm::Dependency::any("desktop-backgrounds-compat"))
    } else {
        eprintln!("unsupported distro falling back to common package names\nNote: this may not be all dependencies");
        pkg
    }
}

pub async fn buildrpm(source: &str, name: &str, version: &str, license: &str) {
    let current_dir = env::current_dir().unwrap();

    let wd = Path::new(source);
    assert!(env::set_current_dir(wd).is_ok());
    let mut pkg = buildpkg(name, version, license).await;
    for entry in walkdir::WalkDir::new(".")
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let file = entry.path();
        if Path::new(&file.as_os_str()).is_file() {
            let dest = file
                .to_string_lossy()
                .replace("./", "/usr/share/sddm/themes/");
            let options = rpm::RPMFileOptions::new(dest);
            pkg = pkg.with_file_async(file, options).await.expect("Error");
        }
    }
    assert!(env::set_current_dir(current_dir).is_ok());
    let package = pkg.build().unwrap();
    let mut f = std::fs::File::create(format!("{}.rpm", name)).unwrap();
    match package.write(&mut f) {
        Ok(()) => println!("created {name}.rpm"),
        Err(e) => {
            eprintln!("Failed to synathsize rpm {e}");
            exit(1);
        }
    };
}
