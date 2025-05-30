#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetTriggerBuildArtifactNpmPackage {
    /// Path to the package.json. e.g. workspace/path/to/package
    #[builder(into)]
    #[serde(rename = "packagePath")]
    pub r#package_path: Box<String>,
    /// Artifact Registry repository, in the form "https://$REGION-npm.pkg.dev/$PROJECT/$REPOSITORY"
    /// 
    /// Npm package in the workspace specified by path will be zipped and uploaded to Artifact Registry with this location as a prefix.
    #[builder(into)]
    #[serde(rename = "repository")]
    pub r#repository: Box<String>,
}
