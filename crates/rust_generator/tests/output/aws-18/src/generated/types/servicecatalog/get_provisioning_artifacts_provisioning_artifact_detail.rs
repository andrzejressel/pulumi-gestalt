#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetProvisioningArtifactsProvisioningArtifactDetail {
    /// Indicates whether the product version is active.
    #[builder(into)]
    #[serde(rename = "active")]
    pub r#active: bool,
    /// The UTC time stamp of the creation time.
    #[builder(into)]
    #[serde(rename = "createdTime")]
    pub r#created_time: String,
    /// The description of the provisioning artifact.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: String,
    /// Information set by the administrator to provide guidance to end users about which provisioning artifacts to use.
    #[builder(into)]
    #[serde(rename = "guidance")]
    pub r#guidance: String,
    /// The identifier of the provisioning artifact.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: String,
    /// The name of the provisioning artifact.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The type of provisioning artifact.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}
