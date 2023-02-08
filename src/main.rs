use clap::{Arg, Command};
use std::fs::write;
use std::path::Path;
mod archive;
mod rpm_build;
mod spec_builder;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn name_from_file(filename: &String) -> String {
    return filename
        .to_owned()
        .replace(".tar", "")
        .replace(".xz", "")
        .replace(".bz2", "")
        .replace(".gz", "");
}

fn main() {
    let matches = Command::new("sddm2rpm")
        .version(VERSION)
        .about("takes sddm theme as tar.gz files and repacks them to rpms")
        .arg(
            Arg::new("version")
                .long("version")
                .help("version of package, defaults to 1.0.0")
                .value_name("VERSION")
                .takes_value(true),
        )
        .arg(
            Arg::new("license")
                .long("license")
                .help("license of package, defaults to GPLv3")
                .value_name("LICENSE")
                .takes_value(true),
        )
        .arg(
            Arg::new("output-spec")
                .long("output-spec")
                .short('s')
                .help("output spec file")
                .value_name("SPEC")
                .takes_value(false),
        )
        .arg(
            Arg::new("url")
                .help("upstream url for rpm")
                .long("url")
                .value_name("URL")
                .takes_value(true),
        )
        .arg(
            Arg::new("source")
                .required(true)
                .help("path to sddm archive")
                .index(1),
        )
        .arg(Arg::new("dest").help("directory to unpack too").index(2))
        .get_matches();
    let url;
    match matches.value_of("url") {
        Some(a) => url = Option::Some(a.to_owned().to_string()),
        None => url = None,
    }
    let source = matches.value_of("source").unwrap().to_owned().to_string();
    let dest = matches
        .value_of("dest")
        .unwrap_or("./unpacked")
        .to_owned()
        .to_string();
    let license = matches
        .value_of("license")
        .unwrap_or("GPLv3")
        .to_owned()
        .to_string();
    let version = matches
        .value_of("version")
        .unwrap_or("1.0.0")
        .to_owned()
        .to_string();
    // always clean up after yourself
    match archive::unpack(&source, &dest) {
        Ok(()) => {
            let name = name_from_file(&source);
            if matches.is_present("output-spec") {
                let spec = spec_builder::gen_spec(
                    &source,
                    dest.clone(),
                    name.clone(),
                    version.clone(),
                    license.clone(),
                    url,
                )
                .unwrap();
                let spec_file_name = format!("{}.spec", name);
                let spec_path = Path::new(&spec_file_name);
                write(spec_path, spec).expect("unable to write spec");
                println!("Please update the spec file with release number and change log.")
            }
            rpm_build::buildrpm(&dest, name, version, license);
            archive::cleanup(&dest);
        }
        Err(()) => {
            archive::cleanup(&dest);
        }
    }
}
