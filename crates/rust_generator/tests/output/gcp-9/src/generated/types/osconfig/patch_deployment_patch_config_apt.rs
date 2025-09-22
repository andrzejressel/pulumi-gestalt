#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PatchDeploymentPatchConfigApt {
    /// List of packages to exclude from update. These packages will be excluded.
    #[builder(into)]
    #[serde(rename = "excludes")]
    pub r#excludes: Option<Vec<String>>,
    /// An exclusive list of packages to be updated. These are the only packages that will be updated.
    /// If these packages are not installed, they will be ignored. This field cannot be specified with
    /// any other patch configuration fields.
    #[builder(into)]
    #[serde(rename = "exclusivePackages")]
    pub r#exclusive_packages: Option<Vec<String>>,
    /// By changing the type to DIST, the patching is performed using apt-get dist-upgrade instead.
    /// Possible values are: `DIST`, `UPGRADE`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Option<String>,
}
