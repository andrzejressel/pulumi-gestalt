#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FlexibleAppVersionDeployment {
    /// Options for the build operations performed as a part of the version deployment. Only applicable when creating a version using source code directly.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "cloudBuildOptions")]
    pub r#cloud_build_options: Option<Box<super::super::types::appengine::FlexibleAppVersionDeploymentCloudBuildOptions>>,
    /// The Docker image for the container that runs the version.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "container")]
    pub r#container: Option<Box<super::super::types::appengine::FlexibleAppVersionDeploymentContainer>>,
    /// Manifest of the files stored in Google Cloud Storage that are included as part of this version.
    /// All files must be readable using the credentials supplied with this call.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "files")]
    pub r#files: Option<Vec<super::super::types::appengine::FlexibleAppVersionDeploymentFile>>,
    /// Zip File
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "zip")]
    pub r#zip: Option<Box<super::super::types::appengine::FlexibleAppVersionDeploymentZip>>,
}
