use anyhow::Context;
use askama::Template;
use pulumi_gestalt_schema::model::Package;
use serde::Serialize;

#[derive(Template)]
#[template(path = "main.rs.jinja")]
struct TemplateModel<'a> {
    functions: String,
    resources: String,
    types: String,
    constants: Vec<String>,
    provider_name: &'a str,
    provider_version: &'a str,
    provider_metadata: &'a str,
}

#[derive(Serialize, Debug)]
struct WasmProviderVersion {
    pub version: String,
    #[serde(rename = "pluginDownloadURL")]
    pub plugin_download_url: Option<String>,
}

pub(crate) fn generate(
    functions: String,
    resources: String,
    types: String,
    constants: Vec<String>,
    package: &Package,
) -> anyhow::Result<String> {
    let provider = WasmProviderVersion {
        version: package.version.clone(),
        plugin_download_url: package.plugin_download_url.clone(),
    };
    let provider = serde_json::to_string(&provider)
        .with_context(|| format!("Failed to serialize provider [{:?}]", provider))?;

    let file = TemplateModel {
        functions,
        resources,
        types,
        constants,
        provider_name: &package.name,
        provider_version: &package.version,
        provider_metadata: &provider,
    }
    .render()?;

    let syntax_tree = syn::parse_file(file.as_str())?;
    let formatted = prettyplease::unparse(&syntax_tree);

    Ok(formatted)
}
