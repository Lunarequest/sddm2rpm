use handlebars::Handlebars;
use serde::Serialize;
use std::env;
use std::path::Path;

const DEFAULT_SPEC_TEMPLATE: &str = include_str!("../templates/spec.hbs");
#[derive(Serialize)]
pub struct SpecParams<'a> {
    // Name of the RPM
    pub name: &'a str,
    // Description of the RPM
    pub summary: &'a str,
    // version of package
    pub version: &'a str,
    // License of the *binary* contents of the RPM
    pub license: &'a str,
    // URL to a home page for this package
    pub url: Option<&'a str>,
    // commands to build the rpm
    pub build_commands: &'a str,
    // files in rpm
    pub files: &'a str,
    // source files
    pub source: &'a str,
}

pub fn gen_spec(
    source: &str,
    unpacked: &str,
    name: &str,
    version: &str,
    license: &str,
    url: Option<&str>,
) -> Result<String, ()> {
    let build_commands = gen_build_commands(unpacked, name);
    let files = format!("%{{_datadir}}/sddm/themes/{}", name);
    let summary = format!("Auto genrated specfile for {} sddm theme", name);
    let mut handlebars = Handlebars::new();
    let data = SpecParams {
        name,
        summary: &summary,
        version,
        license,
        url,
        build_commands: &build_commands,
        files: &files,
        source,
    };
    handlebars
        .register_template_string(name, DEFAULT_SPEC_TEMPLATE)
        .unwrap();
    Ok(handlebars
        .render(name, &data)
        .expect("error rendering template"))
}

fn gen_build_commands(source: &str, name: &str) -> String {
    let mut build_commands = String::new();
    let current_dir = env::current_dir().unwrap();
    let wd = Path::new(&source);
    assert!(env::set_current_dir(wd).is_ok());
    build_commands += format!(
        "mkdir -p %{{buildroot}}/%{{_datadir}}/sddm/themes/{}\n",
        name
    )
    .as_str();

    build_commands += format!("cp -r * %{{buildroot}}/%{{_datadir}}/sddm/themes/{}", name).as_str();
    assert!(env::set_current_dir(current_dir).is_ok());
    build_commands
}
