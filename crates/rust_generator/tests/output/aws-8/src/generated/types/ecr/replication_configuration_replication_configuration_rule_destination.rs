#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ReplicationConfigurationReplicationConfigurationRuleDestination {
    /// A Region to replicate to.
    #[builder(into)]
    #[serde(rename = "region")]
    pub r#region: String,
    /// The account ID of the destination registry to replicate to.
    #[builder(into)]
    #[serde(rename = "registryId")]
    pub r#registry_id: String,
}
