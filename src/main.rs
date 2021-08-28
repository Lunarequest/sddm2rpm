use clap::{App, Arg};
mod archive;
mod rpm_build;

fn name_from_file(filename: &String) -> String {
    return filename
        .to_owned()
        .replace(".tar", "")
        .replace(".xz", "")
        .replace(".bz2", "")
        .replace(".gz", "");
}

fn main() {
    let matches = App::new("sddm2rpm")
        .version("0.1.0")
        .about("takes sddm theme as tar.gz files and repacks them to rpms")
        .arg(
            Arg::with_name("version")
                .short("ver")
                .long("version")
                .help("version of package, defaults to 1.0.0")
                .value_name("VERSION")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("license")
                .long("license")
                .help("license of package, defaults to GPL")
                .value_name("LICENSE")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("source")
                .required(true)
                .help("path to sddm archive")
                .index(1),
        )
        .arg(
            Arg::with_name("dest")
                .help("directory to unpack too")
                .index(2),
        )
        .get_matches();
    let source = matches.value_of("source").unwrap().to_owned().to_string();
    let dest = matches
        .value_of("dest")
        .unwrap_or("./unpacked")
        .to_owned()
        .to_string();
    let license = matches
        .value_of("LICENSE")
        .unwrap_or("GPL")
        .to_owned()
        .to_string();
    let version = matches
        .value_of("VERSION")
        .unwrap_or("1.0.0")
        .to_owned()
        .to_string();
    // always clean up after yourself
    match archive::unpack(&source, &dest) {
        Ok(()) => {
            let name = name_from_file(&source);
            rpm_build::buildrpm(&dest, name, version, license);
            archive::cleanup(&dest);
        }
        Err(()) => {
            archive::cleanup(&dest);
        }
    }
}
