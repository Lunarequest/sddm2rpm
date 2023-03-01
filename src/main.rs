use clap::{value_parser, Arg, ArgAction, Command};
use std::fs::write;
use std::path::Path;
mod archive;
mod rpm_build;
mod spec_builder;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn name_from_file(filename: &String) -> String {
    return filename
        .replace(".zip", "")
        .replace(".tar", "")
        .replace(".xz", "")
        .replace(".bz2", "")
        .replace(".gz", "");
}

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let matches = Command::new("sddm2rpm")
        .version(VERSION)
        .about("takes sddm theme as tar.gz files and repacks them to rpms")
        .arg(
            Arg::new("pkg-version")
                .long("pkg-version")
                .help("version of package, defaults to 1.0.0")
                .value_name("VERSION")
                .required(false)
                .value_parser(value_parser!(String)),
        )
        .arg(
            Arg::new("license")
                .long("license")
                .help("license of package, defaults to GPLv3")
                .value_name("LICENSE")
                .required(false)
                .value_parser(value_parser!(String)),
        )
        .arg(
            Arg::new("output-spec")
                .long("output-spec")
                .short('s')
                .help("output spec file")
                .value_name("SPEC")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("url")
                .help("upstream url for rpm")
                .long("url")
                .value_name("URL")
                .required(false)
                .value_parser(value_parser!(String)),
        )
        .arg(
            Arg::new("source")
                .required(true)
                .help("path to sddm archive")
                .index(1)
                .value_parser(value_parser!(String)),
        )
        .arg(Arg::new("dest").help("directory to unpack too").index(2))
        .get_matches();
    let url = match matches.get_one::<String>("url") {
        Some(url) => Some(url.to_owned()),
        None => None,
    };

    let source = matches
        .get_one::<String>("source")
        .unwrap()
        .to_owned()
        .to_string();
    let dest = matches
        .get_one::<String>("dest")
        .unwrap_or(&"./unpacked".to_string())
        .to_owned();
    let license = matches
        .get_one::<String>("license")
        .unwrap_or(&"GPLv3".to_string())
        .to_owned();
    let version = matches
        .get_one::<String>("version")
        .unwrap_or(&"1.0.0".to_string())
        .to_owned();
    // always clean up after yourself
    match archive::unpack(&source, &dest) {
        Ok(()) => {
            let name = name_from_file(&source);
            if matches.get_flag("output-spec") {
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
            rpm_build::buildrpm(&dest, name, version, license).await;
            archive::cleanup(&dest);
        }
        Err(()) => {
            archive::cleanup(&dest);
        }
    }
}
