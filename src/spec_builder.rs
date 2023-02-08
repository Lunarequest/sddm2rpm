use handlebars::Handlebars;
use serde::Serialize;
use std::env;
use std::path::Path;

const DEFAULT_SPEC_TEMPLATE: &str = include_str!("../templates/spec.hbs");
#[derive(Serialize)]
pub struct SpecParams {
    // Name of the RPM
    pub name: String,
    // Description of the RPM
    pub summary: String,
    // version of package
    pub version: String,
    // License of the *binary* contents of the RPM
    pub license: String,
    // URL to a home page for this package
    pub url: Option<String>,
    // commands to build the rpm
    pub build_commands: String,
    // files in rpm
    pub files: String,
    // source files
    pub source: String,
}

pub fn gen_spec(
    source: &String,
    unpacked: String,
    name: String,
    version: String,
    license: String,
    url: Option<String>,
) -> Result<String, ()> {
    let build_commands = gen_build_commands(&unpacked, &name);
    let files = format!("%{{_datadir}}/sddm/themes/{}", name);
    let summary = format!("Auto genrated specfile for {} sddm theme", name);
    let mut handlebars = Handlebars::new();
    let data = SpecParams {
        name: name.clone(),
        summary: summary,
        version: version,
        license: license,
        url: url,
        build_commands: build_commands,
        files: files,
        source: source.to_owned(),
    };
    handlebars
        .register_template_string(&name.as_str(), DEFAULT_SPEC_TEMPLATE)
        .unwrap();
    return Ok(handlebars
        .render(&name.as_str(), &data)
        .expect("error rendering template"));
}

fn gen_build_commands(source: &String, name: &String) -> String {
    let mut build_commands = String::new();
    let current_dir = env::current_dir().unwrap();
    let wd = Path::new(&source);
    assert!(env::set_current_dir(wd).is_ok());
    build_commands += format!("mkdir -p %{{buildroot}}/%{{_datadir}}/sddm/themes/{}\n", name).as_str();
    
    build_commands +=    format!(
                "cp -r * %{{buildroot}}/%{{_datadir}}/sddm/themes/{}",
                name
            )
            .as_str();
    assert!(env::set_current_dir(current_dir).is_ok());
    build_commands
}
