use askama::Template;

pub(crate) enum DependencySource {
    CratesIo { version: String },
    Local { path: String },
}

pub(crate) struct Dependency {
    pub name: String,
    pub source: DependencySource,
}

pub(crate) struct CargoToml {
    pub name: String,
    pub version: String,
    pub dependencies: Vec<Dependency>,
}

#[derive(Template)]
#[template(path = "Cargo.toml.j2")]
struct CargoTomlTemplate<'a> {
    name: &'a str,
    version: &'a str,
    dependencies: &'a [Dependency],
}

pub(crate) fn render_cargo_toml(cargo_toml: &CargoToml) -> Result<String, askama::Error> {
    CargoTomlTemplate {
        name: &cargo_toml.name,
        version: &cargo_toml.version,
        dependencies: &cargo_toml.dependencies,
    }
    .render()
    .map(|rendered| rendered.trim().to_string())
}
