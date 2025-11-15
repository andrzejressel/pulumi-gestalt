#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ServiceTemplateVpcAccess {
    /// VPC Access connector name. Format: projects/{project}/locations/{location}/connectors/{connector}, where {project} can be project id or number.
    #[builder(into)]
    #[serde(rename = "connector")]
    pub r#connector: Option<String>,
    /// Traffic VPC egress settings.
    /// Possible values are: `ALL_TRAFFIC`, `PRIVATE_RANGES_ONLY`.
    #[builder(into)]
    #[serde(rename = "egress")]
    pub r#egress: Option<String>,
    /// Direct VPC egress settings. Currently only single network interface is supported.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "networkInterfaces")]
    pub r#network_interfaces: Option<Vec<super::super::types::cloudrunv2::ServiceTemplateVpcAccessNetworkInterface>>,
}
