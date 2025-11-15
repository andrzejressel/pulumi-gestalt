#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConnectorSubnet {
    /// Subnet name (relative, not fully qualified). E.g. if the full subnet selfLink is
    /// https://compute.googleapis.com/compute/v1/projects/{project}/regions/{region}/subnetworks/{subnetName} the correct input for this field would be {subnetName}"
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Project in which the subnet exists. If not set, this project is assumed to be the project for which the connector create request was issued.
    #[builder(into)]
    #[serde(rename = "projectId")]
    pub r#project_id: Option<String>,
}
