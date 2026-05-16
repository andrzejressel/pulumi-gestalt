#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetNetworkManagerScope {
    /// A list of management group IDs used a scope for the Network Manager.
    #[builder(into)]
    #[serde(rename = "managementGroupIds")]
    pub r#management_group_ids: Vec<String>,
    /// A list of subscription IDs used as the scope for the Network Manager.
    #[builder(into)]
    #[serde(rename = "subscriptionIds")]
    pub r#subscription_ids: Vec<String>,
}
