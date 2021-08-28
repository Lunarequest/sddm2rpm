use handlebars::Handlebars;
use serde::Serialize;
use std::fs::read_dir;

const DEFAULT_SPEC_TEMPLATE: &str = include_str!("../templates/spec.hbs");
#[derive(Serialize)]
pub struct SpecParams {
    // Name of the RPM
    pub name: String,

    // Description of the RPM
    pub summary: String,

    // License of the *binary* contents of the RPM
    pub license: String,

    // URL to a home page for this package
    pub url: Option<String>,
    // commands to build the rpm
    pub build_commands: String,
    // files in rpm
    pub files: String,
}

pub fn gen_spec(
    source: &String,
    name: String,
    version: String,
    license: String,
    url: Option<String>,
) -> Result<String, ()> {
    let build_commands = gen_build_commands(source, name);
    let files = format!("%{{_datadir}}/sddm/themes/{}", name);
    let summary = format!("Auto genrated specfile for {} sddm theme", name);
    let mut handlebars = Handlebars::new();
    let data = SpecParams {
        name,
        summary,
        license,
        url,
        build_commands,
        files,
    };
    handlebars
        .register_template_string(&name.as_str(), DEFAULT_SPEC_TEMPLATE)
        .unwrap();
    return Ok(handlebars
        .render(&name.as_str(), &data)
        .expect("error rendering template"));
}

fn gen_build_commands(source: &String, name: String) -> String {
    let mut build_commands = String::new();
    build_commands =
        build_commands + format!("mkdir %{{buildroot}}/usr/share/sddm/themes/{}\n", name);
    let pkg_dir = read_dir(source);
    build_commands = build_commands
        + format!(
            "cp -r {} %{{buildroot}}/usr/share/sddm/themes/{}",
            name, name
        );
    return build_commands;
}
