#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetVirtualNodeSpecServiceDiscoveryAwsCloudMap {
    #[builder(into)]
    #[serde(rename = "attributes")]
    pub r#attributes: std::collections::HashMap<String, String>,
    #[builder(into)]
    #[serde(rename = "namespaceName")]
    pub r#namespace_name: String,
    #[builder(into)]
    #[serde(rename = "serviceName")]
    pub r#service_name: String,
}
