#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FlowDestinationFlowConfigDestinationConnectorPropertiesCustomerProfiles {
    /// Unique name of the Amazon Connect Customer Profiles domain.
    #[builder(into)]
    #[serde(rename = "domainName")]
    pub r#domain_name: String,
    /// Object specified in the Amazon Connect Customer Profiles flow destination.
    #[builder(into)]
    #[serde(rename = "objectTypeName")]
    pub r#object_type_name: Option<String>,
}
