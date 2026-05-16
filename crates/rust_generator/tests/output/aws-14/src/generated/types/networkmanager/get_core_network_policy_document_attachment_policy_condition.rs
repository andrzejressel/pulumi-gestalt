#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetCoreNetworkPolicyDocumentAttachmentPolicyCondition {
    /// string value
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Option<String>,
    /// Valid values include: `equals`, `not-equals`, `contains`, `begins-with`.
    #[builder(into)]
    #[serde(rename = "operator")]
    pub r#operator: Option<String>,
    /// Valid values include: `account-id`, `any`, `tag-value`, `tag-exists`, `resource-id`, `region`, `attachment-type`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
    /// string value
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}
